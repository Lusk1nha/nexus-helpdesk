use async_trait::async_trait;
use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::domain::entities::{Tenant, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::TenantRepository;
use shared_kernel::DatabaseConnection;

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

    async fn find_tenant_user_by_user_id(
        &self,
        user_id: Uuid,
    ) -> Result<Option<TenantUser>, DomainError> {
        // CORREÇÃO: A query deve apontar para tenant_users, não para tenants
        // E precisamos garantir que o campo role seja tratado corretamente
        let query = sqlx::query!(
            r#"
            SELECT tenant_id, user_id, role, is_active, created_at, updated_at
            FROM tenant_users 
            WHERE user_id = $1 AND is_active = true
            LIMIT 1
            "#,
            user_id
        );

        let row = match &self.conn {
            DatabaseConnection::Pool(pool) => query.fetch_optional(pool).await,
            DatabaseConnection::Transaction(tx_mutex) => {
                let mut guard = tx_mutex.lock().await;
                query.fetch_optional(&mut **guard.as_mut().unwrap()).await
            }
        }
        .map_err(|e| DomainError::DatabaseError(e.to_string()))?;

        // Mapeamento manual para garantir que a String do DB vire o Enum Role do Domínio
        match row {
            Some(r) => {
                let relation = TenantUser {
                    tenant_id: r.tenant_id,
                    user_id: r.user_id,
                    // Assume que seu Enum Role implementa FromStr ou TrialFrom<String>
                    // Ou converta manualmente aqui:
                    role: r
                        .role
                        .parse()
                        .map_err(|_| DomainError::DatabaseError("Role inválida no banco".into()))?,
                    is_active: r.is_active,
                    created_at: r.created_at,
                    updated_at: r.updated_at,
                };
                Ok(Some(relation))
            }
            None => Ok(None),
        }
    }

    async fn update_tenant_user(&self, relation: &TenantUser) -> Result<(), DomainError> {
        let role_str = relation.role.to_string();

        let query = sqlx::query!(
            r#"
            UPDATE tenant_users
            SET role = $1, is_active = $2, updated_at = $3
            WHERE tenant_id = $4 AND user_id = $5
            "#,
            role_str,
            relation.is_active,
            relation.updated_at,
            relation.tenant_id,
            relation.user_id,
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

    async fn find_tenant_user(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TenantUser>, DomainError> {
        // No is_active filter here — admin operations need to find deactivated users too.
        let query = sqlx::query!(
            r#"
            SELECT tenant_id, user_id, role, is_active, created_at, updated_at
            FROM tenant_users
            WHERE tenant_id = $1 AND user_id = $2
            LIMIT 1
            "#,
            tenant_id,
            user_id
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
            Some(r) => Ok(Some(TenantUser {
                tenant_id: r.tenant_id,
                user_id: r.user_id,
                role: r
                    .role
                    .parse()
                    .map_err(|_| DomainError::DatabaseError("Role inválida no banco".into()))?,
                is_active: r.is_active,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })),
            None => Ok(None),
        }
    }

    async fn list_members(&self, tenant_id: Uuid) -> Result<Vec<(User, TenantUser)>, DomainError> {
        let query = sqlx::query!(
            r#"
            SELECT
                u.id        AS user_id,
                u.email,
                u.full_name,
                u.avatar_url,
                u.timezone,
                u.is_active AS user_is_active,
                u.created_at AS user_created_at,
                u.updated_at AS user_updated_at,
                tu.role,
                tu.is_active AS tu_is_active,
                tu.created_at AS tu_created_at,
                tu.updated_at AS tu_updated_at
            FROM users u
            JOIN tenant_users tu ON tu.user_id = u.id
            WHERE tu.tenant_id = $1 AND tu.is_active = true
            ORDER BY tu.created_at ASC
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

        let mut result = Vec::new();
        for r in rows {
            let user = User {
                id: r.user_id,
                email: r.email,
                full_name: r.full_name,
                avatar_url: r.avatar_url,
                timezone: r.timezone,
                is_active: r.user_is_active,
                created_at: r.user_created_at,
                updated_at: r.user_updated_at,
            };

            let tenant_user = TenantUser {
                tenant_id,
                user_id: r.user_id,
                role: r
                    .role
                    .parse()
                    .map_err(|_| DomainError::DatabaseError("Role inválida no banco".into()))?,
                is_active: r.tu_is_active,
                created_at: r.tu_created_at,
                updated_at: r.tu_updated_at,
            };

            result.push((user, tenant_user));
        }

        Ok(result)
    }
}
