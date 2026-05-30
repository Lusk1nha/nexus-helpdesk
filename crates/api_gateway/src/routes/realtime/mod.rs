use std::convert::Infallible;
use std::time::Duration;

use axum::Router;
use axum::extract::{Path, State};
use axum::response::sse::{Event, KeepAlive, Sse};
use axum::routing::get;
use tokio_stream::StreamExt;
use tokio_stream::wrappers::BroadcastStream;
use uuid::Uuid;

use crate::app_state::AppState;
use crate::error::ApiError;
use crate::middleware::auth::AuthUser;
use domain_identity::domain::entities::role::Role;

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/tickets/{ticket_id}", get(ticket_events_handler))
        .route("/system", get(system_events_handler))
}

// ─── Ticket SSE ──────────────────────────────────────────────────────────────

/// SSE stream for a specific ticket.
///
/// Emits `message_added` and `status_changed` events as they happen.
/// Customers may only subscribe to their own tickets; agents/admins may
/// subscribe to any ticket in their tenant.
pub async fn ticket_events_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(ticket_id): Path<Uuid>,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>>, ApiError> {
    // Verify the ticket belongs to the caller's tenant, and — for customers —
    // that they own it. Agents/admins may subscribe to any ticket in the tenant.
    use domain_ticketing::application::use_cases::get_ticket::GetTicketCommand;
    let customer_filter = match claims.role {
        Role::Customer => Some(claims.sub),
        Role::Agent | Role::Admin => None,
    };
    state
        .ticketing
        .get_ticket
        .execute(GetTicketCommand {
            ticket_id,
            tenant_id: claims.tenant_id,
            customer_filter,
        })
        .await?;

    let rx = state.realtime.subscribe_ticket(ticket_id);
    let stream = BroadcastStream::new(rx).filter_map(move |result| match result {
        Ok(event) => {
            let data = serde_json::to_string(&event).unwrap_or_default();
            let event_name = match &event {
                crate::realtime::TicketSseEvent::MessageAdded { .. } => "message_added",
                crate::realtime::TicketSseEvent::StatusChanged { .. } => "status_changed",
                crate::realtime::TicketSseEvent::AssigneeChanged { .. } => "assignee_changed",
            };
            Some(Ok(Event::default().event(event_name).data(data)))
        }
        Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(n)) => {
            tracing::warn!(ticket_id = %ticket_id, missed = n, "SSE client lagged on ticket stream");
            None
        }
    });

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    ))
}

// ─── System SSE ──────────────────────────────────────────────────────────────

/// SSE stream for system-wide events (agents and admins only).
///
/// Emits `ticket_created` and `ticket_status_changed` events.
pub async fn system_events_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
) -> Result<Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>>, ApiError> {
    if matches!(claims.role, Role::Customer) {
        return Err(ApiError::Forbidden(
            "Apenas agentes e administradores podem assinar eventos do sistema.".to_string(),
        ));
    }

    let rx = state.realtime.subscribe_system();
    let stream = BroadcastStream::new(rx).filter_map(move |result| match result {
        Ok(event) => {
            // Filter events to the caller's tenant.
            let tenant_matches = match &event {
                crate::realtime::SystemSseEvent::TicketCreated { tenant_id, .. } => {
                    *tenant_id == claims.tenant_id
                }
                crate::realtime::SystemSseEvent::TicketStatusChanged { tenant_id, .. } => {
                    // Events from AiWorker carry Uuid::nil as tenant_id.
                    // Pass them through; the client can match by ticket_id.
                    tenant_id.is_nil() || *tenant_id == claims.tenant_id
                }
            };
            if !tenant_matches {
                return None;
            }

            let data = serde_json::to_string(&event).unwrap_or_default();
            let event_name = match &event {
                crate::realtime::SystemSseEvent::TicketCreated { .. } => "ticket_created",
                crate::realtime::SystemSseEvent::TicketStatusChanged { .. } => {
                    "ticket_status_changed"
                }
            };
            Some(Ok(Event::default().event(event_name).data(data)))
        }
        Err(tokio_stream::wrappers::errors::BroadcastStreamRecvError::Lagged(n)) => {
            tracing::warn!(missed = n, "SSE client lagged on system stream");
            None
        }
    });

    Ok(Sse::new(stream).keep_alive(
        KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    ))
}
