use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;

/// Configura um banco de dados PostgreSQL isolado via Testcontainers para os testes de integração.
pub async fn setup_isolated_db() -> (PgPool, testcontainers::ContainerAsync<Postgres>) {
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

    // Como este arquivo está dentro de tests/common/, o caminho relativo para a raiz
    // das migrations muda para três níveis acima: ../../../migrations
    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("Falha ao rodar migrations no testcontainer");

    (pool, container)
}
