use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::{TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct ListUsersCommand {
    pub tenant_id: Uuid,
}

pub struct ListUsersUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl ListUsersUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    pub async fn execute(
        &self,
        command: ListUsersCommand,
    ) -> Result<Vec<(User, TenantUser)>, DomainError> {
        let mut uow = self.uow_manager.begin().await?;
        let members = uow.tenants().list_members(command.tenant_id).await?;
        uow.commit().await?;
        Ok(members)
    }
}
