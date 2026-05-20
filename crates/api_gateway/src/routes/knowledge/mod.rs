use axum::{Router, routing::post};

use crate::app_state::AppState;

pub mod contracts;
pub mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(handlers::ingest_knowledge_handler))
}
