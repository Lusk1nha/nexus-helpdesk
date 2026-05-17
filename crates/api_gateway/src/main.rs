// crates/api_gateway/src/main.rs

use axum::http::{HeaderValue, Method};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
mod config;
mod error;
mod middleware;
mod routes;
mod utils;

use app_state::AppState;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inicializa Variáveis de Ambiente e Logs
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 2. Centraliza todas as Configurações
    let app_config = config::AppConfig::from_env();

    // 3. Configura Pool do PostgreSQL usando a Config
    tracing::info!("Conectando ao banco de dados...");
    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&app_config.database_url)
        .await?;

    // 4. Configura CORS Dinâmico com base na variável de ambiente
    let cors_origin = app_config
        .frontend_url
        .parse::<HeaderValue>()
        .expect("FRONTEND_URL configurada de forma inválida");

    let cors = CorsLayer::new()
        .allow_origin(cors_origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    // 5. Cria o Estado e o Aplicativo Axum
    let state = AppState::new(db_pool, app_config.clone());

    let app = routes::create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // 6. Inicia o Servidor usando o Host e Port da Configuração
    let addr = SocketAddr::from((app_config.host, app_config.port));
    tracing::info!("🚀 Nexus API Gateway escutando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
