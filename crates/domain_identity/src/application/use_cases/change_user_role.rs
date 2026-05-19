use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::Role;
use crate::domain::error::DomainError;
use crate::domain::ports::UnitOfWorkManager;

pub struct ChangeUserRoleCommand {
    pub operator_tenant_id: Uuid,
    pub target_user_id: Uuid,
    pub new_role: Role,
}

pub struct ChangeUserRoleUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
}

impl ChangeUserRoleUseCase {
    pub fn new(uow_manager: Arc<dyn UnitOfWorkManager>) -> Self {
        Self { uow_manager }
    }

    #[tracing::instrument(
        name = "change_user_role",
        skip(self, command),
        fields(
            operator_tenant_id = %command.operator_tenant_id,
            target_user_id = %command.target_user_id,
            new_role = %command.new_role,
        )
    )]
    pub async fn execute(&self, command: ChangeUserRoleCommand) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let mut tenant_user = uow
            .tenants()
            .find_tenant_user(command.operator_tenant_id, command.target_user_id)
            .await?
            .ok_or(DomainError::UserNotFound)?;

        tenant_user.change_role(command.new_role);
        uow.tenants().update_tenant_user(&tenant_user).await?;
        uow.commit().await?;

        tracing::info!("user role updated");
        Ok(())
    }
}
