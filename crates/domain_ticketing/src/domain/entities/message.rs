use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SenderType {
    Customer,
    Agent,
    AI,
    System,
}

impl ToString for SenderType {
    fn to_string(&self) -> String {
        match self {
            SenderType::Customer => "customer".to_string(),
            SenderType::Agent => "agent".to_string(),
            SenderType::AI => "ai".to_string(),
            SenderType::System => "system".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct TicketMessage {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_id: Option<Uuid>,
    pub sender_type: SenderType,
    pub content: String,
    pub is_internal_note: bool,
    pub created_at: OffsetDateTime,
}

impl TicketMessage {
    // Construtor para uma mensagem normal de um usuário humano
    pub fn new_human(
        ticket_id: Uuid,
        sender_id: Uuid,
        sender_type: SenderType,
        content: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            ticket_id,
            sender_id: Some(sender_id),
            sender_type,
            content,
            is_internal_note: false,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    // Construtor específico para a IA, que não tem um "user_id" associado
    pub fn new_ai_response(ticket_id: Uuid, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            ticket_id,
            sender_id: None,
            sender_type: SenderType::AI,
            content,
            is_internal_note: false, // A resposta da IA geralmente vai para o cliente, ou vira nota interna dependendo da sua regra de aprovação
            created_at: OffsetDateTime::now_utc(),
        }
    }
}
