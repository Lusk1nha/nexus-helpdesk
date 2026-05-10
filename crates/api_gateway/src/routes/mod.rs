use crate::app_state::AppState;
use axum::Router;

pub mod identity;

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .nest("/api/v1/identity", identity::routes())
        .with_state(state)
}
