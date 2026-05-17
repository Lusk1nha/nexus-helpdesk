use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::message::{SenderType, TicketMessage};
use crate::domain::error::DomainError;
use crate::domain::ports::MessageRepository;

use shared_kernel::DatabaseConnection;

pub struct PgMessageRepository {
    conn: DatabaseConnection,
}

impl PgMessageRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            conn: DatabaseConnection::Pool(pool),
        }
    }

    pub fn with_transaction(tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>) -> Self {
        Self {
            conn: DatabaseConnection::Transaction(tx),
        }
    }
}

#[async_trait]
impl MessageRepository for PgMessageRepository {
    async fn add_message(&self, message: &TicketMessage) -> Result<(), DomainError> {
        let sender_type_str = message.sender_type.to_string();

        let query = sqlx::query!(
            r#"
            INSERT INTO ticket_messages (id, ticket_id, sender_id, sender_type, content, is_internal_note, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            message.id,
            message.ticket_id,
            message.sender_id,
            sender_type_str,
            message.content,
            message.is_internal_note,
            message.created_at
        );

        match &self.conn {
            DatabaseConnection::Pool(pool) => query.execute(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                let tx = guard.as_mut().expect("Transação já foi finalizada!");
                query.execute(&mut **tx).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_ticket_id(&self, ticket_id: Uuid) -> Result<Vec<TicketMessage>, DomainError> {
        let query = sqlx::query!(
            r#"
            SELECT id, ticket_id, sender_id, sender_type, content, is_internal_note, created_at 
            FROM ticket_messages 
            WHERE ticket_id = $1
            ORDER BY created_at ASC
            "#,
            ticket_id
        );

        let rows = match &self.conn {
            DatabaseConnection::Pool(pool) => query.fetch_all(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                query.fetch_all(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let mut messages = Vec::new();
        for r in rows {
            let sender_type = match r.sender_type.as_str() {
                "customer" => SenderType::Customer,
                "agent" => SenderType::Agent,
                "ai" => SenderType::AI,
                "system" => SenderType::System,
                _ => {
                    tracing::warn!(
                        "Tipo de sender inválido encontrado no banco: {}",
                        r.sender_type
                    );
                    continue;
                }
            };

            messages.push(TicketMessage {
                id: r.id,
                ticket_id: r.ticket_id,
                sender_id: r.sender_id,
                sender_type,
                content: r.content,
                is_internal_note: r.is_internal_note,
                created_at: r.created_at,
            });
        }

        Ok(messages)
    }
}
