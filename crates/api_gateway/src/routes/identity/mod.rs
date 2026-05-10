use crate::app_state::AppState;
use axum::{Router, routing::post};

pub mod contracts;
pub mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new().route("/register", post(handlers::register_tenant_handler))
}
