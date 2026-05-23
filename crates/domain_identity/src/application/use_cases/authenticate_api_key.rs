use std::sync::Arc;

use crate::domain::entities::ApiKey;
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct AuthenticateApiKeyCommand {
    pub key_hash: String,
}

pub struct AuthenticateApiKeyUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl AuthenticateApiKeyUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    /// Resolves an incoming API key (already hashed by the caller) to its full
    /// record, enforcing that the key is active. Updates `last_used_at` as a
    /// side-effect.
    #[tracing::instrument(name = "authenticate_api_key", skip(self, command))]
    pub async fn execute(&self, command: AuthenticateApiKeyCommand) -> Result<ApiKey, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let api_key = uow
            .api_keys()
            .find_by_hash(&command.key_hash)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if !api_key.is_active() {
            return Err(DomainError::InvalidCredentials);
        }

        uow.api_keys().touch_last_used(api_key.id).await?;
        uow.commit().await?;

        Ok(api_key)
    }
}
