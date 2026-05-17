// crates/api_gateway/src/error.rs

use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

// Aliases para evitar colisão de nomes entre os domínios
use domain_identity::domain::error::DomainError as IdentityError;
use domain_ticketing::domain::error::DomainError as TicketingError;

pub enum ApiError {
    Validation(validator::ValidationErrors),
    Identity(IdentityError),
    Ticketing(TicketingError),
    Internal(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            // --- ERROS DE VALIDAÇÃO (Payloads) ---
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

            // --- ERROS DE IDENTIDADE (IAM) ---
            ApiError::Identity(IdentityError::UserAlreadyExists) => (
                StatusCode::CONFLICT,
                "Este e-mail já está em uso.".to_string(),
            ),
            ApiError::Identity(IdentityError::InvalidCredentials) => (
                StatusCode::UNAUTHORIZED,
                "Credenciais inválidas.".to_string(),
            ),
            ApiError::Identity(IdentityError::DatabaseError(msg)) => {
                tracing::error!("Erro de Banco (Identity): {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor.".to_string(),
                )
            }
            ApiError::Identity(err) => (StatusCode::BAD_REQUEST, err.to_string()),

            // --- ERROS DE TICKETING (Engine & AI) ---
            ApiError::Ticketing(TicketingError::TicketNotFound)
            | ApiError::Ticketing(TicketingError::MessageNotFound) => {
                (StatusCode::NOT_FOUND, "Recurso não encontrado.".to_string())
            }
            ApiError::Ticketing(err @ TicketingError::UnauthorizedTenantAccess) => (
                StatusCode::FORBIDDEN,
                err.to_string(), // Retorna "Acesso negado: Este ticket pertence a outra empresa"
            ),
            ApiError::Ticketing(TicketingError::AiEngineError(msg)) => {
                tracing::error!("Falha crítica no Motor de IA: {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "A Inteligência Artificial está temporariamente indisponível.".to_string(),
                )
            }
            ApiError::Ticketing(TicketingError::DatabaseError(msg)) => {
                tracing::error!("Erro de Banco (Ticketing): {}", msg);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Erro interno do servidor.".to_string(),
                )
            }
            ApiError::Ticketing(err) => {
                // Fallback para Erros de Regra de Negócio (Ticket fechado, transição inválida, etc)
                (StatusCode::BAD_REQUEST, err.to_string())
            }

            // --- ERROS INTERNOS GENÉRICOS ---
            ApiError::Internal(msg) => {
                tracing::error!("Erro interno genérico: {}", msg);
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

// ==========================================
// Mágica do Operador `?`
// ==========================================

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
