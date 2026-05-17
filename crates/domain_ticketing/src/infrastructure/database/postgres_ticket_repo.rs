use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::ticket::{Ticket, TicketStatus};
use crate::domain::error::DomainError;
use crate::domain::ports::TicketRepository;

use shared_kernel::DatabaseConnection;

pub struct PgTicketRepository {
    conn: DatabaseConnection,
}

impl PgTicketRepository {
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
impl TicketRepository for PgTicketRepository {
    async fn create(&self, ticket: &Ticket) -> Result<(), DomainError> {
        let status_str = ticket.status.to_string();

        let query = sqlx::query!(
            r#"
            INSERT INTO tickets (id, tenant_id, customer_id, title, description, status, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            ticket.id,
            ticket.tenant_id,
            ticket.customer_id,
            ticket.title,
            ticket.description,
            status_str,
            ticket.created_at,
            ticket.updated_at
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

    async fn update(&self, ticket: &Ticket) -> Result<(), DomainError> {
        let status_str = ticket.status.to_string();

        let query = sqlx::query!(
            r#"
            UPDATE tickets 
            SET title = $1, description = $2, status = $3, updated_at = $4
            WHERE id = $5
            "#,
            ticket.title,
            ticket.description,
            status_str,
            ticket.updated_at,
            ticket.id
        );

        match &self.conn {
            DatabaseConnection::Pool(pool) => query.execute(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                query.execute(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Ticket>, DomainError> {
        let query = sqlx::query!(
            r#"
            SELECT id, tenant_id, customer_id, title, description, status, created_at, updated_at 
            FROM tickets 
            WHERE id = $1
            "#,
            id
        );

        let row = match &self.conn {
            DatabaseConnection::Pool(pool) => query.fetch_optional(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                query.fetch_optional(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        match row {
            Some(r) => {
                let status = match r.status.as_str() {
                    "open" => TicketStatus::Open,
                    "processing_ai" => TicketStatus::ProcessingAI,
                    "awaiting_agent_approval" => TicketStatus::AwaitingAgentApproval,
                    "resolved" => TicketStatus::Resolved,
                    "closed" => TicketStatus::Closed,
                    _ => {
                        return Err(DomainError::DatabaseError(
                            "Status de ticket inválido.".into(),
                        ));
                    }
                };

                Ok(Some(Ticket {
                    id: r.id,
                    tenant_id: r.tenant_id,
                    customer_id: r.customer_id,
                    title: r.title,
                    description: r.description,
                    status,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                }))
            }
            None => Ok(None),
        }
    }

    async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<Ticket>, DomainError> {
        let query = sqlx::query!(
            r#"
            SELECT id, tenant_id, customer_id, title, description, status, created_at, updated_at 
            FROM tickets 
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
            tenant_id
        );

        let rows = match &self.conn {
            DatabaseConnection::Pool(pool) => query.fetch_all(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                query.fetch_all(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        let mut tickets = Vec::new();
        for r in rows {
            let status = match r.status.as_str() {
                "open" => TicketStatus::Open,
                "processing_ai" => TicketStatus::ProcessingAI,
                "awaiting_agent_approval" => TicketStatus::AwaitingAgentApproval,
                "resolved" => TicketStatus::Resolved,
                "closed" => TicketStatus::Closed,
                _ => continue,
            };

            tickets.push(Ticket {
                id: r.id,
                tenant_id: r.tenant_id,
                customer_id: r.customer_id,
                title: r.title,
                description: r.description,
                status,
                created_at: r.created_at,
                updated_at: r.updated_at,
            });
        }

        Ok(tickets)
    }
}
