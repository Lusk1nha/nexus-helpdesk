use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct LogoutCommand {
    /// JTI of the refresh token to revoke, if the caller provided one.
    pub refresh_jti: Option<Uuid>,
    /// User whose sessions should be wiped. If `revoke_all` is true every
    /// active refresh token of this user is revoked (global logout).
    pub user_id: Uuid,
    pub revoke_all: bool,
}

pub struct LogoutUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl LogoutUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(name = "logout", skip(self, command), fields(user_id = %command.user_id))]
    pub async fn execute(&self, command: LogoutCommand) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        if command.revoke_all {
            uow.refresh_tokens()
                .revoke_all_for_user(command.user_id)
                .await?;
        } else if let Some(jti) = command.refresh_jti {
            uow.refresh_tokens().revoke(jti).await?;
        }

        uow.commit().await?;
        Ok(())
    }
}
