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

    #[tracing::instrument(
        name = "get_ticket",
        skip(self, command),
        fields(ticket_id = %command.ticket_id, tenant_id = %command.tenant_id)
    )]
    pub async fn execute(&self, command: GetTicketCommand) -> Result<Ticket, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let ticket = uow
            .tickets()
            .find_by_id(command.ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;

        if ticket.tenant_id != command.tenant_id {
            tracing::warn!(
                ticket_id = %command.ticket_id,
                "cross-tenant ticket access attempt rejected"
            );
            return Err(DomainError::UnauthorizedTenantAccess);
        }

        uow.commit().await?;
        tracing::info!(ticket_id = %ticket.id, status = %ticket.status.to_string(), "ticket retrieved");
        Ok(ticket)
    }
}
