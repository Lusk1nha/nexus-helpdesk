use std::sync::Arc;

use uuid::Uuid;

use crate::domain::{DomainError, entities::Tenant, ports::UnitOfWorkManager};

pub struct UpdateTenantCommand {
    pub tenant_id: Uuid,

    pub new_name: Option<String>,
    pub new_description: Option<String>,
}

pub struct UpdateTenantUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl UpdateTenantUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "update_tenant",
        skip(self, command),
        fields(
            tenant_id = %command.tenant_id, 
            new_name = ?command.new_name, 
            new_description = ?command.new_description
        )
    )]
    pub async fn execute(&self, command: UpdateTenantCommand) -> Result<Tenant, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let mut tenant = uow
            .tenants()
            .find_by_id(command.tenant_id)
            .await?
            .ok_or(DomainError::TenantNotFound)?;

        if let Some(new_name) = command.new_name {
            tenant.update_name(new_name);
        }

        tenant.update_description(command.new_description);

        uow.tenants().update(&tenant).await?;
        uow.commit().await?;


        tracing::info!(
            "tenant updated successfully, {tenant_id}",
            tenant_id = tenant.id
        );
        Ok(tenant)
    }
}
