use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use domain_identity::domain::entities::Role;
use serde_json::json;

use crate::app_state::AppState;
use crate::utils::jwt::{Claims, verify_jwt};

pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| value.trim_start_matches("Bearer "))
            .ok_or_else(|| {
                tracing::warn!(path = %parts.uri, "auth rejected: missing or malformed token");
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Unauthorized", "message": "Token ausente ou mal formatado"})),
                )
            })?;

        let claims = verify_jwt(token, &state.config.jwt_secret).map_err(|e| {
            tracing::warn!(path = %parts.uri, error = %e, "auth rejected: invalid token");
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized", "message": "Token inválido ou expirado"})),
            )
        })?;

        Ok(AuthUser(claims))
    }
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
