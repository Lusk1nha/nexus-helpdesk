use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct RevokeApiKeyCommand {
    pub api_key_id: Uuid,
    pub tenant_id: Uuid,
}

pub struct RevokeApiKeyUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl RevokeApiKeyUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(name = "revoke_api_key", skip(self, command), fields(api_key_id = %command.api_key_id))]
    pub async fn execute(&self, command: RevokeApiKeyCommand) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;
        uow.api_keys()
            .revoke(command.api_key_id, command.tenant_id)
            .await?;
        uow.commit().await?;
        Ok(())
    }
}
