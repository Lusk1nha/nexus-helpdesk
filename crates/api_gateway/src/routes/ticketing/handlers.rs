use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use super::contracts::{CreateTicketPayload, CreateTicketResponse};
use crate::{app_state::AppState, error::ApiError, middleware::auth::AuthUser};
use domain_ticketing::application::use_cases::create_ticket::CreateTicketCommand;

#[utoipa::path(
    post,
    path = "/api/v1/tickets",
    request_body = CreateTicketPayload,
    responses(
        (status = 201, description = "Ticket criado com sucesso", body = CreateTicketResponse),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Não autorizado")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_ticket_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<CreateTicketPayload>,
) -> Result<(StatusCode, Json<CreateTicketResponse>), ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let command = CreateTicketCommand {
        tenant_id: claims.tenant_id,
        customer_id: claims.sub,
        title: payload.title,
        description: payload.description,
    };

    let ticket = state.ticketing.create_ticket.execute(command).await?;

    Ok((StatusCode::CREATED, Json(ticket.into())))
}
