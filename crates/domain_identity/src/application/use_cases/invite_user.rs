use std::sync::Arc;
use uuid::Uuid;

use crate::domain::entities::{Credential, Role, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::{PasswordHasher, UnitOfWorkManager};

pub struct InviteUserCommand {
    pub operator_tenant_id: Uuid,
    pub email: String,
    pub full_name: String,
    pub role: Role,
    pub temporary_password: String,
}

pub struct InviteUserUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl InviteUserUseCase {
    pub fn new(
        uow_manager: Arc<dyn UnitOfWorkManager>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            uow_manager,
            password_hasher,
        }
    }

    pub async fn execute(&self, command: InviteUserCommand) -> Result<User, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        if uow.users().find_by_email(&command.email).await?.is_some() {
            return Err(DomainError::UserAlreadyExists);
        }

        let user = User::new(command.email, command.full_name);

        let password_hash = self
            .password_hasher
            .hash(&command.temporary_password)
            .map_err(|e| DomainError::SecurityError(e.to_string()))?;

        let credential = Credential::new(user.id, password_hash);
        let tenant_user = TenantUser::new(command.operator_tenant_id, user.id, command.role);

        uow.users().create(&user).await?;
        uow.credentials().create(&credential).await?;
        uow.tenants().add_user_to_tenant(&tenant_user).await?;

        uow.commit().await?;
        Ok(user)
    }
}
