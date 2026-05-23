//! Cookie helpers for refresh token transport.
//!
//! The refresh token lives in an httpOnly cookie shared across all subdomains
//! (via `Domain=.nexus.test`/`.nexus.com`). The access token continues to be
//! returned in the response body and held in-memory on the frontend.

use axum_extra::extract::cookie::{Cookie, SameSite};
use time::Duration as TimeDuration;

use crate::config::AppConfig;

/// Cookie name carrying the refresh JWT.
pub const REFRESH_COOKIE_NAME: &str = "nexus_refresh";

/// Builds a long-lived refresh cookie with security attributes derived from config.
pub fn build_refresh_cookie<'a>(value: String, config: &AppConfig) -> Cookie<'a> {
    let mut builder = Cookie::build((REFRESH_COOKIE_NAME, value))
        .http_only(true)
        .secure(config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(TimeDuration::days(config.refresh_token_ttl_days as i64));

    if let Some(domain) = config.cookie_domain.as_deref() {
        builder = builder.domain(domain.to_string());
    }
    builder.build()
}

/// Builds an expired cookie that overrides any existing refresh cookie on the
/// client. Same domain/path/secure flags as `build_refresh_cookie` so the
/// browser actually replaces the stored value.
pub fn clear_refresh_cookie<'a>(config: &AppConfig) -> Cookie<'a> {
    let mut builder = Cookie::build((REFRESH_COOKIE_NAME, ""))
        .http_only(true)
        .secure(config.cookie_secure)
        .same_site(SameSite::Lax)
        .path("/")
        .max_age(TimeDuration::ZERO);

    if let Some(domain) = config.cookie_domain.as_deref() {
        builder = builder.domain(domain.to_string());
    }
    builder.build()
}
