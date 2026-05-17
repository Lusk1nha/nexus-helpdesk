use crate::app_state::AppState;
use axum::{
    Router,
    routing::{get, post},
};

pub mod contracts;
pub mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(handlers::get_me_handler))
        .route("/register", post(handlers::register_tenant_handler))
        .route("/login", post(handlers::login_handler))
        .route(
            "/admin/users/{id}/unlock-and-reset",
            post(handlers::admin_reset_user_password_handler),
        )
}
