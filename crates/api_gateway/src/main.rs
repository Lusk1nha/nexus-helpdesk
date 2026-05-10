use axum::http::{HeaderValue, Method};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app_state;
mod config;
mod error;
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

    // 2. Configura Pool do PostgreSQL
    let db_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL deve estar configurada no .env");
    tracing::info!("Conectando ao banco de dados...");

    let app_config = config::AppConfig::from_env();
    let db_pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;

    let state = AppState::new(db_pool, app_config);

    // 3. Configura CORS (Restritivo para B2B/SaaS)
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap()) // Vite default
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any);

    // 4. Cria o Aplicativo Axum
    let app = routes::create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(cors);

    // 5. Inicia o Servidor
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("🚀 Nexus API Gateway escutando em {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
