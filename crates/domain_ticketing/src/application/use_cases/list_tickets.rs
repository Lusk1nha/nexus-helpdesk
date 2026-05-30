use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::ticket::{Ticket, TicketStatus};
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct ListTicketsCommand {
    pub tenant_id: Uuid,
    pub status_filter: Option<TicketStatus>,
    /// When set, only tickets opened by this customer are returned.
    /// Agents/admins pass `None` to see every ticket in the tenant.
    pub customer_filter: Option<Uuid>,
}

pub struct ListTicketsUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
}

impl ListTicketsUseCase {
    pub fn new(uow_manager: Arc<dyn TicketingUnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "list_tickets",
        skip(self, command),
        fields(tenant_id = %command.tenant_id)
    )]
    pub async fn execute(&self, command: ListTicketsCommand) -> Result<Vec<Ticket>, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let mut tickets = uow.tickets().list_by_tenant(command.tenant_id).await?;

        if let Some(customer_id) = command.customer_filter {
            tickets.retain(|t| t.customer_id == customer_id);
        }

        if let Some(status) = command.status_filter {
            tickets.retain(|t| t.status == status);
        }

        uow.commit().await?;
        tracing::info!(count = tickets.len(), "tickets listed");
        Ok(tickets)
    }
}
