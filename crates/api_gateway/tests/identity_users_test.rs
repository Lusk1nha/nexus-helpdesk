mod common;

use axum::http::StatusCode;
use common::spawn_test_app;

// ─── Invite user ──────────────────────────────────────────────────────────────

#[tokio::test]
async fn invite_user_as_admin_returns_201() {
    let app = spawn_test_app().await;
    app.register_tenant("admin@corp.com", "StrongPass123!").await;
    let token = app.login("admin@corp.com", "StrongPass123!").await;

    let (status, body) = app
        .post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": "agent@corp.com",
                "fullName": "New Agent",
                "role": "agent",
                "temporaryPassword": "TempPass123!"
            }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::CREATED, "invite failed: {body}");
    assert!(body["data"]["userId"].is_string());
}

#[tokio::test]
async fn invite_user_duplicate_email_returns_409() {
    let app = spawn_test_app().await;
    app.register_tenant("admin2@corp.com", "StrongPass123!").await;
    let token = app.login("admin2@corp.com", "StrongPass123!").await;

    let payload = serde_json::json!({
        "email": "dup@corp.com",
        "fullName": "Dup User",
        "role": "agent",
        "temporaryPassword": "TempPass123!"
    });

    app.post_json_authed("/api/v1/identity/users", payload.clone(), &token).await;

    let (status, _) = app
        .post_json_authed("/api/v1/identity/users", payload, &token)
        .await;

    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn invite_user_without_admin_role_returns_403() {
    let app = spawn_test_app().await;
    app.register_tenant("admin3@corp.com", "StrongPass123!").await;
    let admin_token = app.login("admin3@corp.com", "StrongPass123!").await;

    app.post_json_authed(
        "/api/v1/identity/users",
        serde_json::json!({
            "email": "cust@corp.com",
            "fullName": "Customer",
            "role": "customer",
            "temporaryPassword": "TempPass123!"
        }),
        &admin_token,
    ).await;

    let customer_token = app.login("cust@corp.com", "TempPass123!").await;

    let (status, _) = app
        .post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": "another@corp.com",
                "fullName": "Another",
                "role": "agent",
                "temporaryPassword": "TempPass123!"
            }),
            &customer_token,
        )
        .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

// ─── List users ───────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_users_as_admin_returns_all_tenant_members() {
    let app = spawn_test_app().await;
    app.register_tenant("lister@corp.com", "StrongPass123!").await;
    let token = app.login("lister@corp.com", "StrongPass123!").await;

    for i in 1..=2u8 {
        app.post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": format!("member{i}@corp.com"),
                "fullName": format!("Member {i}"),
                "role": "agent",
                "temporaryPassword": "TempPass123!"
            }),
            &token,
        ).await;
    }

    let (status, body) = app.get_json("/api/v1/identity/users", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    let members = body["data"].as_array().unwrap();
    assert_eq!(members.len(), 3);
}

#[tokio::test]
async fn list_users_does_not_include_other_tenant_members() {
    let app = spawn_test_app().await;

    let (s1, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenantName": "Alpha Corp", "adminFullName": "Admin A",
        "adminEmail": "t1@corp.com", "adminPassword": "StrongPass123!"
    })).await;
    assert_eq!(s1, StatusCode::CREATED);
    let token1 = app.login("t1@corp.com", "StrongPass123!").await;

    let (s2, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenantName": "Beta Corp", "adminFullName": "Admin B",
        "adminEmail": "t2@other.com", "adminPassword": "StrongPass123!"
    })).await;
    assert_eq!(s2, StatusCode::CREATED);
    let token2 = app.login("t2@other.com", "StrongPass123!").await;

    app.post_json_authed(
        "/api/v1/identity/users",
        serde_json::json!({
            "email": "extra@other.com",
            "fullName": "Extra",
            "role": "agent",
            "temporaryPassword": "TempPass123!"
        }),
        &token2,
    ).await;

    let (status, body) = app.get_json("/api/v1/identity/users", Some(&token1)).await;

    assert_eq!(status, StatusCode::OK);
    let members = body["data"].as_array().unwrap();
    assert_eq!(members.len(), 1);
}
