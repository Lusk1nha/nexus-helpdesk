use axum::http::{HeaderValue, Method};
use sqlx::{PgPool, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use domain_ticketing::application::workers::ai_worker::{AiTask, AiWorker};

use api_gateway::app_state::AppState;
use api_gateway::config::AppConfig;
use api_gateway::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Inicializa o ecossistema (Logs e Env)
    setup_telemetry();
    let config = AppConfig::from_env();

    // 2. Conecta ao Banco de Dados
    let db_pool = setup_database(&config.database_url).await?;

    // 3. Inicializa os Workers em Background e pega o canal de comunicação
    let ai_task_sender = spawn_background_workers();

    // 4. Configura o Estado da Aplicação
    let state = AppState::new(db_pool, config.clone(), ai_task_sender);

    // 5. Constrói o Roteador (incluindo o Swagger!)
    let app = routes::create_router(state)
        .layer(TraceLayer::new_for_http())
        .layer(setup_cors(&config.frontend_url));

    // 6. Inicia o Servidor
    let addr = SocketAddr::from((config.host, config.port));
    tracing::info!("🚀 Nexus API Gateway escutando em http://{}", addr);
    tracing::info!("📚 Swagger UI disponível em http://{}/swagger-ui", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

fn setup_telemetry() {
    dotenvy::dotenv().ok();
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
    let cors_origin = frontend_url
        .parse::<HeaderValue>()
        .expect("FRONTEND_URL configurada de forma inválida");

    CorsLayer::new()
        .allow_origin(cors_origin)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(tower_http::cors::Any)
}

fn spawn_background_workers() -> tokio::sync::mpsc::Sender<AiTask> {
    let (ai_task_sender, ai_task_receiver) = tokio::sync::mpsc::channel::<AiTask>(100);

    let ai_worker = AiWorker::new(ai_task_receiver);
    tokio::spawn(async move {
        ai_worker.start().await;
    });

    ai_task_sender
}
