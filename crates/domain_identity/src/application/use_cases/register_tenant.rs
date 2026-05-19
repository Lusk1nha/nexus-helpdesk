use std::sync::Arc;

use crate::domain::entities::{Credential, Role, Tenant, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::{PasswordHasher, UnitOfWorkManager};

pub struct RegisterTenantCommand {
    pub tenant_name: String,
    pub admin_full_name: String,
    pub admin_email: String,
    pub admin_plain_password: String,
}

pub struct RegisterTenantUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterTenantUseCase {
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
        name = "register_tenant",
        skip(self, command),
        fields(tenant_name = %command.tenant_name, admin_email = %command.admin_email)
    )]
    pub async fn execute(
        &self,
        command: RegisterTenantCommand,
    ) -> Result<(Tenant, User), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        if uow
            .users()
            .find_by_email(&command.admin_email)
            .await?
            .is_some()
        {
            tracing::warn!(email = %command.admin_email, "registration rejected: email already exists");
            return Err(DomainError::UserAlreadyExists);
        }

        let slug = slug::slugify(&command.tenant_name);
        let tenant = Tenant::new(command.tenant_name, slug);
        let user = User::new(command.admin_email, command.admin_full_name);
        let hashed_password = self.password_hasher.hash(&command.admin_plain_password)?;
        let credential = Credential::new(user.id, hashed_password);
        let relation = TenantUser::new(tenant.id, user.id, Role::Admin);

        uow.tenants().create(&tenant).await?;
        uow.users().create(&user).await?;
        uow.credentials().create(&credential).await?;
        uow.tenants().add_user_to_tenant(&relation).await?;
        uow.commit().await?;

        tracing::info!(
            tenant_id = %tenant.id,
            user_id = %user.id,
            "tenant registered"
        );
        Ok((tenant, user))
    }
}
