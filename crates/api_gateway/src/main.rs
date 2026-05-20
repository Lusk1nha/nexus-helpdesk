use axum::http::{HeaderValue, Method};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use domain_ticketing::application::workers::ai_worker::{AiTask, AiWorker};
use domain_ticketing::infrastructure::database::postgres_uow::PgTicketingUoWManager;

use api_gateway::app_state::AppState;
use api_gateway::config::AppConfig;
use api_gateway::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Load .env into the process environment before anything reads it
    dotenvy::dotenv().ok();

    // 2. Set up structured logging
    setup_telemetry();

    // 3. Load and validate all configuration (panics early with a clear message
    //    if a required variable is missing)
    let config = AppConfig::load();

    tracing::info!(
        host = %config.host,
        port = config.port,
        frontend_url = %config.frontend_url,
        ollama_url = %config.ollama_url,
        qdrant_url = %config.qdrant_url,
        "configuration loaded"
    );

    // 4. Connect to the database
    let db_pool = setup_database(&config.database_url).await?;

    // 5. Spin up background workers (AI queue)
    let ai_task_sender = spawn_background_workers(db_pool.clone(), config.ollama_url.clone());

    // 6. Build application state and router
    let state = AppState::new(db_pool, config.clone(), ai_task_sender);
    let app = routes::create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(setup_cors(&config.frontend_url));

    // 7. Bind and serve
    let addr: SocketAddr = format!("{}:{}", config.host, config.port).parse()?;
    tracing::info!("🚀 Nexus API Gateway escutando em http://{}", addr);
    tracing::info!("📚 Swagger UI disponível em http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_telemetry() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "api_gateway=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
}

async fn setup_database(database_url: &str) -> Result<PgPool, sqlx::Error> {
    tracing::info!("Conectando ao banco de dados...");
    PgPoolOptions::new()
        .max_connections(20)
        .connect(database_url)
        .await
}

fn setup_cors(frontend_url: &str) -> CorsLayer {
    let origin = frontend_url
        .parse::<HeaderValue>()
        .expect("FRONTEND_URL inválida");

    CorsLayer::new()
        .allow_origin(origin)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
        ])
        .allow_headers(tower_http::cors::Any)
}

fn spawn_background_workers(
    db_pool: PgPool,
    ollama_url: String,
) -> tokio::sync::mpsc::Sender<AiTask> {
    let (sender, receiver) = tokio::sync::mpsc::channel::<AiTask>(100);
    let uow_manager = Arc::new(PgTicketingUoWManager::new(db_pool));

    let worker = AiWorker::new(receiver, uow_manager, ollama_url);
    tokio::spawn(async move { worker.start().await });

    sender
}
