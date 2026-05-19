use std::sync::Arc;
use uuid::Uuid;

use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct UpdateUserStatusCommand {
    pub operator_tenant_id: Uuid,
    pub target_user_id: Uuid,
    pub active: bool,
}

pub struct UpdateUserStatusUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl UpdateUserStatusUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "update_user_status",
        skip(self, command),
        fields(
            operator_tenant_id = %command.operator_tenant_id,
            target_user_id = %command.target_user_id,
            active = command.active,
        )
    )]
    pub async fn execute(&self, command: UpdateUserStatusCommand) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let mut tenant_user = uow
            .tenants()
            .find_tenant_user(command.operator_tenant_id, command.target_user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        if command.active {
            tenant_user.reactivate();
        } else {
            tenant_user.deactivate();
        }

        uow.tenants().update_tenant_user(&tenant_user).await?;
        uow.commit().await?;

        tracing::info!("user status updated");
        Ok(())
    }
}
