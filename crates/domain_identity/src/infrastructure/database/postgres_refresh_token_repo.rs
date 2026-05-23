use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::RefreshToken;
use crate::domain::error::DomainError;
use crate::domain::ports::RefreshTokenRepository;
use shared_kernel::DatabaseConnection;

pub struct PgRefreshTokenRepository {
    conn: DatabaseConnection,
}

impl PgRefreshTokenRepository {
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
impl RefreshTokenRepository for PgRefreshTokenRepository {
    async fn create(&self, token: &RefreshToken) -> Result<(), DomainError> {
        let q = sqlx::query(
            r#"
            INSERT INTO refresh_tokens (jti, user_id, tenant_id, token_hash, expires_at, revoked_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(token.jti)
        .bind(token.user_id)
        .bind(token.tenant_id)
        .bind(&token.token_hash)
        .bind(token.expires_at)
        .bind(token.revoked_at)
        .bind(token.created_at);

        match &self.conn {
            DatabaseConnection::Pool(pool) => q.execute(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                q.execute(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_jti(&self, jti: Uuid) -> Result<Option<RefreshToken>, DomainError> {
        let q = sqlx::query_as::<_, RefreshTokenRow>(
            r#"
            SELECT jti, user_id, tenant_id, token_hash, expires_at, revoked_at, created_at
            FROM refresh_tokens
            WHERE jti = $1
            "#,
        )
        .bind(jti);

        let row = match &self.conn {
            DatabaseConnection::Pool(pool) => q.fetch_optional(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                q.fetch_optional(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(row.map(Into::into))
    }

    async fn revoke(&self, jti: Uuid) -> Result<(), DomainError> {
        let q = sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE jti = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(jti);

        match &self.conn {
            DatabaseConnection::Pool(pool) => q.execute(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                q.execute(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), DomainError> {
        let q = sqlx::query(
            r#"
            UPDATE refresh_tokens
            SET revoked_at = NOW()
            WHERE user_id = $1 AND revoked_at IS NULL
            "#,
        )
        .bind(user_id);

        match &self.conn {
            DatabaseConnection::Pool(pool) => q.execute(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                q.execute(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

#[derive(sqlx::FromRow)]
struct RefreshTokenRow {
    jti: Uuid,
    user_id: Uuid,
    tenant_id: Uuid,
    token_hash: String,
    expires_at: OffsetDateTime,
    revoked_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
}

impl From<RefreshTokenRow> for RefreshToken {
    fn from(r: RefreshTokenRow) -> Self {
        Self {
            jti: r.jti,
            user_id: r.user_id,
            tenant_id: r.tenant_id,
            token_hash: r.token_hash,
            expires_at: r.expires_at,
            revoked_at: r.revoked_at,
            created_at: r.created_at,
        }
    }
}
