use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::message::{SenderType, TicketMessage};
use crate::domain::entities::ticket::TicketStatus;
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct AddMessageCommand {
    pub ticket_id: Uuid,
    pub tenant_id: Uuid,
    pub sender_id: Uuid,
    pub sender_type: SenderType,
    pub content: String,
}

pub struct AddMessageToTicketUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
}

impl AddMessageToTicketUseCase {
    pub fn new(uow_manager: Arc<dyn TicketingUnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "add_message",
        skip(self, command),
        fields(
            ticket_id = %command.ticket_id,
            tenant_id = %command.tenant_id,
            sender_id = %command.sender_id,
        )
    )]
    pub async fn execute(&self, command: AddMessageCommand) -> Result<TicketMessage, DomainError> {
        if command.content.trim().is_empty() {
            return Err(DomainError::EmptyMessageContent);
        }

        let mut uow = self.uow_manager.begin().await?;

        let ticket = uow
            .tickets()
            .find_by_id(command.ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;

        if ticket.tenant_id != command.tenant_id {
            tracing::warn!(ticket_id = %command.ticket_id, "cross-tenant message add rejected");
            return Err(DomainError::UnauthorizedTenantAccess);
        }

        if ticket.status == TicketStatus::Closed {
            return Err(DomainError::TicketAlreadyClosed(command.ticket_id));
        }

        let message = TicketMessage::new_human(
            command.ticket_id,
            command.sender_id,
            command.sender_type,
            command.content,
        );

        uow.messages().add_message(&message).await?;
        uow.commit().await?;

        tracing::info!(
            ticket_id = %command.ticket_id,
            message_id = %message.id,
            "message added to ticket"
        );
        Ok(message)
    }
}
