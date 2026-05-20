use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use serde::Deserialize;
use uuid::Uuid;
use validator::Validate;

use super::contracts::{
    AddMessagePayload, CreateTicketPayload, CreateTicketResponse, MessageResponse, TicketResponse,
    UpdateTicketStatusPayload,
};
use crate::{
    app_state::AppState, error::ApiError, middleware::auth::AuthUser, response::ApiResponse,
};

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
    post, path = "/api/v1/tickets",
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
) -> Result<(StatusCode, Json<ApiResponse<CreateTicketResponse>>), ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let ticket = state
        .ticketing
        .create_ticket
        .execute(CreateTicketCommand {
            tenant_id: claims.tenant_id,
            customer_id: claims.sub,
            title: payload.title,
            description: payload.description,
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %ticket.id, "ticket created via API");
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(ticket.into())),
    ))
}

// ─── List ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
pub struct ListTicketsQuery {
    pub status: Option<String>,
}

#[utoipa::path(
    get, path = "/api/v1/tickets",
    params(("status" = Option<String>, Query, description = "Filtrar por status")),
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
) -> Result<(StatusCode, Json<ApiResponse<Vec<TicketResponse>>>), ApiError> {
    let status_filter = query
        .status
        .as_deref()
        .map(|s| {
            s.parse::<TicketStatus>()
                .map_err(|_| ApiError::Internal(format!("Status inválido: '{s}'")))
        })
        .transpose()?;

    let tickets = state
        .ticketing
        .list_tickets
        .execute(ListTicketsCommand {
            tenant_id: claims.tenant_id,
            status_filter,
        })
        .await?;

    let body: Vec<TicketResponse> = tickets.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(ApiResponse::success(body))))
}

// ─── Get ──────────────────────────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/tickets/{id}",
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
) -> Result<(StatusCode, Json<ApiResponse<TicketResponse>>), ApiError> {
    let ticket = state
        .ticketing
        .get_ticket
        .execute(GetTicketCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
        })
        .await?;

    Ok((StatusCode::OK, Json(ApiResponse::success(ticket.into()))))
}

// ─── Update status ────────────────────────────────────────────────────────────

#[utoipa::path(
    patch, path = "/api/v1/tickets/{id}/status",
    request_body = UpdateTicketStatusPayload,
    params(("id" = Uuid, Path, description = "ID do ticket")),
    responses(
        (status = 200, description = "Status atualizado", body = TicketResponse),
        (status = 400, description = "Transição inválida"),
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
) -> Result<(StatusCode, Json<ApiResponse<TicketResponse>>), ApiError> {
    let new_status = payload
        .status
        .parse::<TicketStatus>()
        .map_err(|_| ApiError::Internal(format!("Status inválido: '{}'", payload.status)))?;

    let updated = state
        .ticketing
        .update_ticket_status
        .execute(UpdateTicketStatusCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
            new_status,
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %updated.id, status = %updated.status.to_string(), "ticket status changed via API");
    Ok((StatusCode::OK, Json(ApiResponse::success(updated.into()))))
}

// ─── Approve / Reject AI response ────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/tickets/{id}/approve-ai",
    params(("id" = Uuid, Path, description = "ID do ticket em awaiting_agent_approval")),
    responses(
        (status = 200, description = "Resposta da IA aprovada — ticket resolvido", body = TicketResponse),
        (status = 400, description = "Ticket não está em awaiting_agent_approval"),
        (status = 401, description = "Não autorizado"),
        (status = 404, description = "Ticket não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn approve_ai_response_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<TicketResponse>>), ApiError> {
    let ticket = state
        .ticketing
        .update_ticket_status
        .execute(UpdateTicketStatusCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
            new_status: TicketStatus::Resolved,
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %ticket.id, "AI response approved");

    // Fire-and-forget: index resolved Q&A in Qdrant for future RAG retrieval
    {
        let ai_engine = state.ai_engine.clone();
        let messages_result = state
            .ticketing
            .list_ticket_messages
            .execute(ListTicketMessagesCommand {
                ticket_id: ticket.id,
                tenant_id: ticket.tenant_id,
            })
            .await;

        let tenant_id = ticket.tenant_id;
        let ticket_id_copy = ticket.id;

        let description = ticket.description.clone();

        tokio::spawn(async move {
            let ai_reply = messages_result
                .ok()
                .and_then(|msgs| {
                    msgs.into_iter()
                        .find(|m| matches!(m.sender_type, SenderType::AI))
                        .map(|m| m.content)
                })
                .unwrap_or_default();

            let doc = if ai_reply.is_empty() {
                description
            } else {
                format!("Problema: {description}\nSolução: {ai_reply}")
            };

            if let Err(e) = ai_engine
                .index_document(&doc, tenant_id, ticket_id_copy, "resolved_ticket")
                .await
            {
                tracing::warn!(
                    ticket_id = %ticket_id_copy,
                    error = %e,
                    "failed to index resolved ticket in Qdrant — non-fatal"
                );
            } else {
                tracing::info!(ticket_id = %ticket_id_copy, "resolved Q&A pair indexed in Qdrant");
            }
        });
    }

    Ok((StatusCode::OK, Json(ApiResponse::success(ticket.into()))))
}

#[utoipa::path(
    post, path = "/api/v1/tickets/{id}/reject-ai",
    params(("id" = Uuid, Path, description = "ID do ticket em awaiting_agent_approval")),
    responses(
        (status = 200, description = "Resposta da IA rejeitada — ticket reaberto", body = TicketResponse),
        (status = 400, description = "Ticket não está em awaiting_agent_approval"),
        (status = 401, description = "Não autorizado"),
        (status = 404, description = "Ticket não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn reject_ai_response_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<TicketResponse>>), ApiError> {
    let ticket = state
        .ticketing
        .update_ticket_status
        .execute(UpdateTicketStatusCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
            new_status: TicketStatus::Open,
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %ticket.id, "AI response rejected, ticket reopened");
    Ok((StatusCode::OK, Json(ApiResponse::success(ticket.into()))))
}

// ─── Messages ─────────────────────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/tickets/{id}/messages",
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
) -> Result<(StatusCode, Json<ApiResponse<Vec<MessageResponse>>>), ApiError> {
    let messages = state
        .ticketing
        .list_ticket_messages
        .execute(ListTicketMessagesCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
        })
        .await?;

    let body: Vec<MessageResponse> = messages.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(ApiResponse::success(body))))
}

#[utoipa::path(
    post, path = "/api/v1/tickets/{id}/messages",
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
) -> Result<(StatusCode, Json<ApiResponse<MessageResponse>>), ApiError> {
    payload.validate().map_err(ApiError::Validation)?;

    let sender_type = match claims.role {
        Role::Customer => SenderType::Customer,
        Role::Agent | Role::Admin => SenderType::Agent,
    };

    let message = state
        .ticketing
        .add_message
        .execute(AddMessageCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
            sender_id: claims.sub,
            sender_type,
            content: payload.content,
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %ticket_id, message_id = %message.id, "message added via API");
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(message.into())),
    ))
}
