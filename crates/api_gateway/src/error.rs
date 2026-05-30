use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

use domain_identity::domain::error::DomainError as IdentityError;
use domain_ticketing::domain::error::DomainError as TicketingError;

pub enum ApiError {
    Validation(validator::ValidationErrors),
    Identity(IdentityError),
    Ticketing(TicketingError),
    Forbidden(String),
    BadRequest(String),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            ApiError::Validation(errs) => {
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

            ApiError::Identity(IdentityError::UserAlreadyExists) => (
                StatusCode::CONFLICT,
                "Este e-mail já está em uso.".to_string(),
            ),
            ApiError::Identity(IdentityError::UserNotFound) => {
                (StatusCode::NOT_FOUND, "Usuário não encontrado.".to_string())
            }
            ApiError::Identity(IdentityError::TenantNotFound) => {
                (StatusCode::NOT_FOUND, "Empresa não encontrada.".to_string())
            }
            ApiError::Identity(IdentityError::InvalidCredentials) => (
                StatusCode::UNAUTHORIZED,
                "Credenciais inválidas.".to_string(),
            ),
            ApiError::Identity(IdentityError::InvalidRole(role)) => (
                StatusCode::BAD_REQUEST,
                format!("Role inválido: '{role}'. Use: admin, agent ou customer."),
            ),
            ApiError::Identity(IdentityError::SlugAlreadyTaken(slug)) => (
                StatusCode::CONFLICT,
                format!("O slug '{slug}' já está em uso por outra empresa."),
            ),
            ApiError::Identity(err @ IdentityError::InvalidSlug(_)) => {
                (StatusCode::BAD_REQUEST, err.to_string())
            }
            ApiError::Identity(IdentityError::DatabaseError(msg)) => {
                tracing::error!(domain = "identity", error = %msg, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor.".to_string(),
                )
            }
            ApiError::Identity(err) => (StatusCode::BAD_REQUEST, err.to_string()),

            ApiError::Ticketing(TicketingError::TicketNotFound)
            | ApiError::Ticketing(TicketingError::MessageNotFound) => {
                (StatusCode::NOT_FOUND, "Recurso não encontrado.".to_string())
            }
            ApiError::Ticketing(err @ TicketingError::UnauthorizedTenantAccess)
            | ApiError::Ticketing(err @ TicketingError::UnauthorizedTicketAccess) => {
                (StatusCode::FORBIDDEN, err.to_string())
            }
            ApiError::Ticketing(TicketingError::AiEngineError(msg)) => {
                tracing::error!(domain = "ticketing", error = %msg, "AI engine error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A Inteligência Artificial está temporariamente indisponível.".to_string(),
                )
            }
            ApiError::Ticketing(TicketingError::DatabaseError(msg)) => {
                tracing::error!(domain = "ticketing", error = %msg, "database error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor.".to_string(),
                )
            }
            ApiError::Ticketing(err) => (StatusCode::BAD_REQUEST, err.to_string()),

            ApiError::Forbidden(msg) => (StatusCode::FORBIDDEN, msg),

            ApiError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg),

            ApiError::Internal(msg) => {
                tracing::error!(error = %msg, "unhandled internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor.".to_string(),
                )
            }
        };

        // Derive a SCREAMING_SNAKE_CASE code from the canonical HTTP reason.
        let code = status
            .canonical_reason()
            .unwrap_or("ERROR")
            .to_uppercase()
            .replace(' ', "_");

        let body = Json(json!({
            "error": {
                "code": code,
                "message": message
            }
        }));

        (status, body).into_response()
    }
}

impl From<IdentityError> for ApiError {
    fn from(err: IdentityError) -> Self {
        ApiError::Identity(err)
    }
}

impl From<TicketingError> for ApiError {
    fn from(err: TicketingError) -> Self {
        ApiError::Ticketing(err)
    }
}

impl From<validator::ValidationErrors> for ApiError {
    fn from(err: validator::ValidationErrors) -> Self {
        ApiError::Validation(err)
    }
}
