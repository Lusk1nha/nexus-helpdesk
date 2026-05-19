use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::ticket::Ticket;
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct GetTicketCommand {
    pub ticket_id: Uuid,
    pub tenant_id: Uuid,
}

pub struct GetTicketUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
}

impl GetTicketUseCase {
    pub fn new(uow_manager: Arc<dyn TicketingUnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    pub async fn execute(&self, command: GetTicketCommand) -> Result<Ticket, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let ticket = uow
            .tickets()
            .find_by_id(command.ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;

        if ticket.tenant_id != command.tenant_id {
            return Err(DomainError::UnauthorizedTenantAccess);
        }

        uow.commit().await?;
        Ok(ticket)
    }
}
