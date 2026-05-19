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

    #[tracing::instrument(
        name = "invite_user",
        skip(self, command),
        fields(
            operator_tenant_id = %command.operator_tenant_id,
            email = %command.email,
            role = %command.role,
        )
    )]
    pub async fn execute(&self, command: InviteUserCommand) -> Result<User, DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        if uow.users().find_by_email(&command.email).await?.is_some() {
            tracing::warn!(email = %command.email, "invite rejected: email already exists");
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

        tracing::info!(
            tenant_id = %command.operator_tenant_id,
            user_id = %user.id,
            "user invited"
        );
        Ok(user)
    }
}
