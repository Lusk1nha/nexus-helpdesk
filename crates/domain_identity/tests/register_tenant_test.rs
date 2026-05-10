use domain_identity::application::use_cases::register_tenant::{
    RegisterTenantCommand, RegisterTenantUseCase,
};
use domain_identity::domain::error::DomainError;
use domain_identity::domain::ports::UserRepository;
use domain_identity::infrastructure::database::{PgUnitOfWorkManager, PgUserRepository};
use domain_identity::infrastructure::security::Argon2Hasher;

use sqlx::{PgPool, postgres::PgPoolOptions};
use std::sync::Arc;
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

use pretty_assertions::assert_eq;

// ==========================================
// SETUP DO TESTCONTAINERS
// ==========================================
async fn setup_isolated_db() -> (PgPool, testcontainers::ContainerAsync<Postgres>) {
    // 1. Inicia o container do PostgreSQL em background
    let container = Postgres::default()
        .start()
        .await
        .expect("Falha ao iniciar container do Postgres");

    // 2. Pega o IP e a porta aleatória que o Docker mapeou para o host
    let host_ip = container
        .get_host()
        .await
        .expect("Falha ao pegar IP do host");
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Falha ao pegar porta");

    // A imagem padrão do `testcontainers-modules::postgres` usa user/pass/db como "postgres"
    let connection_string = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        host_ip, host_port
    );

    // 3. Cria o Pool do SQLx apontando para o container efêmero
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("Falha ao conectar no banco efêmero");

    // 4. Roda as migrations nesse banco novo
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Falha ao rodar migrations no testcontainer");

    // RETORNO IMPORTANTE: Retornamos o container junto com o pool.
    // Se o container for "dropado" (sair de escopo), o Docker encerra ele na mesma hora!
    (pool, container)
}

// ==========================================
// TESTES DE INTEGRAÇÃO
// ==========================================

#[tokio::test]
async fn test_register_tenant_success_saves_all_data() {
    // Trazemos o `_container` para segurar o tempo de vida dele até o final da função
    let (pool, _container) = setup_isolated_db().await;

    // 1. Setup
    let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    let use_case = RegisterTenantUseCase::new(user_repo.clone(), uow_manager, password_hasher);

    let command = RegisterTenantCommand {
        tenant_name: "Nexus Corp LTDA".to_string(),
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
    let saved_user = user_repo.find_by_email("admin@empresa.com").await.unwrap();
    assert!(saved_user.is_some());
}

#[tokio::test]
async fn test_register_tenant_fails_if_email_already_exists() {
    let (pool, _container) = setup_isolated_db().await;

    // 1. Setup
    let user_repo = Arc::new(PgUserRepository::new(pool.clone()));
    let uow_manager = Arc::new(PgUnitOfWorkManager::new(pool.clone()));
    let password_hasher = Arc::new(Argon2Hasher::new());

    let use_case = RegisterTenantUseCase::new(user_repo.clone(), uow_manager, password_hasher);

    let command_1 = RegisterTenantCommand {
        tenant_name: "Empresa Um".to_string(),
        admin_full_name: "Bob".to_string(),
        admin_email: "shared@empresa.com".to_string(),
        admin_plain_password: "123".to_string(),
    };

    let command_2 = RegisterTenantCommand {
        tenant_name: "Empresa Dois".to_string(),
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
