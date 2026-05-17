use axum::{Json, http::StatusCode, response::IntoResponse};
use domain_identity::domain::error::DomainError;
use serde_json::json;

pub enum ApiError {
    Validation(validator::ValidationErrors),
    Domain(DomainError),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            ApiError::Validation(errs) => {
                // Pega a primeira mensagem de erro de validação para facilitar pro Frontend
                let msg = errs
                    .field_errors()
                    .into_iter()
                    .next()
                    .and_then(|(_, errors)| errors.first())
                    .and_then(|err| err.message.as_ref())
                    .map(|cow| cow.to_string())
                    .unwrap_or_else(|| "Erro de validação nos dados enviados.".to_string());

                (StatusCode::BAD_REQUEST, msg)
            }
            ApiError::Domain(DomainError::UserAlreadyExists) => (
                StatusCode::CONFLICT,
                "Este e-mail já está em uso.".to_string(),
            ),
            ApiError::Domain(err) => {
                // Um fallback para outros erros de negócio do domínio
                (StatusCode::BAD_REQUEST, err.to_string())
            }
            ApiError::Internal(msg) => {
                tracing::error!("Erro interno: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor.".to_string(),
                )
            }
        };

        let body = Json(json!({
            "error": status.canonical_reason().unwrap_or("Error"),
            "message": error_message
        }));

        (status, body).into_response()
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        ApiError::Domain(err)
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        ApiError::Validation(err)
    }
}
