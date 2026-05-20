use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::ticket::{Ticket, TicketStatus};
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct UpdateTicketStatusCommand {
    pub ticket_id: Uuid,
    pub tenant_id: Uuid,
    pub new_status: TicketStatus,
}

pub struct UpdateTicketStatusUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
}

impl UpdateTicketStatusUseCase {
    pub fn new(uow_manager: Arc<dyn TicketingUnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "update_ticket_status",
        skip(self, command),
        fields(
            ticket_id = %command.ticket_id,
            tenant_id = %command.tenant_id,
            new_status = %command.new_status.to_string(),
        )
    )]
    pub async fn execute(&self, command: UpdateTicketStatusCommand) -> Result<Ticket, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let mut ticket = uow
            .tickets()
            .find_by_id(command.ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;

        if ticket.tenant_id != command.tenant_id {
            tracing::warn!(ticket_id = %command.ticket_id, "cross-tenant status update rejected");
            return Err(DomainError::UnauthorizedTenantAccess);
        }

        if ticket.status == TicketStatus::Closed {
            return Err(DomainError::TicketAlreadyClosed(command.ticket_id));
        }

        if !ticket.can_transition_to(&command.new_status) {
            tracing::warn!(
                ticket_id = %command.ticket_id,
                from = %ticket.status.to_string(),
                to = %command.new_status.to_string(),
                "invalid ticket status transition"
            );
            return Err(DomainError::InvalidStatusTransition {
                from: ticket.status.to_string(),
                to: command.new_status.to_string(),
            });
        }

        let prev_status = ticket.status.to_string();
        match &command.new_status {
            TicketStatus::Open => ticket.revert_to_open(),
            TicketStatus::Resolved => ticket.resolve(),
            TicketStatus::Closed => ticket.close(),
            other => {
                return Err(DomainError::InvalidStatusTransition {
                    from: prev_status,
                    to: other.to_string(),
                });
            }
        }

        uow.tickets().update(&ticket).await?;
        uow.commit().await?;

        tracing::info!(
            ticket_id = %ticket.id,
            from = %prev_status,
            to = %ticket.status.to_string(),
            "ticket status updated"
        );
        Ok(ticket)
    }
}
