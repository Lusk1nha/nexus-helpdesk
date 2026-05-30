use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::message::TicketMessage;
use crate::domain::entities::ticket::{Ticket, TicketStatus};

use crate::domain::error::DomainError;

#[async_trait]
pub trait TicketRepository: Send + Sync {
    async fn create(&self, ticket: &Ticket) -> Result<(), DomainError>;
    async fn update(&self, ticket: &Ticket) -> Result<(), DomainError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Ticket>, DomainError>;

    // Essencial para o painel do Agente
    async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Ticket>, DomainError>;
}

#[async_trait]
pub trait MessageRepository: Send + Sync {
    async fn add_message(&self, message: &TicketMessage) -> Result<(), DomainError>;

    // Essencial para passar o contexto do chamado para a IA do Ollama ler
    async fn find_by_ticket_id(&self, ticket_id: Uuid) -> Result<Vec<TicketMessage>, DomainError>;
}

/// Outbound port for pushing realtime events to connected clients.
/// The implementation lives in the api_gateway layer (RealtimeHub).
pub trait TicketEventPublisher: Send + Sync {
    fn publish_message_added(&self, ticket_id: Uuid, message: &TicketMessage);
    fn publish_status_changed(&self, ticket_id: Uuid, status: &TicketStatus);
    fn publish_ticket_created(&self, ticket: &Ticket);
    fn publish_assignee_changed(&self, ticket_id: Uuid, assignee_id: Option<Uuid>);
}
