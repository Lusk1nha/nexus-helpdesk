use std::sync::Arc;

use crate::domain::entities::{Credential, Role, Tenant, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::{PasswordHasher, UnitOfWorkManager};

/// Self-service signup for end customers.
///
/// Unlike [`InviteUserUseCase`], this is a *public* flow: the customer chooses
/// their own password and is always created with [`Role::Customer`] in the
/// tenant resolved from `slug`. The role is forced server-side so the endpoint
/// can never be abused to mint an admin/agent account.
pub struct SelfRegisterCustomerCommand {
    /// Tenant subdomain slug the customer is signing up under.
    pub tenant_slug: String,
    pub email: String,
    pub full_name: String,
    pub plain_password: String,
}

pub struct SelfRegisterCustomerUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl SelfRegisterCustomerUseCase {
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
        name = "self_register_customer",
        skip(self, command),
        fields(slug = %command.tenant_slug, email = %command.email)
    )]
    pub async fn execute(
        &self,
        command: SelfRegisterCustomerCommand,
    ) -> Result<(Tenant, User), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let tenant = uow
            .tenants()
            .find_by_slug(&command.tenant_slug)
            .await?
            .ok_or(DomainError::TenantNotFound)?;

        if uow.users().find_by_email(&command.email).await?.is_some() {
            tracing::warn!(email = %command.email, "signup rejected: email already exists");
            return Err(DomainError::UserAlreadyExists);
        }

        let user = User::new(command.email, command.full_name);
        let password_hash = self
            .password_hasher
            .hash(&command.plain_password)
            .map_err(|e| DomainError::SecurityError(e.to_string()))?;

        let credential = Credential::new(user.id, password_hash);
        let tenant_user = TenantUser::new(tenant.id, user.id, Role::Customer);

        uow.users().create(&user).await?;
        uow.credentials().create(&credential).await?;
        uow.tenants().add_user_to_tenant(&tenant_user).await?;
        uow.commit().await?;

        tracing::info!(
            tenant_id = %tenant.id,
            user_id = %user.id,
            "customer self-registered"
        );
        Ok((tenant, user))
    }
}
