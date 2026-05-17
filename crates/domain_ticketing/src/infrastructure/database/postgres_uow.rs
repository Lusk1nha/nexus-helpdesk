use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::error::DomainError;
use crate::domain::ports::{
    MessageRepository, TicketRepository, TicketingUnitOfWork, TicketingUnitOfWorkManager,
};

// Importamos os repositórios reais
use super::{postgres_message_repo::PgMessageRepository, postgres_ticket_repo::PgTicketRepository};

pub struct PgTicketingUoWManager {
    pool: PgPool,
}

impl PgTicketingUoWManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TicketingUnitOfWorkManager for PgTicketingUoWManager {
    async fn begin(&self) -> Result<Box<dyn TicketingUnitOfWork>, DomainError> {
        let tx = self.pool.begin().await.map_err(|e| {
            DomainError::DatabaseError(format!("Falha ao iniciar transação de ticketing: {}", e))
        })?;

        let shared_tx = Arc::new(Mutex::new(Some(tx)));

        Ok(Box::new(PgTicketingUoW { tx: shared_tx }))
    }
}

pub struct PgTicketingUoW {
    tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl TicketingUnitOfWork for PgTicketingUoW {
    fn tickets(&mut self) -> Box<dyn TicketRepository + '_> {
        Box::new(PgTicketRepository::with_transaction(self.tx.clone()))
    }

    fn messages(&mut self) -> Box<dyn MessageRepository + '_> {
        Box::new(PgMessageRepository::with_transaction(self.tx.clone()))
    }

    async fn commit(self: Box<Self>) -> Result<(), DomainError> {
        let mut guard = self.tx.lock().await;
        if let Some(tx) = guard.take() {
            tx.commit().await.map_err(|e| {
                DomainError::DatabaseError(format!("Erro ao commitar transação: {}", e))
            })?;
        }
        Ok(())
    }

    async fn rollback(self: Box<Self>) -> Result<(), DomainError> {
        let mut guard = self.tx.lock().await;
        if let Some(tx) = guard.take() {
            tx.rollback().await.map_err(|e| {
                DomainError::DatabaseError(format!("Erro ao fazer rollback: {}", e))
            })?;
        }
        Ok(())
    }
}
