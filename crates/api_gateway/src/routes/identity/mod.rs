use crate::app_state::AppState;
use axum::{
    Router,
    routing::{delete, get, patch, post},
};

pub mod contracts;
pub mod handlers;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/me", get(handlers::get_me_handler))
        .route("/register", post(handlers::register_tenant_handler))
        .route("/login", post(handlers::login_handler))
        .route("/refresh", post(handlers::refresh_token_handler))
        .route("/logout", post(handlers::logout_handler))
        .route(
            "/admin/users/{id}/unlock-and-reset",
            post(handlers::admin_reset_user_password_handler),
        )
        .route(
            "/users",
            post(handlers::invite_user_handler).get(handlers::list_users_handler),
        )
        .route(
            "/users/{id}/role",
            patch(handlers::change_user_role_handler),
        )
        .route(
            "/users/{id}/status",
            patch(handlers::update_user_status_handler),
        )
        .route("/tenant", get(handlers::get_tenant_handler))
        .route(
            "/api-keys",
            post(handlers::create_api_key_handler).get(handlers::list_api_keys_handler),
        )
        .route("/api-keys/{id}", delete(handlers::revoke_api_key_handler))
}
