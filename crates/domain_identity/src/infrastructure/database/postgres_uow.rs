use async_trait::async_trait;
use sqlx::PgPool;

use crate::domain::entities::{Credential, Tenant, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::IdentityUnitOfWork;

pub struct PgIdentityUnitOfWork {
    pool: PgPool,
}

impl PgIdentityUnitOfWork {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl IdentityUnitOfWork for PgIdentityUnitOfWork {
    async fn commit_tenant_registration(
        &self,
        tenant: &Tenant,
        user: &User,
        credential: &Credential,
        relation: &TenantUser,
    ) -> Result<(), DomainError> {
        // 1. Inicia a transação. O banco "trava" o estado para essa conexão.
        let mut tx = self.pool.begin().await.map_err(|e| {
            DomainError::DatabaseError(format!("Falha ao iniciar transação: {}", e))
        })?;

        // 2. Insere o Tenant
        sqlx::query!(
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
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::DatabaseError(format!("Erro ao salvar tenant: {}", e)))?;

        // 3. Insere o Usuário
        sqlx::query!(
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
            user.updated_at,
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::DatabaseError(format!("Erro ao salvar usuário: {}", e)))?;

        // 4. Insere a Credencial
        sqlx::query!(
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
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| DomainError::DatabaseError(format!("Erro ao salvar credencial: {}", e)))?;

        // 5. Insere o Vínculo (Tenant_User)
        let role_str = relation.role.to_string();
        sqlx::query!(
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
        )
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            DomainError::DatabaseError(format!("Erro ao vincular usuário ao tenant: {}", e))
        })?;

        tx.commit().await.map_err(|e| {
            DomainError::DatabaseError(format!("Falha ao commitar transação: {}", e))
        })?;

        Ok(())
    }
}
