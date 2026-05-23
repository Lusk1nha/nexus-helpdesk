use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::ApiKey;
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct ListApiKeysCommand {
    pub tenant_id: Uuid,
}

pub struct ListApiKeysUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl ListApiKeysUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    pub async fn execute(&self, command: ListApiKeysCommand) -> Result<Vec<ApiKey>, DomainError> {
        let mut uow = self.uow_manager.begin().await?;
        let keys = uow.api_keys().list_by_tenant(command.tenant_id).await?;
        uow.commit().await?;
        Ok(keys)
    }
}
