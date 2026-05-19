use api_gateway::{app_state::AppState, config::AppConfig, routes::create_router};
use axum::{Router, body::Body, http::{Request, StatusCode}};
use domain_ticketing::application::workers::ai_worker::AiTask;
use http_body_util::BodyExt;
use serde_json::Value;
use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers::runners::AsyncRunner;
use testcontainers_modules::postgres::Postgres;
use tower::ServiceExt;

#[allow(dead_code)]
pub struct TestApp {
    pub router: Router,
    pub pool: PgPool,
    _container: testcontainers::ContainerAsync<Postgres>,
}

impl TestApp {
    pub async fn post_json(&self, uri: &str, body: Value) -> (StatusCode, Value) {
        let response = self
            .router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("content-type", "application/json")
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, json)
    }

    #[allow(dead_code)]
    pub async fn post_json_authed(&self, uri: &str, body: Value, token: &str) -> (StatusCode, Value) {
        let response = self
            .router
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(uri)
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", token))
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, json)
    }

    #[allow(dead_code)]
    pub async fn patch_json_authed(&self, uri: &str, body: Value, token: &str) -> (StatusCode, Value) {
        let response = self
            .router
            .clone()
            .oneshot(
                Request::builder()
                    .method("PATCH")
                    .uri(uri)
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {}", token))
                    .body(Body::from(body.to_string()))
                    .unwrap(),
            )
            .await
            .unwrap();

        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, json)
    }

    #[allow(dead_code)]
    pub async fn get_json(&self, uri: &str, token: Option<&str>) -> (StatusCode, Value) {
        let mut builder = Request::builder().method("GET").uri(uri);

        if let Some(tok) = token {
            builder = builder.header("authorization", format!("Bearer {}", tok));
        }

        let response = self
            .router
            .clone()
            .oneshot(builder.body(Body::empty()).unwrap())
            .await
            .unwrap();

        let status = response.status();
        let bytes = response.into_body().collect().await.unwrap().to_bytes();
        let json: Value = serde_json::from_slice(&bytes).unwrap_or(Value::Null);
        (status, json)
    }

    /// Registers a tenant and returns (tenant_id, user_id).
    pub async fn register_tenant(
        &self,
        email: &str,
        password: &str,
    ) -> (String, String) {
        let (status, body) = self
            .post_json(
                "/api/v1/identity/register",
                serde_json::json!({
                    "tenantName": "Test Corp",
                    "adminFullName": "Admin User",
                    "adminEmail": email,
                    "adminPassword": password
                }),
            )
            .await;

        assert_eq!(status, StatusCode::CREATED, "register failed: {body}");
        (
            body["data"]["tenantId"].as_str().unwrap().to_string(),
            body["data"]["userId"].as_str().unwrap().to_string(),
        )
    }

    /// Logs in and returns the JWT token.
    pub async fn login(&self, email: &str, password: &str) -> String {
        let (status, body) = self
            .post_json(
                "/api/v1/identity/login",
                serde_json::json!({ "email": email, "password": password }),
            )
            .await;

        assert_eq!(status, StatusCode::OK, "login failed: {body}");
        body["data"]["token"].as_str().unwrap().to_string()
    }
}

pub async fn spawn_test_app() -> TestApp {
    let _ = tracing_subscriber::fmt()
        .with_env_filter("api_gateway=debug")
        .with_test_writer()
        .try_init();

    let container = Postgres::default()
        .start()
        .await
        .expect("failed to start postgres container");

    let host_ip = container.get_host().await.expect("failed to get host");
    let host_port = container
        .get_host_port_ipv4(5432)
        .await
        .expect("failed to get port");

    let connection_string = format!(
        "postgres://postgres:postgres@{}:{}/postgres",
        host_ip, host_port
    );

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&connection_string)
        .await
        .expect("failed to connect to test db");

    sqlx::migrate!("../../migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    let config = AppConfig {
        database_url: connection_string,
        jwt_secret: "test_secret_key_for_testing_only_32chars!".to_string(),
        port: 0,
        host: "127.0.0.1".to_string(),
        frontend_url: "http://localhost:5173".to_string(),
        // Point to an unreachable address — tests use a dummy AI channel drain,
        // so the worker is never invoked during API tests.
        ollama_url: "http://127.0.0.1:1".to_string(),
        qdrant_url: "http://127.0.0.1:1".to_string(),
    };

    let (ai_sender, mut ai_receiver) = tokio::sync::mpsc::channel::<AiTask>(100);
    tokio::spawn(async move {
        while ai_receiver.recv().await.is_some() {}
    });

    let state = AppState::new(pool.clone(), config, ai_sender);
    let router = create_router(state);

    TestApp {
        router,
        pool,
        _container: container,
    }
}
