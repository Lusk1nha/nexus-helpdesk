use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use uuid::Uuid;

/// Inicializa um banco de dados isolado no Docker para testes de integração de Ticketing
pub async fn setup_isolated_db() -> (PgPool, testcontainers::ContainerAsync<Postgres>) {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("domain_ticketing=debug") // Filtra logs do seu domínio
        .with_test_writer() // Garante que o output case com o coletor do cargo test
        .try_init();

    let container = Postgres::default()
        .start()
        .await
        .expect("Falha ao iniciar container do Postgres");

    let host_ip = container
        .get_host()
        .await
        .expect("Falha ao pegar IP do host");

    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("Falha ao pegar porta");

    let connection_string = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        host_ip, host_port
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("Falha ao conectar no banco efêmero");

    // Executa as migrations a partir da raiz do workspace (3 níveis acima de tests/common/)
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Falha ao rodar migrations no testcontainer");

    (pool, container)
}

/// Cria um Tenant válido diretamente no banco para satisfazer FKs nos testes
pub async fn seed_test_tenant(pool: &PgPool, id: Uuid) {
    sqlx::query(
        "INSERT INTO tenants (id, name, slug, plan, is_active, created_at, updated_at) \
         VALUES ($1, 'Tenant de Teste', 'tenant-teste', 'free', true, NOW(), NOW())",
    )
    .bind(id)
    .execute(pool)
    .await
    .expect("Falha ao rodar seed de Tenant");
}

/// Cria um Usuário válido diretamente no banco para satisfazer FKs nos testes
pub async fn seed_test_user(pool: &PgPool, id: Uuid) {
    sqlx::query(
        "INSERT INTO users (id, email, full_name, is_active, created_at, updated_at) \
         VALUES ($1, 'cliente@teste.com', 'Cliente do Teste', true, NOW(), NOW())",
    )
    .bind(id)
    .execute(pool)
    .await
    .expect("Falha ao rodar seed de User");
}
