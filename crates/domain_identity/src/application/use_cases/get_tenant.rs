use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::Tenant;
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct GetTenantCommand {
    pub tenant_id: Uuid,
}

pub struct GetTenantUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl GetTenantUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "get_tenant",
        skip(self, command),
        fields(tenant_id = %command.tenant_id)
    )]
    pub async fn execute(&self, command: GetTenantCommand) -> Result<Tenant, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let tenant = uow
            .tenants()
            .find_by_id(command.tenant_id)
            .await?
            .ok_or(DomainError::TenantNotFound)?;

        uow.commit().await?;
        tracing::info!("tenant retrieved");
        Ok(tenant)
    }
}
