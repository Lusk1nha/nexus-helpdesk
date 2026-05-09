use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::Credential;
use crate::domain::error::DomainError;
use crate::domain::ports::CredentialRepository;
use crate::infrastructure::database::DatabaseConnection;

pub struct PgCredentialRepository {
    conn: DatabaseConnection,
}

impl PgCredentialRepository {
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
impl CredentialRepository for PgCredentialRepository {
    async fn create(&self, credential: &Credential) -> Result<(), DomainError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO credentials (user_id, password_hash, failed_attempts, last_login_at, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            credential.user_id,
            credential.password_hash,
            credential.failed_attempts,
            credential.last_login_at,
            credential.created_at,
            credential.updated_at,
        );

        match &self.conn {
            DatabaseConnection::Pool(pool) => query.execute(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                let tx = guard.as_mut().expect("Transação finalizada inesperadamente");
                query.execute(&mut **tx).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn update(&self, credential: &Credential) -> Result<(), DomainError> {
        // Deixamos o updated_at para a Trigger do Postgres
        let query = sqlx::query!(
            r#"
            UPDATE credentials 
            SET password_hash = $1, failed_attempts = $2, last_login_at = $3
            WHERE user_id = $4
            "#,
            credential.password_hash,
            credential.failed_attempts,
            credential.last_login_at,
            credential.user_id,
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

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<Credential>, DomainError> {
        let query = sqlx::query_as!(
            Credential,
            r#"
            SELECT user_id, password_hash, failed_attempts, last_login_at, created_at, updated_at 
            FROM credentials 
            WHERE user_id = $1
            "#,
            user_id
        );

        let result = match &self.conn {
            DatabaseConnection::Pool(pool) => query.fetch_optional(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                query.fetch_optional(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(result)
    }
}
