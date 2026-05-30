mod common;

use axum::http::StatusCode;
use common::spawn_test_app;

// ─── Create ───────────────────────────────────────────────────────────────────

#[tokio::test]
async fn create_ticket_authenticated_returns_201() {
    let app = spawn_test_app().await;
    app.register_tenant("customer@support.com", "StrongPass123!")
        .await;
    let token = app.login("customer@support.com", "StrongPass123!").await;

    let (status, body) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({
                "title": "Cannot access my account",
                "description": "Trying to log in for an hour."
            }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::CREATED);
    assert!(body["data"]["ticketId"].is_string());
    assert_eq!(body["data"]["status"].as_str().unwrap(), "open");
    assert!(body["meta"]["timestamp"].is_string());
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

    let (s, _) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenantName": "Tenant Alpha", "tenantSlug": "tenant-alpha",
                "adminFullName": "Admin A",
                "adminEmail": "a@tenant.com", "adminPassword": "StrongPass123!"
            }),
        )
        .await;
    assert_eq!(s, StatusCode::CREATED);
    let token_a = app.login("a@tenant.com", "StrongPass123!").await;

    for i in 1..=2u8 {
        app.post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": format!("Ticket {i}"), "description": "desc" }),
            &token_a,
        )
        .await;
    }

    let (s2, _) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenantName": "Tenant Beta", "tenantSlug": "tenant-beta",
                "adminFullName": "Admin B",
                "adminEmail": "b@other.com", "adminPassword": "StrongPass123!"
            }),
        )
        .await;
    assert_eq!(s2, StatusCode::CREATED);
    let token_b = app.login("b@other.com", "StrongPass123!").await;
    app.post_json_authed(
        "/api/v1/tickets",
        serde_json::json!({ "title": "Tenant B ticket", "description": "desc" }),
        &token_b,
    )
    .await;

    let (status, body) = app.get_json("/api/v1/tickets", Some(&token_a)).await;

    assert_eq!(status, StatusCode::OK);
    let tickets = body["data"].as_array().unwrap();
    assert_eq!(tickets.len(), 2);
}

#[tokio::test]
async fn list_tickets_filters_by_status() {
    let app = spawn_test_app().await;
    app.register_tenant("filter@test.com", "StrongPass123!")
        .await;
    let token = app.login("filter@test.com", "StrongPass123!").await;

    let (_, b1) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Open ticket", "description": "will stay open" }),
            &token,
        )
        .await;
    let ticket_id = b1["data"]["ticketId"].as_str().unwrap();

    app.patch_json_authed(
        &format!("/api/v1/tickets/{ticket_id}/status"),
        serde_json::json!({ "status": "closed" }),
        &token,
    )
    .await;

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
    let tickets = body["data"].as_array().unwrap();
    assert_eq!(tickets.len(), 1);
    assert_eq!(tickets[0]["status"].as_str().unwrap(), "open");
}

// ─── Get ──────────────────────────────────────────────────────────────────────

#[tokio::test]
async fn get_ticket_returns_200() {
    let app = spawn_test_app().await;
    app.register_tenant("getter@test.com", "StrongPass123!")
        .await;
    let token = app.login("getter@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "My ticket", "description": "details here" }),
            &token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

    let (status, body) = app
        .get_json(&format!("/api/v1/tickets/{ticket_id}"), Some(&token))
        .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["id"].as_str().unwrap(), ticket_id);
    assert_eq!(body["data"]["title"].as_str().unwrap(), "My ticket");
}

#[tokio::test]
async fn get_ticket_from_other_tenant_returns_403() {
    let app = spawn_test_app().await;

    let (s1, _) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenantName": "Owner Corp", "tenantSlug": "owner-corp",
                "adminFullName": "Owner",
                "adminEmail": "owner@test.com", "adminPassword": "StrongPass123!"
            }),
        )
        .await;
    assert_eq!(s1, StatusCode::CREATED);
    let owner_token = app.login("owner@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Owner ticket", "description": "private" }),
            &owner_token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

    let (s2, _) = app
        .post_json(
            "/api/v1/identity/register",
            serde_json::json!({
                "tenantName": "Intruder Corp", "tenantSlug": "intruder-corp",
                "adminFullName": "Intruder",
                "adminEmail": "intruder@other.com", "adminPassword": "StrongPass123!"
            }),
        )
        .await;
    assert_eq!(s2, StatusCode::CREATED);
    let intruder_token = app.login("intruder@other.com", "StrongPass123!").await;

    let (status, _) = app
        .get_json(
            &format!("/api/v1/tickets/{ticket_id}"),
            Some(&intruder_token),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

// ─── Update status ────────────────────────────────────────────────────────────

#[tokio::test]
async fn close_ticket_returns_200_with_closed_status() {
    let app = spawn_test_app().await;
    app.register_tenant("closer@test.com", "StrongPass123!")
        .await;
    let token = app.login("closer@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "To close", "description": "closing it" }),
            &token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

    let (status, body) = app
        .patch_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/status"),
            serde_json::json!({ "status": "closed" }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["data"]["status"].as_str().unwrap(), "closed");
}

#[tokio::test]
async fn invalid_status_transition_returns_400() {
    let app = spawn_test_app().await;
    app.register_tenant("transition@test.com", "StrongPass123!")
        .await;
    let token = app.login("transition@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Ticket", "description": "test transition" }),
            &token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

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
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

    let (status, body) = app
        .get_json(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            Some(&token),
        )
        .await;

    assert_eq!(status, StatusCode::OK);
    let messages = body["data"].as_array().unwrap();
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0]["senderType"].as_str().unwrap(), "customer");
    assert_eq!(
        messages[0]["content"].as_str().unwrap(),
        "My device won't start."
    );
}

#[tokio::test]
async fn add_message_returns_201() {
    let app = spawn_test_app().await;
    app.register_tenant("reply@test.com", "StrongPass123!")
        .await;
    let token = app.login("reply@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Follow-up", "description": "Initial problem." }),
            &token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

    let (status, body) = app
        .post_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            serde_json::json!({ "content": "I tried restarting and it worked!" }),
            &token,
        )
        .await;

    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(
        body["data"]["content"].as_str().unwrap(),
        "I tried restarting and it worked!"
    );
}

#[tokio::test]
async fn cannot_add_message_to_closed_ticket() {
    let app = spawn_test_app().await;
    app.register_tenant("closed@test.com", "StrongPass123!")
        .await;
    let token = app.login("closed@test.com", "StrongPass123!").await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "Will close", "description": "Closing soon." }),
            &token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

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

// ─── Customer ownership scoping (same tenant) ───────────────────────────────────

/// Registers a tenant (admin) and two self-service customers under it, then
/// has the first customer open a ticket. Returns (admin_token, customer_b_token,
/// ticket_id) — where customer B does NOT own the ticket.
async fn setup_two_customers_one_ticket(app: &common::TestApp) -> (String, String, String) {
    // Tenant + admin. `register_tenant` derives the slug from the email
    // local-part, so "owner@scope.com" → slug "t-owner".
    let (_tenant_id, _admin_id) = app
        .register_tenant("owner@scope.com", "StrongPass123!")
        .await;
    let admin_token = app.login("owner@scope.com", "StrongPass123!").await;
    let slug = "t-owner";

    let customer_a_token = app
        .signup_customer(slug, "Customer A", "cust-a@scope.com", "StrongPass123!")
        .await;
    let customer_b_token = app
        .signup_customer(slug, "Customer B", "cust-b@scope.com", "StrongPass123!")
        .await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "A's private ticket", "description": "only mine" }),
            &customer_a_token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap().to_string();

    (admin_token, customer_b_token, ticket_id)
}

#[tokio::test]
async fn customer_cannot_get_another_customers_ticket_in_same_tenant() {
    let app = spawn_test_app().await;
    let (_admin_token, customer_b_token, ticket_id) = setup_two_customers_one_ticket(&app).await;

    let (status, _) = app
        .get_json(
            &format!("/api/v1/tickets/{ticket_id}"),
            Some(&customer_b_token),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn customer_cannot_list_another_customers_ticket_messages() {
    let app = spawn_test_app().await;
    let (_admin_token, customer_b_token, ticket_id) = setup_two_customers_one_ticket(&app).await;

    let (status, _) = app
        .get_json(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            Some(&customer_b_token),
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn customer_cannot_add_message_to_another_customers_ticket() {
    let app = spawn_test_app().await;
    let (_admin_token, customer_b_token, ticket_id) = setup_two_customers_one_ticket(&app).await;

    let (status, _) = app
        .post_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            serde_json::json!({ "content": "Sneaking into someone else's ticket." }),
            &customer_b_token,
        )
        .await;
    assert_eq!(status, StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn customer_can_still_access_their_own_ticket() {
    let app = spawn_test_app().await;

    app.register_tenant("owner@self.com", "StrongPass123!").await;
    let customer_token = app
        .signup_customer("t-owner", "Mine", "mine@self.com", "StrongPass123!")
        .await;

    let (_, created) = app
        .post_json_authed(
            "/api/v1/tickets",
            serde_json::json!({ "title": "My own ticket", "description": "belongs to me" }),
            &customer_token,
        )
        .await;
    let ticket_id = created["data"]["ticketId"].as_str().unwrap();

    let (get_status, _) = app
        .get_json(&format!("/api/v1/tickets/{ticket_id}"), Some(&customer_token))
        .await;
    assert_eq!(get_status, StatusCode::OK);

    let (msg_status, _) = app
        .get_json(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            Some(&customer_token),
        )
        .await;
    assert_eq!(msg_status, StatusCode::OK);

    let (add_status, _) = app
        .post_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            serde_json::json!({ "content": "Following up on my own ticket." }),
            &customer_token,
        )
        .await;
    assert_eq!(add_status, StatusCode::CREATED);
}

#[tokio::test]
async fn admin_retains_full_access_to_any_customer_ticket() {
    let app = spawn_test_app().await;
    let (admin_token, _customer_b_token, ticket_id) = setup_two_customers_one_ticket(&app).await;

    // Agents/admins are not scoped to ownership — the admin can read and reply to
    // a ticket opened by any customer in the tenant.
    let (get_status, _) = app
        .get_json(&format!("/api/v1/tickets/{ticket_id}"), Some(&admin_token))
        .await;
    assert_eq!(get_status, StatusCode::OK);

    let (msg_status, _) = app
        .get_json(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            Some(&admin_token),
        )
        .await;
    assert_eq!(msg_status, StatusCode::OK);

    let (add_status, _) = app
        .post_json_authed(
            &format!("/api/v1/tickets/{ticket_id}/messages"),
            serde_json::json!({ "content": "Agent replying to the customer." }),
            &admin_token,
        )
        .await;
    assert_eq!(add_status, StatusCode::CREATED);
}
