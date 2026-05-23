use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::entities::{RefreshToken, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct RefreshSessionCommand {
    pub presented_jti: Uuid,
    pub presented_token_hash: String,
    pub new_jti: Uuid,
    pub new_token_hash: String,
    pub new_expires_at: OffsetDateTime,
}

pub struct RefreshSessionResult {
    pub user: User,
    pub tenant_user: TenantUser,
}

pub struct RefreshSessionUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl RefreshSessionUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    /// Rotates a refresh token: validates the presented one, marks it revoked,
    /// then persists the replacement. Returns the user + tenant association so
    /// the caller can mint a fresh access token.
    #[tracing::instrument(name = "refresh_session", skip(self, command), fields(jti = %command.presented_jti))]
    pub async fn execute(
        &self,
        command: RefreshSessionCommand,
    ) -> Result<RefreshSessionResult, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        // Lock-da-linha: dois refreshes concorrentes com o mesmo jti vão
        // serializar aqui. O primeiro revoga + insere o substituto; o segundo
        // só ganha o lock depois do COMMIT do primeiro e verá o token como
        // `revoked_at != NULL`, falhando com InvalidCredentials.
        let existing = uow
            .refresh_tokens()
            .find_by_jti_for_update(command.presented_jti)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if !existing.is_active() || existing.token_hash != command.presented_token_hash {
            tracing::warn!(jti = %command.presented_jti, "refresh rejected: token revoked, expired, or mismatched hash");
            // If the hash mismatches we may be facing token theft — revoke
            // every active session for this user as a precaution.
            if existing.token_hash != command.presented_token_hash {
                uow.refresh_tokens()
                    .revoke_all_for_user(existing.user_id)
                    .await?;
                uow.commit().await?;
            }
            return Err(DomainError::InvalidCredentials);
        }

        let user = uow
            .users()
            .find_by_id(existing.user_id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if !user.is_active {
            return Err(DomainError::InvalidCredentials);
        }

        let tenant_user = uow
            .tenants()
            .find_tenant_user(existing.tenant_id, existing.user_id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if !tenant_user.is_active {
            return Err(DomainError::InvalidCredentials);
        }

        // Rotation: revoke the presented token, persist the replacement.
        uow.refresh_tokens().revoke(existing.jti).await?;
        let replacement = RefreshToken::new(
            command.new_jti,
            existing.user_id,
            existing.tenant_id,
            command.new_token_hash,
            command.new_expires_at,
        );
        uow.refresh_tokens().create(&replacement).await?;

        uow.commit().await?;

        Ok(RefreshSessionResult { user, tenant_user })
    }
}
