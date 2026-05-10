use axum::{
    Json, async_trait,
    extract::FromRequestParts,
    http::{StatusCode, header::AUTHORIZATION, request::Parts},
};
use serde_json::json;

use crate::app_state::AppState;
use crate::utils::jwt::{Claims, verify_jwt};

// Wrapper para usar o Extractor nos handlers
pub struct AuthUser(pub Claims);

#[async_trait]
impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, Json<serde_json::Value>);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // 1. Extrai o header Authorization
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

        // 2. Valida a assinatura criptográfica e a expiração
        let claims = verify_jwt(auth_header, &state.config.jwt_secret).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": "Unauthorized", "message": "Token inválido ou expirado"})),
            )
        })?;

        // 3. Devolve os dados seguros do usuário para o Handler
        Ok(AuthUser(claims))
    }
}
