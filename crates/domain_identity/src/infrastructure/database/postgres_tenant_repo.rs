use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::{Tenant, TenantUser};
use crate::domain::error::DomainError;
use crate::domain::ports::TenantRepository;
use crate::infrastructure::database::DatabaseConnection;

pub struct PgTenantRepository {
    conn: DatabaseConnection,
}

impl PgTenantRepository {
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
impl TenantRepository for PgTenantRepository {
    async fn create(&self, tenant: &Tenant) -> Result<(), DomainError> {
        let query = sqlx::query!(
            r#"
            INSERT INTO tenants (id, name, slug, plan, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
            tenant.id,
            tenant.name,
            tenant.slug,
            tenant.plan,
            tenant.is_active,
            tenant.created_at,
            tenant.updated_at,
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

    async fn update(&self, tenant: &Tenant) -> Result<(), DomainError> {
        let query = sqlx::query!(
            r#"
            UPDATE tenants 
            SET name = $1, slug = $2, plan = $3, is_active = $4
            WHERE id = $5
            "#,
            tenant.name,
            tenant.slug,
            tenant.plan,
            tenant.is_active,
            tenant.id,
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

    async fn add_user_to_tenant(&self, relation: &TenantUser) -> Result<(), DomainError> {
        let role_str = relation.role.to_string();

        let query = sqlx::query!(
            r#"
            INSERT INTO tenant_users (tenant_id, user_id, role, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            relation.tenant_id,
            relation.user_id,
            role_str,
            relation.is_active,
            relation.created_at,
            relation.updated_at,
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

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, DomainError> {
        let query = sqlx::query_as!(
            Tenant,
            r#"
            SELECT id, name, slug, plan, is_active, created_at, updated_at 
            FROM tenants 
            WHERE id = $1
            "#,
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
