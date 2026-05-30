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
    add_message::AddMessageCommand, assign_ticket::AssignTicketCommand,
    create_ticket::CreateTicketCommand, get_ticket::GetTicketCommand,
    list_ticket_messages::ListTicketMessagesCommand, list_tickets::ListTicketsCommand,
    update_ticket_status::UpdateTicketStatusCommand,
};
use domain_ticketing::domain::entities::message::SenderType;
use domain_ticketing::domain::entities::ticket::{TicketPriority, TicketStatus};
use domain_ticketing::domain::ports::TicketEventPublisher;

use crate::utils::jwt::Claims;

/// Customers may only access their own tickets; agents/admins access any ticket
/// in the tenant. Returns the customer id to scope by, or `None` for full access.
fn customer_filter_for(claims: &Claims) -> Option<Uuid> {
    match claims.role {
        Role::Customer => Some(claims.sub),
        Role::Agent | Role::Admin => None,
    }
}

// ─── Create ──────────────────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/tickets",
    tag = "Ticketing",
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

    let priority = payload
        .priority
        .as_deref()
        .map(|p| {
            p.parse::<TicketPriority>()
                .map_err(|_| ApiError::Internal(format!("Prioridade inválida: '{p}'")))
        })
        .transpose()?
        .unwrap_or(TicketPriority::Normal);

    let ticket = state
        .ticketing
        .create_ticket
        .execute(CreateTicketCommand {
            tenant_id: claims.tenant_id,
            customer_id: claims.sub,
            title: payload.title,
            description: payload.description,
            priority,
            category: payload.category.filter(|c| !c.trim().is_empty()),
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %ticket.id, "ticket created via API");
    state.realtime.publish_ticket_created(&ticket);
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
    tag = "Ticketing",
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

    // Customers may only see their own tickets; agents/admins see all.
    let customer_filter = customer_filter_for(&claims);

    let tickets = state
        .ticketing
        .list_tickets
        .execute(ListTicketsCommand {
            tenant_id: claims.tenant_id,
            status_filter,
            customer_filter,
        })
        .await?;

    let body: Vec<TicketResponse> = tickets.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(ApiResponse::success(body))))
}

// ─── Get ──────────────────────────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/tickets/{id}",
    tag = "Ticketing",
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
            customer_filter: customer_filter_for(&claims),
        })
        .await?;

    Ok((StatusCode::OK, Json(ApiResponse::success(ticket.into()))))
}

// ─── Update status ────────────────────────────────────────────────────────────

#[utoipa::path(
    patch, path = "/api/v1/tickets/{id}/status",
    tag = "Ticketing",
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
    state
        .realtime
        .publish_status_changed(updated.id, &updated.status);
    Ok((StatusCode::OK, Json(ApiResponse::success(updated.into()))))
}

// ─── Approve / Reject AI response ────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/tickets/{id}/approve-ai",
    tag = "Ticketing",
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
    state
        .realtime
        .publish_status_changed(ticket.id, &ticket.status);

    // Fire-and-forget: index resolved Q&A in Qdrant for future RAG retrieval
    {
        let ai_engine = state.ai_engine.clone();
        let messages_result = state
            .ticketing
            .list_ticket_messages
            .execute(ListTicketMessagesCommand {
                ticket_id: ticket.id,
                tenant_id: ticket.tenant_id,
                // System-side indexing for RAG; not scoped to a customer.
                customer_filter: None,
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

            let title = format!("Ticket resolvido #{}", ticket_id_copy);
            if let Err(e) = ai_engine
                .index_document(
                    &doc,
                    &title,
                    tenant_id,
                    ticket_id_copy,
                    "resolved_ticket",
                    "system",
                )
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
    tag = "Ticketing",
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
    state
        .realtime
        .publish_status_changed(ticket.id, &ticket.status);
    Ok((StatusCode::OK, Json(ApiResponse::success(ticket.into()))))
}

// ─── Assign (agent self-assignment) ────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/tickets/{id}/assign",
    tag = "Ticketing",
    params(("id" = Uuid, Path, description = "ID do ticket")),
    responses(
        (status = 200, description = "Ticket atribuído ao agente", body = TicketResponse),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Apenas agentes/admins podem assumir tickets"),
        (status = 404, description = "Ticket não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn assign_ticket_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
) -> Result<(StatusCode, Json<ApiResponse<TicketResponse>>), ApiError> {
    // Customers cannot take ownership of tickets — only agents/admins.
    if matches!(claims.role, Role::Customer) {
        return Err(ApiError::Forbidden(
            "Apenas agentes podem assumir tickets.".to_string(),
        ));
    }

    let ticket = state
        .ticketing
        .assign_ticket
        .execute(AssignTicketCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
            assignee_id: claims.sub,
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %ticket.id, "ticket self-assigned via API");
    state
        .realtime
        .publish_assignee_changed(ticket.id, ticket.assignee_id);
    Ok((StatusCode::OK, Json(ApiResponse::success(ticket.into()))))
}

// ─── Messages ─────────────────────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/tickets/{id}/messages",
    tag = "Ticketing",
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
            customer_filter: customer_filter_for(&claims),
        })
        .await?;

    let body: Vec<MessageResponse> = messages.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(ApiResponse::success(body))))
}

#[utoipa::path(
    post, path = "/api/v1/tickets/{id}/messages",
    tag = "Ticketing",
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
            customer_filter: customer_filter_for(&claims),
        })
        .await?;

    tracing::info!(user_id = %claims.sub, ticket_id = %ticket_id, message_id = %message.id, "message added via API");
    state.realtime.publish_message_added(ticket_id, &message);
    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(message.into())),
    ))
}
