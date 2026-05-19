mod common;

use axum::http::StatusCode;
use common::spawn_test_app;

// ─── Register ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn register_tenant_returns_201_with_ids() {
    let app = spawn_test_app().await;

    let (status, body) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenant_name": "Acme Corp",
                "admin_full_name": "John Doe",
                "admin_email": "john@acme.com",
                "admin_password": "StrongPass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["tenant_id"].is_string());
    assert!(body["user_id"].is_string());
}

#[tokio::test]
async fn register_duplicate_email_returns_409() {
    let app = spawn_test_app().await;
    app.register_tenant("dup@example.com", "StrongPass123!").await;

    let (status, body) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenant_name": "Another Corp",
                "admin_full_name": "Jane Doe",
                "admin_email": "dup@example.com",
                "admin_password": "AnotherPass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert!(body["message"].as_str().unwrap().contains("e-mail"));
}

#[tokio::test]
async fn register_with_short_password_returns_400() {
    let app = spawn_test_app().await;

    let (status, _body) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenant_name": "Corp X",
                "admin_full_name": "User X",
                "admin_email": "user@corpx.com",
                "admin_password": "short"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn register_with_invalid_email_returns_400() {
    let app = spawn_test_app().await;

    let (status, _body) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenant_name": "Corp Y",
                "admin_full_name": "User Y",
                "admin_email": "not-an-email",
                "admin_password": "StrongPass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// ─── Login ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn login_with_valid_credentials_returns_200_with_token() {
    let app = spawn_test_app().await;
    app.register_tenant("login@example.com", "StrongPass123!").await;

    let (status, body) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({
                "email": "login@example.com",
                "password": "StrongPass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["token"].is_string());
    assert!(!body["token"].as_str().unwrap().is_empty());
    assert!(body["user_id"].is_string());
    assert!(body["tenant_id"].is_string());
    assert_eq!(body["role"].as_str().unwrap(), "admin");
}

#[tokio::test]
async fn login_with_wrong_password_returns_401() {
    let app = spawn_test_app().await;
    app.register_tenant("wrongpass@example.com", "CorrectPass123!").await;

    let (status, body) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({
                "email": "wrongpass@example.com",
                "password": "WrongPassword!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
    assert!(body["message"].is_string());
}

#[tokio::test]
async fn login_with_unknown_email_returns_401() {
    let app = spawn_test_app().await;

    let (status, _body) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({
                "email": "ghost@nowhere.com",
                "password": "SomePass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ─── Get Me ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_me_with_valid_token_returns_200() {
    let app = spawn_test_app().await;
    app.register_tenant("me@example.com", "StrongPass123!").await;
    let token = app.login("me@example.com", "StrongPass123!").await;

    let (status, body) = app.get_json("/api/v1/identity/me", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["user_id"].is_string());
    assert!(body["tenant_id"].is_string());
    assert_eq!(body["role"].as_str().unwrap(), "admin");
}

#[tokio::test]
async fn get_me_without_token_returns_401() {
    let app = spawn_test_app().await;

    let (status, _body) = app.get_json("/api/v1/identity/me", None).await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_me_with_invalid_token_returns_401() {
    let app = spawn_test_app().await;

    let (status, _body) = app
        .get_json("/api/v1/identity/me", Some("this.is.not.a.valid.jwt"))
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
