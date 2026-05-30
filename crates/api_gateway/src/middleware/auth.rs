use axum::{
    Json,
    extract::FromRequestParts,
    http::{HeaderName, StatusCode, header::AUTHORIZATION, request::Parts},
};
use domain_identity::application::use_cases::AuthenticateApiKeyCommand;
use domain_identity::domain::entities::Role;
use serde_json::json;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::utils::jwt::{Claims, verify_jwt};
use crate::utils::secret::sha256_hex;

const API_KEY_HEADER: HeaderName = HeaderName::from_static("x-api-key");

pub struct AuthUser(pub Claims);

fn unauthorized(message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::UNAUTHORIZED,
        Json(json!({"error": "Unauthorized", "message": message})),
    )
}

async fn authenticate_with_api_key(
    parts: &Parts,
    state: &AppState,
) -> Option<Result<Claims, (StatusCode, Json<serde_json::Value>)>> {
    let raw = parts
        .headers
        .get(&API_KEY_HEADER)
        .and_then(|v| v.to_str().ok())?
        .trim()
        .to_string();
    if raw.is_empty() {
        return None;
    }

    let hash = sha256_hex(&raw);
    let result = state
        .identity
        .authenticate_api_key
        .execute(AuthenticateApiKeyCommand { key_hash: hash })
        .await;

    let api_key = match result {
        Ok(k) => k,
        Err(e) => {
            tracing::warn!(path = %parts.uri, error = %e, "auth rejected: invalid API key");
            return Some(Err(unauthorized("API key inválida ou revogada.")));
        }
    };

    // Synthesize claims so the rest of the request pipeline (which expects
    // user-flavoured Claims) keeps working. `sub` here is the API key id.
    let now = time::OffsetDateTime::now_utc().unix_timestamp() as usize;
    let exp_secs = api_key
        .expires_at
        .map(|t| t.unix_timestamp() as usize)
        .unwrap_or(now + 60 * 60);

    Some(Ok(Claims {
        sub: api_key.id,
        tenant_id: api_key.tenant_id,
        role: api_key.role,
        jti: api_key.id,
        iss: state.config.jwt_issuer.clone(),
        iat: now,
        exp: exp_secs,
    }))
}

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let bearer = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| value.trim_start_matches("Bearer ").to_string());

        // EventSource doesn't support custom headers, so SSE endpoints pass
        // the token as ?token=<jwt>. Accept it as a fallback.
        let query_token = parts.uri.query().and_then(|q| {
            q.split('&').find_map(|pair| {
                let (k, v) = pair.split_once('=')?;
                (k == "token").then(|| v.to_string())
            })
        });

        if let Some(token) = bearer.or(query_token) {
            return verify_jwt(&token, &state.config.jwt_secret, &state.config.jwt_issuer)
                .map(AuthUser)
                .map_err(|e| {
                    tracing::warn!(path = %parts.uri, error = %e, "auth rejected: invalid bearer token");
                    unauthorized("Token inválido ou expirado.")
                });
        }

        if let Some(result) = authenticate_with_api_key(parts, state).await {
            return result.map(AuthUser);
        }

        tracing::warn!(path = %parts.uri, "auth rejected: missing credentials");
        Err(unauthorized(
            "Credenciais ausentes: envie um Bearer token ou o header X-API-Key.",
        ))
    }
}

/// Helper exposed for handlers that want to know *which* credential type
/// authenticated the request (e.g. to forbid API keys on certain endpoints).
#[allow(dead_code)]
pub fn is_api_key_subject(claims: &Claims) -> bool {
    claims.sub == claims.jti && claims.iat == claims.exp.saturating_sub(60 * 60)
}

#[allow(dead_code)]
pub fn placeholder_api_key_id(claims: &Claims) -> Uuid {
    claims.sub
}

pub struct AdminUser(pub Claims);

impl FromRequestParts<AppState> for AdminUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let AuthUser(claims) = AuthUser::from_request_parts(parts, state).await?;

        match claims.role {
            Role::Admin => {}
            _ => {
                tracing::warn!(
                    path = %parts.uri,
                    user_id = %claims.sub,
                    tenant_id = %claims.tenant_id,
                    role = %claims.role,
                    "access denied: admin role required"
                );
                return Err((
                    StatusCode::FORBIDDEN,
                    Json(json!({
                        "error": "Forbidden",
                        "message": "Acesso negado: Esta operação exige privilégios de administrador."
                    })),
                ));
            }
        }

        Ok(AdminUser(claims))
    }
}
