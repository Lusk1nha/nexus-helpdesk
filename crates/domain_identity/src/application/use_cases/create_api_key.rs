use std::sync::Arc;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::domain::entities::{ApiKey, Role};
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct CreateApiKeyCommand {
    pub tenant_id: Uuid,
    pub created_by: Uuid,
    pub name: String,
    pub role: Role,
    pub key_prefix: String,
    pub key_hash: String,
    pub expires_at: Option<OffsetDateTime>,
}

pub struct CreateApiKeyUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl CreateApiKeyUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(name = "create_api_key", skip(self, command), fields(tenant_id = %command.tenant_id))]
    pub async fn execute(&self, command: CreateApiKeyCommand) -> Result<ApiKey, DomainError> {
        let api_key = ApiKey {
            id: Uuid::new_v4(),
            tenant_id: command.tenant_id,
            name: command.name,
            key_prefix: command.key_prefix,
            key_hash: command.key_hash,
            role: command.role,
            created_by: Some(command.created_by),
            expires_at: command.expires_at,
            revoked_at: None,
            last_used_at: None,
            created_at: OffsetDateTime::now_utc(),
        };

        let mut uow = self.uow_manager.begin().await?;
        uow.api_keys().create(&api_key).await?;
        uow.commit().await?;

        Ok(api_key)
    }
}
