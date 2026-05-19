mod common;

use axum::http::StatusCode;
use common::spawn_test_app;

// ─── Create ───────────────────────────────────────────────────────────────────

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
    assert_eq!(body["status"].as_str().unwrap(), "open");
}

#[tokio::test]
async fn create_ticket_without_auth_returns_401() {
    let app = spawn_test_app().await;

    let (status, _) = app
        .post_json(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Test", "description": "No auth." }),
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn create_ticket_with_invalid_token_returns_401() {
    let app = spawn_test_app().await;

    let (status, _) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Fake", "description": "Token is forged." }),
            "totally.invalid.jwt",
        )
        .await;

    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

// ─── List ─────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_tickets_returns_only_tenant_tickets() {
    let app = spawn_test_app().await;

    let (s, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenant_name": "Tenant Alpha", "admin_full_name": "Admin A",
        "admin_email": "a@tenant.com", "admin_password": "StrongPass123!"
    })).await;
    assert_eq!(s, axum::http::StatusCode::CREATED);
    let token_a = app.login("a@tenant.com", "StrongPass123!").await;

    // Create 2 tickets for tenant A
    for i in 1..=2u8 {
        app.post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": format!("Ticket {i}"), "description": "desc" }),
            &token_a,
        )
        .await;
    }

    // Create a separate tenant — its tickets must not appear in A's list
    let (s2, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenant_name": "Tenant Beta", "admin_full_name": "Admin B",
        "admin_email": "b@other.com", "admin_password": "StrongPass123!"
    })).await;
    assert_eq!(s2, axum::http::StatusCode::CREATED);
    let token_b = app.login("b@other.com", "StrongPass123!").await;
    app.post_json_authed(
        "/api/v1/tickets",
        serde_json::json!({ "title": "Tenant B ticket", "description": "desc" }),
        &token_b,
    )
    .await;

    let (status, body) = app.get_json("/api/v1/tickets", Some(&token_a)).await;

    assert_eq!(status, StatusCode::OK);
    let tickets = body.as_array().unwrap();
    assert_eq!(tickets.len(), 2, "tenant A should only see its own tickets");
}

#[tokio::test]
async fn list_tickets_filters_by_status() {
    let app = spawn_test_app().await;
    app.register_tenant("filter@test.com", "StrongPass123!").await;
    let token = app.login("filter@test.com", "StrongPass123!").await;

    let (_, b1) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Open ticket", "description": "will stay open" }),
            &token,
        )
        .await;
    let ticket_id = b1["ticket_id"].as_str().unwrap();

    // Close the first ticket
    app.patch_json_authed(
        &format!("/api/v1/tickets/{ticket_id}/status"),
        serde_json::json!({ "status": "closed" }),
        &token,
    )
    .await;

    // Create a second ticket that stays open
    app.post_json_authed(
        "/api/v1/tickets",
        serde_json::json!({ "title": "Open ticket 2", "description": "stays open" }),
        &token,
    )
    .await;

    let (status, body) = app
        .get_json("/api/v1/tickets?status=open", Some(&token))
        .await;

    assert_eq!(status, StatusCode::OK);
    let tickets = body.as_array().unwrap();
    assert_eq!(tickets.len(), 1, "only the open ticket should be returned");
    assert_eq!(tickets[0]["status"].as_str().unwrap(), "open");
}

// ─── Get ──────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_ticket_returns_200() {
    let app = spawn_test_app().await;
    app.register_tenant("getter@test.com", "StrongPass123!").await;
    let token = app.login("getter@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "My ticket", "description": "details here" }),
            &token,
        )
        .await;
    let ticket_id = created["ticket_id"].as_str().unwrap();

    let (status, body) = app
        .get_json(&format!("/api/v1/tickets/{ticket_id}"), Some(&token))
        .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["id"].as_str().unwrap(), ticket_id);
    assert_eq!(body["title"].as_str().unwrap(), "My ticket");
}

#[tokio::test]
async fn get_ticket_from_other_tenant_returns_403() {
    let app = spawn_test_app().await;

    let (s1, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenant_name": "Owner Corp", "admin_full_name": "Owner",
        "admin_email": "owner@test.com", "admin_password": "StrongPass123!"
    })).await;
    assert_eq!(s1, axum::http::StatusCode::CREATED);
    let owner_token = app.login("owner@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Owner ticket", "description": "private" }),
            &owner_token,
        )
        .await;
    let ticket_id = created["ticket_id"].as_str().unwrap();

    let (s2, _) = app.post_json("/api/v1/identity/register", serde_json::json!({
        "tenant_name": "Intruder Corp", "admin_full_name": "Intruder",
        "admin_email": "intruder@other.com", "admin_password": "StrongPass123!"
    })).await;
    assert_eq!(s2, axum::http::StatusCode::CREATED);
    let intruder_token = app.login("intruder@other.com", "StrongPass123!").await;

    let (status, _) = app
        .get_json(&format!("/api/v1/tickets/{ticket_id}"), Some(&intruder_token))
        .await;

    assert_eq!(status, StatusCode::FORBIDDEN);
}

// ─── Update status ────────────────────────────────────────────────────────────

#[tokio::test]
async fn close_ticket_returns_200_with_closed_status() {
    let app = spawn_test_app().await;
    app.register_tenant("closer@test.com", "StrongPass123!").await;
    let token = app.login("closer@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "To close", "description": "closing it" }),
            &token,
        )
        .await;
    let ticket_id = created["ticket_id"].as_str().unwrap();

    let (status, body) = app
        .patch_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/status"),
            serde_json::json!({ "status": "closed" }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"].as_str().unwrap(), "closed");
}

#[tokio::test]
async fn invalid_status_transition_returns_400() {
    let app = spawn_test_app().await;
    app.register_tenant("transition@test.com", "StrongPass123!").await;
    let token = app.login("transition@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Ticket", "description": "test transition" }),
            &token,
        )
        .await;
    let ticket_id = created["ticket_id"].as_str().unwrap();

    // Open → Resolved is not a valid transition
    let (status, _) = app
        .patch_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/status"),
            serde_json::json!({ "status": "resolved" }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}

// ─── Messages ─────────────────────────────────────────────────────────────────

#[tokio::test]
async fn list_messages_returns_initial_customer_message() {
    let app = spawn_test_app().await;
    app.register_tenant("msgs@test.com", "StrongPass123!").await;
    let token = app.login("msgs@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Issue", "description": "My device won't start." }),
            &token,
        )
        .await;
    let ticket_id = created["ticket_id"].as_str().unwrap();

    let (status, body) = app
        .get_json(&format!("/api/v1/tickets/{ticket_id}/messages"), Some(&token))
        .await;

    assert_eq!(status, StatusCode::OK);
    let messages = body.as_array().unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0]["sender_type"].as_str().unwrap(), "customer");
    assert_eq!(messages[0]["content"].as_str().unwrap(), "My device won't start.");
}

#[tokio::test]
async fn add_message_returns_201() {
    let app = spawn_test_app().await;
    app.register_tenant("reply@test.com", "StrongPass123!").await;
    let token = app.login("reply@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Follow-up", "description": "Initial problem." }),
            &token,
        )
        .await;
    let ticket_id = created["ticket_id"].as_str().unwrap();

    let (status, body) = app
        .post_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            serde_json::json!({ "content": "I tried restarting and it worked!" }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["content"].as_str().unwrap(), "I tried restarting and it worked!");
}

#[tokio::test]
async fn cannot_add_message_to_closed_ticket() {
    let app = spawn_test_app().await;
    app.register_tenant("closed@test.com", "StrongPass123!").await;
    let token = app.login("closed@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Will close", "description": "Closing soon." }),
            &token,
        )
        .await;
    let ticket_id = created["ticket_id"].as_str().unwrap();

    app.patch_json_authed(
        &format!("/api/v1/tickets/{ticket_id}/status"),
        serde_json::json!({ "status": "closed" }),
        &token,
    )
    .await;

    let (status, _) = app
        .post_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            serde_json::json!({ "content": "Trying to reply to a closed ticket." }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::BAD_REQUEST);
}
