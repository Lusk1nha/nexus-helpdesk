use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::domain::error::DomainError;
use crate::domain::ports::{
    CredentialRepository, TenantRepository, UnitOfWork, UnitOfWorkManager, UserRepository,
};

// Importamos os repositórios reais da nossa infraestrutura
use super::{PgCredentialRepository, PgTenantRepository, PgUserRepository};

// ==========================================
// 1. O Gerenciador (Cria as transações)
// ==========================================

pub struct PgUnitOfWorkManager {
    pool: PgPool,
}

impl PgUnitOfWorkManager {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWorkManager for PgUnitOfWorkManager {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, DomainError> {
        let tx = self.pool.begin().await.map_err(|e| {
            DomainError::DatabaseError(format!("Falha ao iniciar transação: {}", e))
        })?;

        let shared_tx = Arc::new(Mutex::new(Some(tx)));

        Ok(Box::new(PgUnitOfWork { tx: shared_tx }))
    }
}

// ==========================================
// 2. O Unit Of Work (Orquestra os repositórios)
// ==========================================

pub struct PgUnitOfWork {
    tx: Arc<Mutex<Option<Transaction<'static, Postgres>>>>,
}

#[async_trait]
impl UnitOfWork for PgUnitOfWork {
    // Injeta a transação compartilhada nos repositórios
    fn users(&mut self) -> Box<dyn UserRepository + '_> {
        Box::new(PgUserRepository::with_transaction(self.tx.clone()))
    }

    fn tenants(&mut self) -> Box<dyn TenantRepository + '_> {
        Box::new(PgTenantRepository::with_transaction(self.tx.clone()))
    }

    fn credentials(&mut self) -> Box<dyn CredentialRepository + '_> {
        Box::new(PgCredentialRepository::with_transaction(self.tx.clone()))
    }

    // Finaliza com sucesso
    async fn commit(self: Box<Self>) -> Result<(), DomainError> {
        let mut guard = self.tx.lock().await;

        if let Some(tx) = guard.take() {
            tx.commit().await.map_err(|e| {
                DomainError::DatabaseError(format!("Erro ao commitar Unit Of Work: {}", e))
            })?;
        }
        Ok(())
    }

    // Aborta as alterações
    async fn rollback(self: Box<Self>) -> Result<(), DomainError> {
        let mut guard = self.tx.lock().await;

        if let Some(tx) = guard.take() {
            tx.rollback().await.map_err(|e| {
                DomainError::DatabaseError(format!("Erro ao fazer rollback do Unit Of Work: {}", e))
            })?;
        }
        Ok(())
    }
}
