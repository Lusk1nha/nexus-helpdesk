use axum::{
    Router,
    routing::{delete, get},
};

use crate::app_state::AppState;

pub mod contracts;
pub mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(handlers::list_knowledge_handler).post(handlers::ingest_knowledge_handler),
        )
        .route("/search", get(handlers::search_knowledge_handler))
        .route("/{id}", delete(handlers::delete_knowledge_handler))
}
