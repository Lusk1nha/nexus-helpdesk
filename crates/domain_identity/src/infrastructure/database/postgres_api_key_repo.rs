use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::str::FromStr;
use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::{ApiKey, Role};
use crate::domain::error::DomainError;
use crate::domain::ports::ApiKeyRepository;
use shared_kernel::DatabaseConnection;

pub struct PgApiKeyRepository {
    conn: DatabaseConnection,
}

impl PgApiKeyRepository {
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
impl ApiKeyRepository for PgApiKeyRepository {
    async fn create(&self, api_key: &ApiKey) -> Result<(), DomainError> {
        let q = sqlx::query(
            r#"
            INSERT INTO api_keys
                (id, tenant_id, name, key_prefix, key_hash, role,
                 created_by, expires_at, revoked_at, last_used_at, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(api_key.id)
        .bind(api_key.tenant_id)
        .bind(&api_key.name)
        .bind(&api_key.key_prefix)
        .bind(&api_key.key_hash)
        .bind(api_key.role.to_string())
        .bind(api_key.created_by)
        .bind(api_key.expires_at)
        .bind(api_key.revoked_at)
        .bind(api_key.last_used_at)
        .bind(api_key.created_at);

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

    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<ApiKey>, DomainError> {
        let q = sqlx::query_as::<_, ApiKeyRow>(
            r#"
            SELECT id, tenant_id, name, key_prefix, key_hash, role,
                   created_by, expires_at, revoked_at, last_used_at, created_at
            FROM api_keys
            WHERE key_hash = $1
            "#,
        )
        .bind(key_hash);

        let row = match &self.conn {
            DatabaseConnection::Pool(pool) => q.fetch_optional(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                q.fetch_optional(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        row.map(TryInto::try_into).transpose()
    }

    async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ApiKey>, DomainError> {
        let q = sqlx::query_as::<_, ApiKeyRow>(
            r#"
            SELECT id, tenant_id, name, key_prefix, key_hash, role,
                   created_by, expires_at, revoked_at, last_used_at, created_at
            FROM api_keys
            WHERE tenant_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(tenant_id);

        let rows = match &self.conn {
            DatabaseConnection::Pool(pool) => q.fetch_all(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                q.fetch_all(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        rows.into_iter().map(TryInto::try_into).collect()
    }

    async fn revoke(&self, id: Uuid, tenant_id: Uuid) -> Result<(), DomainError> {
        let q = sqlx::query(
            r#"
            UPDATE api_keys
            SET revoked_at = NOW()
            WHERE id = $1 AND tenant_id = $2 AND revoked_at IS NULL
            "#,
        )
        .bind(id)
        .bind(tenant_id);

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

    async fn touch_last_used(&self, id: Uuid) -> Result<(), DomainError> {
        let q = sqlx::query(
            r#"
            UPDATE api_keys
            SET last_used_at = NOW()
            WHERE id = $1
            "#,
        )
        .bind(id);

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
struct ApiKeyRow {
    id: Uuid,
    tenant_id: Uuid,
    name: String,
    key_prefix: String,
    key_hash: String,
    role: String,
    created_by: Option<Uuid>,
    expires_at: Option<OffsetDateTime>,
    revoked_at: Option<OffsetDateTime>,
    last_used_at: Option<OffsetDateTime>,
    created_at: OffsetDateTime,
}

impl TryFrom<ApiKeyRow> for ApiKey {
    type Error = DomainError;

    fn try_from(r: ApiKeyRow) -> Result<Self, Self::Error> {
        Ok(Self {
            id: r.id,
            tenant_id: r.tenant_id,
            name: r.name,
            key_prefix: r.key_prefix,
            key_hash: r.key_hash,
            role: Role::from_str(&r.role)?,
            created_by: r.created_by,
            expires_at: r.expires_at,
            revoked_at: r.revoked_at,
            last_used_at: r.last_used_at,
            created_at: r.created_at,
        })
    }
}
