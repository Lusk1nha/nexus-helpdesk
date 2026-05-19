use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::application::workers::AiTask;
use crate::domain::entities::message::{SenderType, TicketMessage};
use crate::domain::entities::ticket::Ticket;
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

pub struct CreateTicketCommand {
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub title: String,
    pub description: String,
}

pub struct CreateTicketUseCase {
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
    ai_queue_sender: Sender<AiTask>,
}

impl CreateTicketUseCase {
    pub fn new(
        uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
        ai_queue_sender: Sender<AiTask>,
    ) -> Self {
        Self {
            uow_manager,
            ai_queue_sender,
        }
    }

    #[tracing::instrument(
        name = "create_ticket",
        skip(self, command),
        fields(tenant_id = %command.tenant_id, customer_id = %command.customer_id)
    )]
    pub async fn execute(&self, command: CreateTicketCommand) -> Result<Ticket, DomainError> {
        if command.description.trim().is_empty() {
            return Err(DomainError::EmptyMessageContent);
        }

        let ticket = Ticket::new(
            command.tenant_id,
            command.customer_id,
            command.title.clone(),
            command.description.clone(),
        );

        let message = TicketMessage::new_human(
            ticket.id,
            command.customer_id,
            SenderType::Customer,
            command.description.clone(),
        );

        let mut uow = self.uow_manager.begin().await?;
        uow.tickets().create(&ticket).await?;
        uow.messages().add_message(&message).await?;
        uow.commit().await?;

        let task = AiTask {
            ticket_id: ticket.id,
            tenant_id: ticket.tenant_id,
            context: command.description,
        };

        if let Err(e) = self.ai_queue_sender.send(task).await {
            tracing::error!(
                ticket_id = %ticket.id,
                error = %e,
                "failed to enqueue ticket for AI processing"
            );
        } else {
            tracing::info!(ticket_id = %ticket.id, "ticket created and queued for AI");
        }

        Ok(ticket)
    }
}
