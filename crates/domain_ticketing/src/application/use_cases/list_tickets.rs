use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::ticket::{Ticket, TicketStatus};
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct ListTicketsCommand {
    pub tenant_id: Uuid,
    pub status_filter: Option<TicketStatus>,
}

pub struct ListTicketsUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
}

impl ListTicketsUseCase {
    pub fn new(uow_manager: Arc<dyn TicketingUnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    pub async fn execute(&self, command: ListTicketsCommand) -> Result<Vec<Ticket>, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let mut tickets = uow.tickets().list_by_tenant(command.tenant_id).await?;

        if let Some(status) = command.status_filter {
            tickets.retain(|t| t.status == status);
        }

        uow.commit().await?;
        Ok(tickets)
    }
}
