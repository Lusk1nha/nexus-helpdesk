use axum::{Json, extract::{Path, Query, State}, http::StatusCode};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use super::contracts::{
    AddMessagePayload, CreateTicketPayload, CreateTicketResponse, MessageResponse, TicketResponse,
    UpdateTicketStatusPayload,
};
use crate::{app_state::AppState, error::ApiError, middleware::auth::AuthUser};

use domain_identity::domain::entities::role::Role;
use domain_ticketing::application::use_cases::{
    add_message::AddMessageCommand, create_ticket::CreateTicketCommand,
    get_ticket::GetTicketCommand, list_ticket_messages::ListTicketMessagesCommand,
    list_tickets::ListTicketsCommand, update_ticket_status::UpdateTicketStatusCommand,
};
use domain_ticketing::domain::entities::message::SenderType;
use domain_ticketing::domain::entities::ticket::TicketStatus;

// ─── Create ──────────────────────────────────────────────────────────────────

#[utoipa::path(
    post,
    path = "/api/v1/tickets",
    request_body = CreateTicketPayload,
    responses(
        (status = 201, description = "Ticket criado com sucesso", body = CreateTicketResponse),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Não autorizado")
    ),
    security(("bearer_auth" = []))
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

// ─── List ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ListTicketsQuery {
    pub status: Option<String>,
}

#[utoipa::path(
    get,
    path = "/api/v1/tickets",
    params(("status" = Option<String>, Query, description = "Filtrar por status (open, processing_ai, awaiting_agent_approval, resolved, closed)")),
    responses(
        (status = 200, description = "Lista de tickets do tenant", body = Vec<TicketResponse>),
        (status = 401, description = "Não autorizado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_tickets_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Query(query): Query<ListTicketsQuery>,
) -> Result<(StatusCode, Json<Vec<TicketResponse>>), ApiError> {
    let status_filter = query
        .status
        .as_deref()
        .map(|s| {
            s.parse::<TicketStatus>()
                .map_err(|_| ApiError::Internal(format!("Status inválido: '{}'", s)))
        })
        .transpose()?;

    let command = ListTicketsCommand {
        tenant_id: claims.tenant_id,
        status_filter,
    };

    let tickets = state.ticketing.list_tickets.execute(command).await?;
    let body: Vec<TicketResponse> = tickets.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(body)))
}

// ─── Get ──────────────────────────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}",
    params(("id" = Uuid, Path, description = "ID do ticket")),
    responses(
        (status = 200, description = "Detalhes do ticket", body = TicketResponse),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Ticket não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_ticket_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
) -> Result<(StatusCode, Json<TicketResponse>), ApiError> {
    let command = GetTicketCommand {
        ticket_id,
        tenant_id: claims.tenant_id,
    };

    let ticket = state.ticketing.get_ticket.execute(command).await?;
    Ok((StatusCode::OK, Json(ticket.into())))
}

// ─── Update status ────────────────────────────────────────────────────────────

#[utoipa::path(
    patch,
    path = "/api/v1/tickets/{id}/status",
    request_body = UpdateTicketStatusPayload,
    params(("id" = Uuid, Path, description = "ID do ticket")),
    responses(
        (status = 200, description = "Status atualizado", body = TicketResponse),
        (status = 400, description = "Transição de status inválida"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Ticket não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_ticket_status_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
    Json(payload): Json<UpdateTicketStatusPayload>,
) -> Result<(StatusCode, Json<TicketResponse>), ApiError> {
    let new_status = payload
        .status
        .parse::<TicketStatus>()
        .map_err(|_| ApiError::Internal(format!("Status inválido: '{}'", payload.status)))?;

    let command = UpdateTicketStatusCommand {
        ticket_id,
        tenant_id: claims.tenant_id,
        new_status,
    };

    let updated = state
        .ticketing
        .update_ticket_status
        .execute(command)
        .await?;
    Ok((StatusCode::OK, Json(updated.into())))
}

// ─── Messages ─────────────────────────────────────────────────────────────────

#[utoipa::path(
    get,
    path = "/api/v1/tickets/{id}/messages",
    params(("id" = Uuid, Path, description = "ID do ticket")),
    responses(
        (status = 200, description = "Mensagens do ticket", body = Vec<MessageResponse>),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Ticket não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_ticket_messages_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
) -> Result<(StatusCode, Json<Vec<MessageResponse>>), ApiError> {
    let command = ListTicketMessagesCommand {
        ticket_id,
        tenant_id: claims.tenant_id,
    };

    let messages = state
        .ticketing
        .list_ticket_messages
        .execute(command)
        .await?;
    let body: Vec<MessageResponse> = messages.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(body)))
}

#[utoipa::path(
    post,
    path = "/api/v1/tickets/{id}/messages",
    request_body = AddMessagePayload,
    params(("id" = Uuid, Path, description = "ID do ticket")),
    responses(
        (status = 201, description = "Mensagem adicionada", body = MessageResponse),
        (status = 400, description = "Conteúdo vazio ou ticket fechado"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado"),
        (status = 404, description = "Ticket não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn add_message_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
    Json(payload): Json<AddMessagePayload>,
) -> Result<(StatusCode, Json<MessageResponse>), ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let sender_type = match claims.role {
        Role::Customer => SenderType::Customer,
        Role::Agent | Role::Admin => SenderType::Agent,
    };

    let command = AddMessageCommand {
        ticket_id,
        tenant_id: claims.tenant_id,
        sender_id: claims.sub,
        sender_type,
        content: payload.content,
    };

    let message = state.ticketing.add_message.execute(command).await?;
    Ok((StatusCode::CREATED, Json(message.into())))
}
