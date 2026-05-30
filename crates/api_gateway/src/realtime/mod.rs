use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::Serialize;
use time::OffsetDateTime;
use tokio::sync::broadcast;
use uuid::Uuid;

use domain_ticketing::domain::entities::message::TicketMessage;
use domain_ticketing::domain::entities::ticket::{Ticket, TicketStatus};
use domain_ticketing::domain::ports::TicketEventPublisher;

const TICKET_CHANNEL_CAPACITY: usize = 64;
const SYSTEM_CHANNEL_CAPACITY: usize = 256;

// ─── Event types ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum TicketSseEvent {
    MessageAdded {
        ticket_id: Uuid,
        message_id: Uuid,
        sender_id: Option<Uuid>,
        sender_type: String,
        content: String,
        is_internal_note: bool,
        #[serde(with = "time::serde::rfc3339")]
        created_at: OffsetDateTime,
    },
    StatusChanged {
        ticket_id: Uuid,
        new_status: String,
    },
    AssigneeChanged {
        ticket_id: Uuid,
        assignee_id: Option<Uuid>,
    },
}

/// System-wide events broadcast to agent/admin dashboards.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SystemSseEvent {
    TicketCreated {
        ticket_id: Uuid,
        tenant_id: Uuid,
        customer_id: Uuid,
        title: String,
        status: String,
    },
    TicketStatusChanged {
        ticket_id: Uuid,
        tenant_id: Uuid,
        new_status: String,
    },
}

// ─── Hub ──────────────────────────────────────────────────────────────────────

/// Central broker for all realtime SSE streams.
///
/// - One `broadcast` channel per ticket for ticket-scoped events.
/// - One global channel for system-wide events (agent dashboards).
///
/// Uses `std::sync::RwLock` so that the synchronous `TicketEventPublisher`
/// implementation can call it without needing an async context.
#[derive(Clone)]
pub struct RealtimeHub {
    ticket_channels: Arc<RwLock<HashMap<Uuid, broadcast::Sender<TicketSseEvent>>>>,
    system_tx: broadcast::Sender<SystemSseEvent>,
}

impl RealtimeHub {
    pub fn new() -> Self {
        let (system_tx, _) = broadcast::channel(SYSTEM_CHANNEL_CAPACITY);
        Self {
            ticket_channels: Arc::new(RwLock::new(HashMap::new())),
            system_tx,
        }
    }

    /// Returns an existing sender for `ticket_id`, or creates a new channel.
    fn get_or_create_ticket_sender(&self, ticket_id: Uuid) -> broadcast::Sender<TicketSseEvent> {
        {
            let channels = self.ticket_channels.read().unwrap();
            if let Some(tx) = channels.get(&ticket_id) {
                return tx.clone();
            }
        }
        let mut channels = self.ticket_channels.write().unwrap();
        
        if let Some(tx) = channels.get(&ticket_id) {
            return tx.clone();
        }
        
        let (tx, _) = broadcast::channel(TICKET_CHANNEL_CAPACITY);
        channels.insert(ticket_id, tx.clone());
        tx
    }

    pub fn subscribe_ticket(&self, ticket_id: Uuid) -> broadcast::Receiver<TicketSseEvent> {
        self.get_or_create_ticket_sender(ticket_id).subscribe()
    }

    pub fn subscribe_system(&self) -> broadcast::Receiver<SystemSseEvent> {
        self.system_tx.subscribe()
    }
}

// ─── TicketEventPublisher impl ───────────────────────────────────────────────

impl TicketEventPublisher for RealtimeHub {
    fn publish_message_added(&self, ticket_id: Uuid, message: &TicketMessage) {
        let tx = self.get_or_create_ticket_sender(ticket_id);
        let event = TicketSseEvent::MessageAdded {
            ticket_id,
            message_id: message.id,
            sender_id: message.sender_id,
            sender_type: message.sender_type.to_string(),
            content: message.content.clone(),
            is_internal_note: message.is_internal_note,
            created_at: message.created_at,
        };
        let _ = tx.send(event);
    }

    fn publish_status_changed(&self, ticket_id: Uuid, status: &TicketStatus) {
        let status_str = status.to_string();

        // Ticket-scoped event
        {
            let channels = self.ticket_channels.read().unwrap();
            if let Some(tx) = channels.get(&ticket_id) {
                let _ = tx.send(TicketSseEvent::StatusChanged {
                    ticket_id,
                    new_status: status_str.clone(),
                });
            }
        }

        // System-wide event (no tenant_id available from this port, use Nil)
        let _ = self.system_tx.send(SystemSseEvent::TicketStatusChanged {
            ticket_id,
            tenant_id: Uuid::nil(),
            new_status: status_str,
        });
    }

    fn publish_assignee_changed(&self, ticket_id: Uuid, assignee_id: Option<Uuid>) {
        let channels = self.ticket_channels.read().unwrap();
        if let Some(tx) = channels.get(&ticket_id) {
            let _ = tx.send(TicketSseEvent::AssigneeChanged {
                ticket_id,
                assignee_id,
            });
        }
    }

    fn publish_ticket_created(&self, ticket: &Ticket) {
        let _ = self.system_tx.send(SystemSseEvent::TicketCreated {
            ticket_id: ticket.id,
            tenant_id: ticket.tenant_id,
            customer_id: ticket.customer_id,
            title: ticket.title.clone(),
            status: ticket.status.to_string(),
        });
    }
}
