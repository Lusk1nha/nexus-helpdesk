use domain_identity::application::use_cases::register_tenant::{
    RegisterTenantCommand, RegisterTenantUseCase,
};
use domain_identity::domain::error::DomainError;
use domain_identity::infrastructure::database::PgUnitOfWorkManager;
use domain_identity::infrastructure::security::Argon2Hasher;

use domain_identity::domain::ports::UnitOfWorkManager;

use std::sync::Arc;

use pretty_assertions::assert_eq;

mod common;
use common::setup_isolated_db;

// ==========================================
// TESTES DE INTEGRAÇÃO
// ==========================================

#[tokio::test]
async fn test_register_tenant_success_saves_all_data() {
    // Trazemos o `_container` para segurar o tempo de vida dele até o final da função
    let (pool, _container) = setup_isolated_db().await;

    // 1. Setup
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    let use_case = RegisterTenantUseCase::new(uow_manager.clone(), password_hasher);

    let command = RegisterTenantCommand {
        tenant_name: "Nexus Corp LTDA".to_string(),
        tenant_slug: "nexus-corp-ltda".to_string(),
        admin_full_name: "Jane Doe".to_string(),
        admin_email: "admin@empresa.com".to_string(),
        admin_plain_password: "SenhaSuperForte123!".to_string(),
    };

    // 2. Ação
    let result = use_case.execute(command).await;

    // 3. Asserts
    assert!(result.is_ok(), "O caso de uso deveria ter retornado Ok");
    let (tenant, user) = result.unwrap();

    assert_eq!(tenant.name, "Nexus Corp LTDA");
    assert_eq!(tenant.slug, "nexus-corp-ltda");
    assert_eq!(user.email, "admin@empresa.com");

    // 4. Verifica banco
    let mut uow = uow_manager.begin().await.unwrap();
    let saved_user = uow
        .users()
        .find_by_email("admin@empresa.com")
        .await
        .unwrap();

    assert!(saved_user.is_some());
}

#[tokio::test]
async fn test_register_tenant_fails_if_email_already_exists() {
    let (pool, _container) = setup_isolated_db().await;

    // 1. Setup
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    let use_case = RegisterTenantUseCase::new(uow_manager, password_hasher);

    let command_1 = RegisterTenantCommand {
        tenant_name: "Empresa Um".to_string(),
        tenant_slug: "empresa-um".to_string(),
        admin_full_name: "Bob".to_string(),
        admin_email: "shared@empresa.com".to_string(),
        admin_plain_password: "123".to_string(),
    };

    let command_2 = RegisterTenantCommand {
        tenant_name: "Empresa Dois".to_string(),
        tenant_slug: "empresa-dois".to_string(),
        admin_full_name: "Alice".to_string(),
        admin_email: "shared@empresa.com".to_string(), // Mesmo e-mail
        admin_plain_password: "456".to_string(),
    };

    // 2. Ação
    let _ = use_case.execute(command_1).await.unwrap();
    let second_attempt = use_case.execute(command_2).await;

    // 3. Asserts
    assert!(second_attempt.is_err(), "A segunda criação deve falhar");
    assert!(matches!(
        second_attempt.unwrap_err(),
        DomainError::UserAlreadyExists
    ));
}
