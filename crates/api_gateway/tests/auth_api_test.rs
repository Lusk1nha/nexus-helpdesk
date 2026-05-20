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
                "tenantName": "Acme Corp",
                "adminFullName": "John Doe",
                "adminEmail": "john@acme.com",
                "adminPassword": "StrongPass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["data"]["tenantId"].is_string());
    assert!(body["data"]["userId"].is_string());
    assert!(body["meta"]["timestamp"].is_string());
}

#[tokio::test]
async fn register_duplicate_email_returns_409() {
    let app = spawn_test_app().await;
    app.register_tenant("dup@example.com", "StrongPass123!")
        .await;

    let (status, body) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenantName": "Another Corp",
                "adminFullName": "Jane Doe",
                "adminEmail": "dup@example.com",
                "adminPassword": "AnotherPass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::CONFLICT);
    assert!(
        body["error"]["message"]
            .as_str()
            .unwrap()
            .contains("e-mail")
    );
    assert_eq!(body["error"]["code"].as_str().unwrap(), "CONFLICT");
}

#[tokio::test]
async fn register_with_short_password_returns_400() {
    let app = spawn_test_app().await;

    let (status, _) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenantName": "Corp X",
                "adminFullName": "User X",
                "adminEmail": "user@corpx.com",
                "adminPassword": "short"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn register_with_invalid_email_returns_400() {
    let app = spawn_test_app().await;

    let (status, _) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenantName": "Corp Y",
                "adminFullName": "User Y",
                "adminEmail": "not-an-email",
                "adminPassword": "StrongPass123!"
            }),
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// ─── Login ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn login_with_valid_credentials_returns_200_with_token() {
    let app = spawn_test_app().await;
    app.register_tenant("login@example.com", "StrongPass123!")
        .await;

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
    assert!(body["data"]["token"].is_string());
    assert!(!body["data"]["token"].as_str().unwrap().is_empty());
    assert!(body["data"]["userId"].is_string());
    assert!(body["data"]["tenantId"].is_string());
    assert_eq!(body["data"]["role"].as_str().unwrap(), "admin");
}

#[tokio::test]
async fn login_with_wrong_password_returns_401() {
    let app = spawn_test_app().await;
    app.register_tenant("wrongpass@example.com", "CorrectPass123!")
        .await;

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
    assert!(body["error"]["message"].is_string());
}

#[tokio::test]
async fn login_with_unknown_email_returns_401() {
    let app = spawn_test_app().await;

    let (status, _) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({ "email": "ghost@nowhere.com", "password": "SomePass123!" }),
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ─── Get Me ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_me_with_valid_token_returns_200() {
    let app = spawn_test_app().await;
    app.register_tenant("me@example.com", "StrongPass123!")
        .await;
    let token = app.login("me@example.com", "StrongPass123!").await;

    let (status, body) = app.get_json("/api/v1/identity/me", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    assert!(body["data"]["userId"].is_string());
    assert!(body["data"]["tenantId"].is_string());
    assert_eq!(body["data"]["role"].as_str().unwrap(), "admin");
}

#[tokio::test]
async fn get_me_without_token_returns_401() {
    let app = spawn_test_app().await;
    let (status, _) = app.get_json("/api/v1/identity/me", None).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn get_me_with_invalid_token_returns_401() {
    let app = spawn_test_app().await;
    let (status, _) = app
        .get_json("/api/v1/identity/me", Some("this.is.not.valid"))
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
