use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use domain_ticketing::domain::entities::ticket::Ticket;

#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateTicketPayload {
    #[schema(example = "Sistema não abre")]
    pub title: String,

    #[schema(example = "Tento logar e recebo erro 500")]
    pub description: String,
}

#[derive(Serialize, ToSchema)]
pub struct CreateTicketResponse {
    #[schema(value_type = String)]
    pub ticket_id: Uuid,
    pub status: String,
    pub message: String,
}

impl From<Ticket> for CreateTicketResponse {
    fn from(ticket: Ticket) -> Self {
        Self {
            ticket_id: ticket.id,
            status: ticket.status.to_string(),
            message: "Ticket criado com sucesso e encaminhado para análise da IA.".to_string(),
        }
    }
}
