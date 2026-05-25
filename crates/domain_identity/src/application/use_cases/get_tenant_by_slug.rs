use std::sync::Arc;

use crate::domain::entities::Tenant;
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct GetTenantBySlugCommand {
    pub slug: String,
}

pub struct GetTenantBySlugUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl GetTenantBySlugUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "get_tenant_by_slug",
        skip(self, command),
        fields(slug = %command.slug)
    )]
    pub async fn execute(&self, command: GetTenantBySlugCommand) -> Result<Tenant, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let tenant = uow
            .tenants()
            .find_by_slug(&command.slug)
            .await?
            .ok_or(DomainError::TenantNotFound)?;

        uow.commit().await?;
        Ok(tenant)
    }
}
