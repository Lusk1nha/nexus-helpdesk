use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::User;
use crate::domain::error::DomainError;
use crate::domain::ports::UserRepository;
use shared_kernel::database::DatabaseConnection;

pub struct PgUserRepository {
    conn: DatabaseConnection,
}

impl PgUserRepository {
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
impl UserRepository for PgUserRepository {
    async fn create(&self, user: &User) -> Result<(), DomainError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO users (id, email, full_name, avatar_url, timezone, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            "#,
            user.id,
            user.email,
            user.full_name,
            user.avatar_url,
            user.timezone,
            user.is_active,
            user.created_at,
            user.updated_at
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

    async fn update(&self, user: &User) -> Result<(), DomainError> {
        let query = sqlx::query!(
            r#"
            UPDATE users 
            SET email = $1, full_name = $2, avatar_url = $3, timezone = $4, is_active = $5
            WHERE id = $6
            "#,
            user.email,
            user.full_name,
            user.avatar_url,
            user.timezone,
            user.is_active,
            user.id
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

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError> {
        let query = sqlx::query_as!(
            User,
            r#"SELECT id, email, full_name, avatar_url, timezone, is_active, created_at, updated_at FROM users WHERE email = $1"#,
            email
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError> {
        let query = sqlx::query_as!(
            User,
            r#"SELECT id, email, full_name, avatar_url, timezone, is_active, created_at, updated_at FROM users WHERE id = $1"#,
            id
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
