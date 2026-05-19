mod common;

use axum::http::StatusCode;
use common::spawn_test_app;

// ─── Create Ticket ────────────────────────────────────────────────────────────

#[tokio::test]
async fn create_ticket_authenticated_returns_201() {
    let app = spawn_test_app().await;
    app.register_tenant("customer@support.com", "StrongPass123!").await;
    let token = app.login("customer@support.com", "StrongPass123!").await;

    let (status, body) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({
                "title": "Cannot access my account",
                "description": "I have been trying to log in for the past hour and keep getting an error."
            }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["ticket_id"].is_string());
    assert!(body["status"].is_string());
}

#[tokio::test]
async fn create_ticket_without_auth_returns_401() {
    let app = spawn_test_app().await;

    let (status, _body) = app
        .post_json(
            "/api/v1/tickets",
            serde_json::json!({
                "title": "Some issue",
                "description": "This request has no auth token."
            }),
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_ticket_with_invalid_token_returns_401() {
    let app = spawn_test_app().await;

    let (status, _body) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({
                "title": "Fake ticket",
                "description": "Token is forged."
            }),
            "Bearer totally.invalid.jwt",
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
