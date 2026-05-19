mod common;

use axum::http::StatusCode;
use common::spawn_test_app;

// ─── Change user role ─────────────────────────────────────────────────────────

#[tokio::test]
async fn change_user_role_returns_204() {
    let app = spawn_test_app().await;
    app.register_tenant("admin@corp.com", "StrongPass123!").await;
    let admin_token = app.login("admin@corp.com", "StrongPass123!").await;

    let (_, invited) = app
        .post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": "agent@corp.com",
                "full_name": "Agent User",
                "role": "agent",
                "temporary_password": "TempPass123!"
            }),
            &admin_token,
        )
        .await;
    let user_id = invited["user_id"].as_str().unwrap();

    let (status, _) = app
        .patch_json_authed(
            &format!("/api/v1/identity/users/{user_id}/role"),
            serde_json::json!({ "role": "customer" }),
            &admin_token,
        )
        .await;

    assert_eq!(status, StatusCode::NO_CONTENT);
}

#[tokio::test]
async fn change_role_for_unknown_user_returns_404() {
    let app = spawn_test_app().await;
    app.register_tenant("admin2@corp.com", "StrongPass123!").await;
    let token = app.login("admin2@corp.com", "StrongPass123!").await;

    let (status, _) = app
        .patch_json_authed(
            "/api/v1/identity/users/00000000-0000-0000-0000-000000000000/role",
            serde_json::json!({ "role": "agent" }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::NOT_FOUND);
}

// ─── Update user status ───────────────────────────────────────────────────────

#[tokio::test]
async fn deactivate_user_prevents_login() {
    let app = spawn_test_app().await;
    app.register_tenant("admin3@corp.com", "StrongPass123!").await;
    let admin_token = app.login("admin3@corp.com", "StrongPass123!").await;

    let (_, invited) = app
        .post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": "deact@corp.com",
                "full_name": "To Deactivate",
                "role": "agent",
                "temporary_password": "TempPass123!"
            }),
            &admin_token,
        )
        .await;
    let user_id = invited["user_id"].as_str().unwrap();

    // Confirm the user can log in before deactivation
    let (pre_status, _) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({ "email": "deact@corp.com", "password": "TempPass123!" }),
        )
        .await;
    assert_eq!(pre_status, StatusCode::OK);

    // Deactivate
    let (status, _) = app
        .patch_json_authed(
            &format!("/api/v1/identity/users/{user_id}/status"),
            serde_json::json!({ "active": false }),
            &admin_token,
        )
        .await;
    assert_eq!(status, StatusCode::NO_CONTENT);

    // Login must now be rejected
    let (post_status, _) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({ "email": "deact@corp.com", "password": "TempPass123!" }),
        )
        .await;
    assert_eq!(post_status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn reactivate_user_restores_login() {
    let app = spawn_test_app().await;
    app.register_tenant("admin4@corp.com", "StrongPass123!").await;
    let admin_token = app.login("admin4@corp.com", "StrongPass123!").await;

    let (_, invited) = app
        .post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": "react@corp.com",
                "full_name": "To Reactivate",
                "role": "agent",
                "temporary_password": "TempPass123!"
            }),
            &admin_token,
        )
        .await;
    let user_id = invited["user_id"].as_str().unwrap();

    // Deactivate then reactivate
    app.patch_json_authed(
        &format!("/api/v1/identity/users/{user_id}/status"),
        serde_json::json!({ "active": false }),
        &admin_token,
    )
    .await;

    app.patch_json_authed(
        &format!("/api/v1/identity/users/{user_id}/status"),
        serde_json::json!({ "active": true }),
        &admin_token,
    )
    .await;

    let (status, _) = app
        .post_json(
            "/api/v1/identity/login",
            serde_json::json!({ "email": "react@corp.com", "password": "TempPass123!" }),
        )
        .await;
    assert_eq!(status, StatusCode::OK);
}

// ─── Get tenant ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_tenant_returns_tenant_info() {
    let app = spawn_test_app().await;

    let (_, reg) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenant_name": "Acme Corp",
                "admin_full_name": "Admin",
                "admin_email": "getme@acme.com",
                "admin_password": "StrongPass123!"
            }),
        )
        .await;
    let token = app.login("getme@acme.com", "StrongPass123!").await;

    let (status, body) = app.get_json("/api/v1/identity/tenant", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["name"].as_str().unwrap(), "Acme Corp");
    assert!(body["id"].is_string());
    assert_eq!(body["plan"].as_str().unwrap(), "free");
    assert!(body["is_active"].as_bool().unwrap());
    drop(reg);
}
