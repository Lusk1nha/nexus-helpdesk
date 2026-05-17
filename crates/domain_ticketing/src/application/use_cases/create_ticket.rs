use std::sync::Arc;
use tokio::sync::mpsc::Sender;
use uuid::Uuid;

use crate::application::workers::AiTask;

use crate::domain::entities::message::{SenderType, TicketMessage};
use crate::domain::entities::ticket::Ticket;
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

// 1. O Comando de Entrada do Use Case
pub struct CreateTicketCommand {
    pub tenant_id: Uuid,
    pub customer_id: Uuid,

    pub title: String,
    pub description: String,
}

// 2. O Use Case
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

    pub async fn execute(&self, command: CreateTicketCommand) -> Result<Ticket, DomainError> {
        // Validação básica
        if command.description.trim().is_empty() {
            return Err(DomainError::EmptyMessageContent);
        }

        // Cria as entidades
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

        // ==========================================
        // Transação de Banco de Dados (Síncrono/ACID)
        // ==========================================
        let mut uow = self.uow_manager.begin().await?;

        uow.tickets().create(&ticket).await?;
        uow.messages().add_message(&message).await?;

        uow.commit().await?;

        // ==========================================
        // Despacho Assíncrono para o Worker de IA
        // ==========================================
        let task = AiTask {
            ticket_id: ticket.id,
            tenant_id: ticket.tenant_id,
            context: command.description,
        };

        // Envia para o canal na RAM. Isso não bloqueia a execução!
        // Se a fila estiver cheia (dependendo de como criarmos o canal), ele espera.
        if let Err(e) = self.ai_queue_sender.send(task).await {
            // Em um sistema real, você poderia ter uma "Dead Letter Queue" ou logar severamente
            tracing::error!(
                "ALERTA CRÍTICO: Falha ao enviar ticket {} para a fila da IA. Motor sobrecarregado ou desligado? Erro: {:?}",
                ticket.id,
                e
            );
        } else {
            tracing::info!(
                "Ticket {} enfileirado para processamento da IA com sucesso.",
                ticket.id
            );
        }

        Ok(ticket)
    }
}
