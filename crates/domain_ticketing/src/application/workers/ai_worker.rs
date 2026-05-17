use serde_json::json;
use tokio::sync::mpsc::Receiver;
use tracing::{error, info};
use uuid::Uuid;

// A mesma task que o CreateTicketUseCase envia para a fila
#[derive(Debug, Clone)]
pub struct AiTask {
    pub ticket_id: Uuid,
    pub tenant_id: Uuid,
    pub context: String, // A descrição do chamado do cliente
}

pub struct AiWorker {
    receiver: Receiver<AiTask>,
    http_client: reqwest::Client,
    // Futuramente: Você pode injetar o TicketRepository aqui
    // para mudar o status de 'ProcessingAI' para 'AwaitingAgentApproval'
}

impl AiWorker {
    pub fn new(receiver: Receiver<AiTask>) -> Self {
        Self {
            receiver,
            http_client: reqwest::Client::new(),
        }
    }

    /// O Loop Infinito que roda em background
    pub async fn start(mut self) {
        info!("🤖 AI Worker iniciado. Aguardando tickets na fila...");

        // Fica bloqueado aqui (sem gastar CPU) até chegar uma mensagem no canal
        while let Some(task) = self.receiver.recv().await {
            info!("📥 Ticket recebido pelo AI Worker: {}", task.ticket_id);

            // Usamos o 'tokio::spawn' novamente dentro do worker se quisermos
            // processar múltiplos chamados simultaneamente no Ollama (cuidado com a VRAM da GPU!)
            // Por enquanto, faremos sequencial para não estourar o Ollama local.
            self.process_ticket(task).await;
        }

        info!("🛑 AI Worker desligado (Canal foi fechado).");
    }

    async fn process_ticket(&self, task: AiTask) {
        info!("🧠 Consultando LLM para o ticket: {}...", task.ticket_id);

        // Prompt de Sistema rigoroso (System Prompt)
        let system_prompt = "Você é um agente de suporte ao cliente nível 1. \
            Analise o problema relatado e forneça uma resposta educada, técnica e direta. \
            Não invente links ou prometa coisas que não pode cumprir.";

        // Montando o Payload para o Ollama (ex: usando o modelo llama3 ou phi3)
        let ollama_payload = json!({
            "model": "phi3", // Substitua pelo modelo que você rodou no 'ollama run'
            "messages": [
                { "role": "system", "content": system_prompt },
                { "role": "user", "content": task.context }
            ],
            "stream": false // Para simplificar, pegamos a resposta inteira de uma vez
        });

        // Disparo HTTP para o Ollama local
        // Assumindo porta padrão 11434 do Ollama
        let response = self
            .http_client
            .post("http://127.0.0.1:11434/api/chat")
            .json(&ollama_payload)
            .send()
            .await;

        match response {
            Ok(res) if res.status().is_success() => {
                if let Ok(body) = res.json::<serde_json::Value>().await {
                    let ai_reply = body["message"]["content"]
                        .as_str()
                        .unwrap_or("Sem resposta.");
                    info!(
                        "✅ Resposta da IA gerada com sucesso para o ticket {}!",
                        task.ticket_id
                    );

                    // AQUI NO FUTURO:
                    // 1. Salvar `ai_reply` no `PgMessageRepository` como SenderType::AI
                    // 2. Mudar status do Ticket para `AwaitingAgentApproval`

                    // Apenas para debug temporário:
                    println!(
                        "--- SUGESTÃO DA IA ---\n{}\n----------------------",
                        ai_reply
                    );
                }
            }
            Ok(res) => {
                error!(
                    "❌ Ollama retornou erro HTTP {}: {:?}",
                    res.status(),
                    res.text().await
                );
            }
            Err(e) => {
                error!(
                    "❌ Falha na conexão com o Ollama local. Ele está rodando? Erro: {}",
                    e
                );
            }
        }
    }
}
