use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TicketStatus {
    Open,                  // Cliente acabou de abrir
    ProcessingAI,          // O Worker do Tokio pegou da fila e está gerando resposta
    AwaitingAgentApproval, // A IA respondeu, mas um humano precisa aprovar (Human-in-the-loop)
    Resolved,              // Problema resolvido
    Closed,                // Encerrado
}

impl ToString for TicketStatus {
    fn to_string(&self) -> String {
        match self {
            TicketStatus::Open => "open".to_string(),
            TicketStatus::ProcessingAI => "processing_ai".to_string(),
            TicketStatus::AwaitingAgentApproval => "awaiting_agent_approval".to_string(),
            TicketStatus::Resolved => "resolved".to_string(),
            TicketStatus::Closed => "closed".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Ticket {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid, // Quem abriu
    pub title: String,
    pub description: String,
    pub status: TicketStatus,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Ticket {
    pub fn new(tenant_id: Uuid, customer_id: Uuid, title: String, description: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            tenant_id,
            customer_id,
            title,
            description,
            status: TicketStatus::Open, // Sempre nasce aberto
            created_at: now,
            updated_at: now,
        }
    }

    // A máquina de estados protegida
    pub fn mark_as_processing(&mut self) {
        self.status = TicketStatus::ProcessingAI;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn await_human_approval(&mut self) {
        self.status = TicketStatus::AwaitingAgentApproval;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn revert_to_open(&mut self) {
        self.status = TicketStatus::Open;
        self.updated_at = OffsetDateTime::now_utc();
    }
}
