use domain_identity::application::use_cases::{
    login::{LoginCommand, LoginUseCase},
    register_tenant::{RegisterTenantCommand, RegisterTenantUseCase},
    reset_password::{ResetPasswordCommand, ResetPasswordUseCase},
};
use domain_identity::domain::error::DomainError;
use domain_identity::infrastructure::database::PgUnitOfWorkManager;
use domain_identity::infrastructure::security::Argon2Hasher;

use std::sync::Arc;

mod common;
use common::setup_isolated_db;

#[tokio::test]
async fn test_admin_can_reset_password_and_unlock_user_from_same_tenant() {
    let (pool, _container) = setup_isolated_db().await;
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    // 1. Cria a empresa
    let register_uc = RegisterTenantUseCase::new(uow_manager.clone(), password_hasher.clone());
    let (tenant, admin_user) = register_uc
        .execute(RegisterTenantCommand {
            tenant_name: "Alpha Company".to_string(),
            admin_full_name: "Admin Alpha".to_string(),
            admin_email: "admin@alpha.com".to_string(),
            admin_plain_password: "OldPassword123!".to_string(),
        })
        .await
        .unwrap();

    let login_uc = LoginUseCase::new(uow_manager.clone(), password_hasher.clone());

    // 2. Bloqueia o admin errando a senha de propósito 5 vezes
    for _ in 0..5 {
        let _ = login_uc
            .execute(LoginCommand {
                email: "admin@alpha.com".to_string(),
                plain_password: "WrongPassword!".to_string(),
            })
            .await;
    }

    // 3. Admin aciona o override para limpar o lockout e injetar uma nova senha temporária
    let reset_uc = ResetPasswordUseCase::new(uow_manager.clone(), password_hasher.clone());
    let result = reset_uc
        .execute(ResetPasswordCommand {
            target_user_id: admin_user.id,
            operator_tenant_id: tenant.id, // Mesmo tenant!
            is_admin_override: true,
            new_plain_password: Some("TemporaryPassword999!".to_string()),
        })
        .await;

    assert!(result.is_ok());

    // 4. Verifica se a conta foi completamente reativada/desbloqueada e aceita a nova senha no login
    let login_again = login_uc
        .execute(LoginCommand {
            email: "admin@alpha.com".to_string(),
            plain_password: "TemporaryPassword999!".to_string(),
        })
        .await;

    assert!(
        login_again.is_ok(),
        "O login com a senha resetada deveria funcionar e a conta estar desbloqueada."
    );
}

#[tokio::test]
async fn test_reset_password_fails_if_operator_belongs_to_different_tenant_cross_tenant_attack() {
    let (pool, _container) = setup_isolated_db().await;
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    let register_uc = RegisterTenantUseCase::new(uow_manager.clone(), password_hasher.clone());

    // 📦 Cria Empresa A
    let (tenant_a, _admin_a) = register_uc
        .execute(RegisterTenantCommand {
            tenant_name: "Tenant A".to_string(),
            admin_full_name: "Owner A".to_string(),
            admin_email: "owner@a.com".to_string(),
            admin_plain_password: "PasswordA123!".to_string(),
        })
        .await
        .unwrap();

    // 📦 Cria Empresa B (O alvo do ataque)
    let (_tenant_b, admin_b) = register_uc
        .execute(RegisterTenantCommand {
            tenant_name: "Tenant B".to_string(),
            admin_full_name: "Owner B".to_string(),
            admin_email: "victim@b.com".to_string(),
            admin_plain_password: "PasswordB123!".to_string(),
        })
        .await
        .unwrap();

    let reset_uc = ResetPasswordUseCase::new(uow_manager.clone(), password_hasher);

    // 🚨 Ação: Admin da Empresa A tenta resetar a senha do usuário da Empresa B
    let hacker_attack = reset_uc
        .execute(ResetPasswordCommand {
            target_user_id: admin_b.id,      // Id da Vítima
            operator_tenant_id: tenant_a.id, // Tenant do Atacante (Invasão de contexto!)
            is_admin_override: true,
            new_plain_password: Some("HackedPassword123!".to_string()),
        })
        .await;

    // 🛡️ Assert: O domínio deve barrar alegando credenciais inválidas para mascarar IDs
    assert!(hacker_attack.is_err());
    assert!(matches!(
        hacker_attack.unwrap_err(),
        DomainError::InvalidCredentials
    ));
}
