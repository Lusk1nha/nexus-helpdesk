use crate::app_state::AppState;
use axum::{
    Router,
    routing::{get, patch, post},
};

pub mod contracts;
pub mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/", post(handlers::create_ticket_handler))
        .route("/", get(handlers::list_tickets_handler))
        .route("/{id}", get(handlers::get_ticket_handler))
        .route("/{id}/status", patch(handlers::update_ticket_status_handler))
        .route("/{id}/messages", get(handlers::list_ticket_messages_handler))
        .route("/{id}/messages", post(handlers::add_message_handler))
        .route("/{id}/approve-ai", post(handlers::approve_ai_response_handler))
        .route("/{id}/reject-ai", post(handlers::reject_ai_response_handler))
}
