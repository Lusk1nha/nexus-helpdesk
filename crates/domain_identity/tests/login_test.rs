use domain_identity::application::use_cases::{
    login::{LoginCommand, LoginUseCase},
    register_tenant::{RegisterTenantCommand, RegisterTenantUseCase},
};
use domain_identity::domain::error::DomainError;
use domain_identity::infrastructure::database::PgUnitOfWorkManager;
use domain_identity::infrastructure::security::Argon2Hasher;

use domain_identity::domain::ports::UnitOfWorkManager;

use std::sync::Arc;

mod common;
use common::setup_isolated_db;

#[tokio::test]
async fn test_login_success_with_correct_credentials() {
    let (pool, _container) = setup_isolated_db().await;
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    // 1. Cria um Tenant/User de teste usando o use case existente
    let register_uc = RegisterTenantUseCase::new(uow_manager.clone(), password_hasher.clone());
    register_uc
        .execute(RegisterTenantCommand {
            tenant_name: "Login Corporation".to_string(),
            tenant_slug: "login-corp".to_string(),
            admin_full_name: "User Test".to_string(),
            admin_email: "login@test.com".to_string(),
            admin_plain_password: "Password123!".to_string(),
        })
        .await
        .unwrap();

    // 2. Executa a ação de Login
    let login_uc = LoginUseCase::new(uow_manager, password_hasher);
    let result = login_uc
        .execute(LoginCommand {
            email: "login@test.com".to_string(),
            plain_password: "Password123!".to_string(),
        })
        .await;

    // 3. Asserts
    assert!(result.is_ok());
    let (user, tenant_user) = result.unwrap();
    assert_eq!(user.email, "login@test.com");
    assert_eq!(tenant_user.role.to_string(), "admin");
}

#[tokio::test]
async fn test_login_fails_and_increments_attempts_until_lockout() {
    let (pool, _container) = setup_isolated_db().await;
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    let register_uc = RegisterTenantUseCase::new(uow_manager.clone(), password_hasher.clone());
    register_uc
        .execute(RegisterTenantCommand {
            tenant_name: "Security Labs".to_string(),
            tenant_slug: "security-labs".to_string(),
            admin_full_name: "Target User".to_string(),
            admin_email: "hacker@target.com".to_string(),
            admin_plain_password: "CorrectPassword123!".to_string(),
        })
        .await
        .unwrap();

    let login_uc = LoginUseCase::new(uow_manager.clone(), password_hasher);

    // 1. Chuta a senha errada 5 vezes seguidas para estourar o limite (MAX_FAILED_ATTEMPTS = 5)
    for _ in 0..5 {
        let err_result = login_uc
            .execute(LoginCommand {
                email: "hacker@target.com".to_string(),
                plain_password: "WrongPassword!".to_string(),
            })
            .await;

        assert!(matches!(
            err_result.unwrap_err(),
            DomainError::InvalidCredentials
        ));
    }

    // 2. Na 6ª tentativa, mesmo enviando a senha CORRETA, deve falhar porque a conta bloqueou!
    let locked_result = login_uc
        .execute(LoginCommand {
            email: "hacker@target.com".to_string(),
            plain_password: "CorrectPassword123!".to_string(), // Senha certa
        })
        .await;

    assert!(locked_result.is_err());
    assert!(matches!(
        locked_result.unwrap_err(),
        DomainError::InvalidCredentials
    ));

    // 3. Inspeciona o banco diretamente para validar se as 5 tentativas falhas foram salvas de forma persistente
    let mut uow = uow_manager.begin().await.unwrap();
    let user = uow
        .users()
        .find_by_email("hacker@target.com")
        .await
        .unwrap()
        .unwrap();
    let credential = uow
        .credentials()
        .find_by_user_id(user.id)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(credential.failed_attempts, 5);
    assert!(credential.is_locked());
}
