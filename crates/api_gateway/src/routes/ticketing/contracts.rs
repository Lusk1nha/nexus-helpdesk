use domain_ticketing::domain::entities::message::TicketMessage;
use domain_ticketing::domain::entities::ticket::Ticket;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

// ─── Create ──────────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketPayload {
    #[validate(length(min = 3, message = "O título deve ter no mínimo 3 caracteres."))]
    #[schema(example = "Impressora não liga")]
    pub title: String,

    #[validate(length(min = 1, message = "A descrição não pode estar vazia."))]
    #[schema(example = "Minha impressora parou de funcionar depois da última atualização.")]
    pub description: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateTicketResponse {
    pub ticket_id: Uuid,
    #[schema(example = "open")]
    pub status: String,
    #[schema(example = "Ticket criado com sucesso e encaminhado para análise da IA.")]
    pub message: String,
}

impl From<Ticket> for CreateTicketResponse {
    fn from(t: Ticket) -> Self {
        Self {
            ticket_id: t.id,
            status: t.status.to_string(),
            message: "Ticket criado com sucesso e encaminhado para análise da IA.".to_string(),
        }
    }
}

// ─── Get / List ───────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TicketResponse {
    pub id: Uuid,
    pub tenant_id: Uuid,
    pub customer_id: Uuid,
    pub title: String,
    pub description: String,
    #[schema(example = "open")]
    pub status: String,
    #[schema(value_type = String)]
    pub created_at: OffsetDateTime,
    #[schema(value_type = String)]
    pub updated_at: OffsetDateTime,
}

impl From<Ticket> for TicketResponse {
    fn from(t: Ticket) -> Self {
        Self {
            id: t.id,
            tenant_id: t.tenant_id,
            customer_id: t.customer_id,
            title: t.title,
            description: t.description,
            status: t.status.to_string(),
            created_at: t.created_at,
            updated_at: t.updated_at,
        }
    }
}

// ─── Update status ────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTicketStatusPayload {
    #[validate(length(min = 1, message = "O status não pode estar vazio."))]
    #[schema(example = "resolved")]
    pub status: String,
}

// ─── Messages ─────────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct MessageResponse {
    pub id: Uuid,
    pub ticket_id: Uuid,
    pub sender_id: Option<Uuid>,
    #[schema(example = "customer")]
    pub sender_type: String,
    pub content: String,
    pub is_internal_note: bool,
    #[schema(value_type = String)]
    pub created_at: OffsetDateTime,
}

impl From<TicketMessage> for MessageResponse {
    fn from(m: TicketMessage) -> Self {
        Self {
            id: m.id,
            ticket_id: m.ticket_id,
            sender_id: m.sender_id,
            sender_type: m.sender_type.to_string(),
            content: m.content,
            is_internal_note: m.is_internal_note,
            created_at: m.created_at,
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AddMessagePayload {
    #[validate(length(min = 1, message = "A mensagem não pode estar vazia."))]
    #[schema(example = "Obrigado! O problema foi resolvido após reiniciar o serviço.")]
    pub content: String,
}
