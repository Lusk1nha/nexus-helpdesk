use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::entities::RefreshToken;
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct IssueRefreshTokenCommand {
    pub jti: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub token_hash: String,
    pub expires_at: OffsetDateTime,
}

pub struct IssueRefreshTokenUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl IssueRefreshTokenUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(name = "issue_refresh_token", skip(self, command), fields(user_id = %command.user_id))]
    pub async fn execute(&self, command: IssueRefreshTokenCommand) -> Result<(), DomainError> {
        let token = RefreshToken::new(
            command.jti,
            command.user_id,
            command.tenant_id,
            command.token_hash,
            command.expires_at,
        );
        let mut uow = self.uow_manager.begin().await?;
        uow.refresh_tokens().create(&token).await?;
        uow.commit().await?;
        Ok(())
    }
}
