use std::net::SocketAddr;
use std::sync::Arc;

use axum::extract::ConnectInfo;
use axum::http::Request;
use governor::clock::QuantaInstant;
use governor::middleware::NoOpMiddleware;
use tower_governor::governor::GovernorConfigBuilder;
use tower_governor::key_extractor::KeyExtractor;
use tower_governor::{GovernorError, GovernorLayer};

/// Resolves a per-client key for rate limiting, preferring real IPs from
/// (in order):
///   1. `X-Forwarded-For` first hop — set by trusted reverse proxies.
///   2. The TCP peer IP injected as `ConnectInfo` by Axum.
///   3. A static fallback (`"__unknown__"`) so requests without IP info still
///      share *a* bucket instead of blowing up the middleware (e.g. tests
///      driven via `oneshot`).
#[derive(Clone, Default)]
pub struct PeerIpOrFallback;

impl KeyExtractor for PeerIpOrFallback {
    type Key = String;

    fn extract<T>(&self, req: &Request<T>) -> Result<Self::Key, GovernorError> {
        if let Some(value) = req
            .headers()
            .get("x-forwarded-for")
            .and_then(|v| v.to_str().ok())
        {
            if let Some(first) = value.split(',').next() {
                let ip = first.trim();
                if !ip.is_empty() {
                    return Ok(ip.to_string());
                }
            }
        }

        if let Some(ConnectInfo(addr)) = req.extensions().get::<ConnectInfo<SocketAddr>>() {
            return Ok(addr.ip().to_string());
        }

        Ok("__unknown__".to_string())
    }
}

/// Builds a `GovernorLayer` that lets through `burst_size` requests immediately
/// and then refills at one request every `per_second_secs` seconds. Sized for
/// the authentication endpoints: small enough to slow brute-force and
/// credential-stuffing, generous enough not to interfere with a real user
/// re-typing a password.
pub fn auth_rate_limit_layer(
    per_second_secs: u64,
    burst_size: u32,
) -> GovernorLayer<PeerIpOrFallback, NoOpMiddleware<QuantaInstant>, axum::body::Body> {
    let config = Arc::new(
        GovernorConfigBuilder::default()
            .per_second(per_second_secs)
            .burst_size(burst_size)
            .key_extractor(PeerIpOrFallback)
            .finish()
            .expect("rate limit config inválida"),
    );

    GovernorLayer::new(config)
}
