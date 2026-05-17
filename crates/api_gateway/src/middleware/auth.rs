use axum::{
    Json,
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use domain_identity::domain::entities::Role;
use serde_json::json;

use crate::app_state::AppState;
use crate::utils::jwt::{Claims, verify_jwt};

// ==========================================
// 🛡️ Extractor 1: Usuário Autenticado Comum
// ==========================================
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .filter(|value| value.starts_with("Bearer "))
            .map(|value| value.trim_start_matches("Bearer "))
            .ok_or_else(|| {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({"error": "Unauthorized", "message": "Token ausente ou mal formatado"})),
                )
            })?;

        let claims = verify_jwt(auth_header, &state.config.jwt_secret).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized", "message": "Token inválido ou expirado"})),
            )
        })?;

        Ok(AuthUser(claims))
    }
}

// ==========================================
// 👑 Extractor 2: Apenas Administradores / Owners
// ==========================================
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
