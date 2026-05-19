use std::sync::Arc;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::domain::entities::message::TicketMessage;
use crate::domain::error::DomainError;
use crate::domain::ports::TicketingUnitOfWorkManager;

#[derive(Debug, Clone)]
pub struct AiTask {
    pub ticket_id: Uuid,
    pub tenant_id: Uuid,
    pub context: String,
}

pub struct AiWorker {
    receiver: Receiver<AiTask>,
    http_client: reqwest::Client,
    uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
    ollama_url: String,
}

impl AiWorker {
    pub fn new(
        receiver: Receiver<AiTask>,
        uow_manager: Arc<dyn TicketingUnitOfWorkManager>,
        ollama_url: String,
    ) -> Self {
        Self {
            receiver,
            http_client: reqwest::Client::new(),
            uow_manager,
            ollama_url,
        }
    }

    pub async fn start(mut self) {
        info!("🤖 AI Worker iniciado. Aguardando tickets na fila...");

        while let Some(task) = self.receiver.recv().await {
            info!("📥 Ticket recebido pelo AI Worker: {}", task.ticket_id);
            self.process_ticket(task).await;
        }

        info!("🛑 AI Worker desligado (Canal foi fechado).");
    }

    async fn process_ticket(&self, task: AiTask) {
        // 1. Mark the ticket as being processed by AI
        if let Err(e) = self.transition_to_processing(task.ticket_id).await {
            error!(
                "Falha ao marcar ticket {} como processing_ai: {}",
                task.ticket_id, e
            );
            return;
        }

        // 2. Call the LLM
        let system_prompt = "Você é um agente de suporte ao cliente nível 1. \
            Analise o problema relatado e forneça uma resposta educada, técnica e direta. \
            Não invente links ou prometa coisas que não pode cumprir.";

        let payload = serde_json::json!({
            "model": "phi3",
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": &task.context }
            ],
            "stream": false
        });

        let ollama_result = self
            .http_client
            .post(format!("{}/api/chat", self.ollama_url))
            .json(&payload)
            .send()
            .await;

        match ollama_result {
            Ok(res) if res.status().is_success() => {
                match res.json::<serde_json::Value>().await {
                    Ok(body) => {
                        let ai_reply = body["message"]["content"]
                            .as_str()
                            .unwrap_or("Sem resposta.")
                            .to_string();

                        info!(
                            "✅ Resposta da IA gerada para o ticket {}.",
                            task.ticket_id
                        );

                        // 3a. Persist the AI message + transition to AwaitingAgentApproval
                        if let Err(e) =
                            self.persist_ai_response(task.ticket_id, ai_reply).await
                        {
                            error!(
                                "Falha ao persistir resposta da IA para o ticket {}: {}",
                                task.ticket_id, e
                            );
                            let _ = self.revert_to_open(task.ticket_id).await;
                        }
                    }
                    Err(e) => {
                        error!(
                            "Falha ao deserializar resposta do Ollama para o ticket {}: {}",
                            task.ticket_id, e
                        );
                        let _ = self.revert_to_open(task.ticket_id).await;
                    }
                }
            }
            Ok(res) => {
                error!(
                    "Ollama retornou HTTP {} para o ticket {}.",
                    res.status(),
                    task.ticket_id
                );
                let _ = self.revert_to_open(task.ticket_id).await;
            }
            Err(e) => {
                warn!(
                    "Conexão com Ollama falhou para o ticket {} (ele está rodando?): {}",
                    task.ticket_id, e
                );
                let _ = self.revert_to_open(task.ticket_id).await;
            }
        }
    }

    async fn transition_to_processing(&self, ticket_id: Uuid) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;
        let mut ticket = uow
            .tickets()
            .find_by_id(ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;
        ticket.mark_as_processing();
        uow.tickets().update(&ticket).await?;
        uow.commit().await
    }

    async fn persist_ai_response(
        &self,
        ticket_id: Uuid,
        reply: String,
    ) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;
        let mut ticket = uow
            .tickets()
            .find_by_id(ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;

        let message = TicketMessage::new_ai_response(ticket_id, reply);
        uow.messages().add_message(&message).await?;

        ticket.await_human_approval();
        uow.tickets().update(&ticket).await?;
        uow.commit().await
    }

    async fn revert_to_open(&self, ticket_id: Uuid) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;
        let mut ticket = uow
            .tickets()
            .find_by_id(ticket_id)
            .await?
            .ok_or(DomainError::TicketNotFound)?;
        ticket.revert_to_open();
        uow.tickets().update(&ticket).await?;
        uow.commit().await
    }
}
