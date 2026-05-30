use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::ticket::Ticket;
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct AssignTicketCommand {
    pub ticket_id: Uuid,
    pub tenant_id: Uuid,
    /// Agent taking ownership of the ticket (self-assignment).
    pub assignee_id: Uuid,
}

pub struct AssignTicketUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
}

impl AssignTicketUseCase {
    pub fn new(uow_manager: Arc<dyn TicketingUnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "assign_ticket",
        skip(self, command),
        fields(
            ticket_id = %command.ticket_id,
            tenant_id = %command.tenant_id,
            assignee_id = %command.assignee_id,
        )
    )]
    pub async fn execute(&self, command: AssignTicketCommand) -> Result<Ticket, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let mut ticket = uow
            .tickets()
            .find_by_id(command.ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;

        if ticket.tenant_id != command.tenant_id {
            tracing::warn!(ticket_id = %command.ticket_id, "cross-tenant assignment rejected");
            return Err(DomainError::UnauthorizedTenantAccess);
        }

        ticket.assign_to(command.assignee_id);

        uow.tickets().update(&ticket).await?;
        uow.commit().await?;

        tracing::info!(
            ticket_id = %ticket.id,
            assignee_id = %command.assignee_id,
            "ticket assigned"
        );
        Ok(ticket)
    }
}
