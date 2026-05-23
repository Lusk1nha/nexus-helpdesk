mod common;

use axum::http::StatusCode;
use common::spawn_test_app;
use tower::ServiceExt;

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

// ─── Refresh token ───────────────────────────────────────────────────────────

#[tokio::test]
async fn login_returns_access_and_refresh_tokens() {
    let app = spawn_test_app().await;
    app.register_tenant("pair@example.com", "StrongPass123!")
        .await;

    let (status, body) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({"email": "pair@example.com", "password": "StrongPass123!"}),
        )
        .await;

    assert_eq!(status, StatusCode::OK);
    let access = body["data"]["accessToken"].as_str().unwrap();
    let refresh = body["data"]["refreshToken"].as_str().unwrap();
    assert!(!access.is_empty());
    assert!(!refresh.is_empty());
    assert_ne!(access, refresh);
    assert!(body["data"]["accessTokenExpiresIn"].as_i64().unwrap() > 0);
}

#[tokio::test]
async fn refresh_with_valid_token_returns_new_pair() {
    let app = spawn_test_app().await;
    app.register_tenant("refresh@example.com", "StrongPass123!")
        .await;
    let (_, login_body) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({"email": "refresh@example.com", "password": "StrongPass123!"}),
        )
        .await;

    let refresh = login_body["data"]["refreshToken"].as_str().unwrap();

    let (status, body) = app
        .post_json(
            "/api/v1/identity/refresh",
            serde_json::json!({"refreshToken": refresh}),
        )
        .await;

    assert_eq!(status, StatusCode::OK, "refresh failed: {body}");
    let new_access = body["data"]["accessToken"].as_str().unwrap();
    let new_refresh = body["data"]["refreshToken"].as_str().unwrap();
    assert!(!new_access.is_empty());
    assert_ne!(new_refresh, refresh, "refresh token must rotate");
}

#[tokio::test]
async fn refresh_with_reused_token_is_rejected_after_rotation() {
    let app = spawn_test_app().await;
    app.register_tenant("reuse@example.com", "StrongPass123!")
        .await;
    let (_, login_body) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({"email": "reuse@example.com", "password": "StrongPass123!"}),
        )
        .await;
    let refresh = login_body["data"]["refreshToken"].as_str().unwrap();

    // First refresh succeeds…
    let (status, _) = app
        .post_json(
            "/api/v1/identity/refresh",
            serde_json::json!({"refreshToken": refresh}),
        )
        .await;
    assert_eq!(status, StatusCode::OK);

    // …reusing the old token must be rejected.
    let (status, _) = app
        .post_json(
            "/api/v1/identity/refresh",
            serde_json::json!({"refreshToken": refresh}),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn refresh_with_garbage_token_returns_401() {
    let app = spawn_test_app().await;
    let (status, _) = app
        .post_json(
            "/api/v1/identity/refresh",
            serde_json::json!({"refreshToken": "not.a.jwt"}),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ─── Logout ──────────────────────────────────────────────────────────────────

#[tokio::test]
async fn logout_revokes_refresh_token() {
    let app = spawn_test_app().await;
    app.register_tenant("lo@example.com", "StrongPass123!")
        .await;
    let (_, login_body) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({"email": "lo@example.com", "password": "StrongPass123!"}),
        )
        .await;
    let access = login_body["data"]["accessToken"]
        .as_str()
        .unwrap()
        .to_string();
    let refresh = login_body["data"]["refreshToken"]
        .as_str()
        .unwrap()
        .to_string();

    let (status, _) = app
        .post_json_authed(
            "/api/v1/identity/logout",
            serde_json::json!({"refreshToken": refresh}),
            &access,
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    // After logout the refresh token must no longer work.
    let (status, _) = app
        .post_json(
            "/api/v1/identity/refresh",
            serde_json::json!({"refreshToken": refresh}),
        )
        .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ─── API keys ────────────────────────────────────────────────────────────────

#[tokio::test]
async fn admin_can_create_api_key_and_use_it_for_auth() {
    let app = spawn_test_app().await;
    app.register_tenant("apikey@example.com", "StrongPass123!")
        .await;
    let token = app.login("apikey@example.com", "StrongPass123!").await;

    let (status, body) = app
        .post_json_authed(
            "/api/v1/identity/api-keys",
            serde_json::json!({"name": "ci bot", "role": "agent"}),
            &token,
        )
        .await;
    assert_eq!(status, StatusCode::CREATED, "create failed: {body}");

    let plaintext = body["data"]["plaintext"].as_str().unwrap().to_string();
    assert!(plaintext.starts_with("nxk_"));
    let prefix = body["data"]["keyPrefix"].as_str().unwrap();
    assert!(plaintext.contains(prefix));

    // Use the API key to call /me.
    let response = app
        .router
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/api/v1/identity/me")
                .header("x-api-key", &plaintext)
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn revoked_api_key_no_longer_authenticates() {
    let app = spawn_test_app().await;
    app.register_tenant("revoke@example.com", "StrongPass123!")
        .await;
    let token = app.login("revoke@example.com", "StrongPass123!").await;

    let (status, body) = app
        .post_json_authed(
            "/api/v1/identity/api-keys",
            serde_json::json!({"name": "temp bot", "role": "agent"}),
            &token,
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);
    let plaintext = body["data"]["plaintext"].as_str().unwrap().to_string();
    let id = body["data"]["id"].as_str().unwrap().to_string();

    // Revoke it.
    let response = app
        .router
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("DELETE")
                .uri(format!("/api/v1/identity/api-keys/{id}"))
                .header("authorization", format!("Bearer {token}"))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    // Subsequent calls with that key must fail.
    let response = app
        .router
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method("GET")
                .uri("/api/v1/identity/me")
                .header("x-api-key", plaintext)
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn login_endpoint_is_rate_limited() {
    let app = spawn_test_app().await;
    app.register_tenant("rate@example.com", "StrongPass123!")
        .await;

    // The rate limiter is configured at 6s/req with burst 10. Fire 25 wrong-
    // password requests back-to-back from the same fake peer IP and assert
    // we observe at least one 429 — meaning the limiter actually engaged.
    let mut saw_429 = false;
    for _ in 0..25 {
        let response = app
            .router
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .method("POST")
                    .uri("/api/v1/identity/login")
                    .header("content-type", "application/json")
                    .header("x-forwarded-for", "203.0.113.42")
                    .body(axum::body::Body::from(
                        serde_json::json!({
                            "email": "rate@example.com",
                            "password": "WrongPassword!"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        if response.status() == StatusCode::TOO_MANY_REQUESTS {
            saw_429 = true;
            break;
        }
    }
    assert!(saw_429, "rate limiter never returned 429");
}

#[tokio::test]
async fn non_admin_cannot_create_api_key() {
    let app = spawn_test_app().await;
    app.register_tenant("admin@example.com", "StrongPass123!")
        .await;
    let admin_token = app.login("admin@example.com", "StrongPass123!").await;

    // Invite a non-admin user under the same tenant.
    let (status, _) = app
        .post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": "agent@example.com",
                "fullName": "Agent Smith",
                "role": "agent",
                "temporaryPassword": "AgentPass123!"
            }),
            &admin_token,
        )
        .await;
    assert_eq!(status, StatusCode::CREATED);

    let agent_token = app.login("agent@example.com", "AgentPass123!").await;

    let (status, _) = app
        .post_json_authed(
            "/api/v1/identity/api-keys",
            serde_json::json!({"name": "should fail", "role": "agent"}),
            &agent_token,
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}
