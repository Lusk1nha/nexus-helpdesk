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
                "full_name": "New Agent",
                "role": "agent",
                "temporary_password": "TempPass123!"
            }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::CREATED, "invite failed: {body}");
    assert!(body["user_id"].is_string());
}

#[tokio::test]
async fn invite_user_duplicate_email_returns_409() {
    let app = spawn_test_app().await;
    app.register_tenant("admin2@corp.com", "StrongPass123!").await;
    let token = app.login("admin2@corp.com", "StrongPass123!").await;

    let payload = serde_json::json!({
        "email": "dup@corp.com",
        "full_name": "Dup User",
        "role": "agent",
        "temporary_password": "TempPass123!"
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

    // Register tenant A (admin)
    app.register_tenant("admin3@corp.com", "StrongPass123!").await;
    let admin_token = app.login("admin3@corp.com", "StrongPass123!").await;

    // Invite a customer user
    app.post_json_authed(
        "/api/v1/identity/users",
        serde_json::json!({
            "email": "cust@corp.com",
            "full_name": "Customer",
            "role": "customer",
            "temporary_password": "TempPass123!"
        }),
        &admin_token,
    )
    .await;

    // Customer logs in and tries to invite someone — must be rejected
    let customer_token = app.login("cust@corp.com", "TempPass123!").await;

    let (status, _) = app
        .post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": "another@corp.com",
                "full_name": "Another",
                "role": "agent",
                "temporary_password": "TempPass123!"
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

    // Invite two more users
    for i in 1..=2u8 {
        app.post_json_authed(
            "/api/v1/identity/users",
            serde_json::json!({
                "email": format!("member{i}@corp.com"),
                "full_name": format!("Member {i}"),
                "role": "agent",
                "temporary_password": "TempPass123!"
            }),
            &token,
        )
        .await;
    }

    let (status, body) = app.get_json("/api/v1/identity/users", Some(&token)).await;

    assert_eq!(status, StatusCode::OK);
    let members = body.as_array().unwrap();
    // 1 original admin + 2 invited
    assert_eq!(members.len(), 3);
}

#[tokio::test]
async fn list_users_does_not_include_other_tenant_members() {
    let app = spawn_test_app().await;

    // Use distinct tenant names to avoid slug collision within the same test DB.
    let (s1, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenant_name": "Alpha Corp", "admin_full_name": "Admin A",
        "admin_email": "t1@corp.com", "admin_password": "StrongPass123!"
    })).await;
    assert_eq!(s1, axum::http::StatusCode::CREATED);
    let token1 = app.login("t1@corp.com", "StrongPass123!").await;

    let (s2, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenant_name": "Beta Corp", "admin_full_name": "Admin B",
        "admin_email": "t2@other.com", "admin_password": "StrongPass123!"
    })).await;
    assert_eq!(s2, axum::http::StatusCode::CREATED);
    let token2 = app.login("t2@other.com", "StrongPass123!").await;

    // Invite someone to tenant 2
    app.post_json_authed(
        "/api/v1/identity/users",
        serde_json::json!({
            "email": "extra@other.com",
            "full_name": "Extra",
            "role": "agent",
            "temporary_password": "TempPass123!"
        }),
        &token2,
    )
    .await;

    let (status, body) = app.get_json("/api/v1/identity/users", Some(&token1)).await;

    assert_eq!(status, StatusCode::OK);
    let members = body.as_array().unwrap();
    assert_eq!(members.len(), 1, "tenant 1 should only see its own admin");
}
