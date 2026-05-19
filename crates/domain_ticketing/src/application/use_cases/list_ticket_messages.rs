use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::message::TicketMessage;
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct ListTicketMessagesCommand {
    pub ticket_id: Uuid,
    pub tenant_id: Uuid,
}

pub struct ListTicketMessagesUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
}

impl ListTicketMessagesUseCase {
    pub fn new(uow_manager: Arc<dyn TicketingUnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    pub async fn execute(
        &self,
        command: ListTicketMessagesCommand,
    ) -> Result<Vec<TicketMessage>, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let ticket = uow
            .tickets()
            .find_by_id(command.ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;

        if ticket.tenant_id != command.tenant_id {
            return Err(DomainError::UnauthorizedTenantAccess);
        }

        let messages = uow.messages().find_by_ticket_id(command.ticket_id).await?;
        uow.commit().await?;
        Ok(messages)
    }
}
