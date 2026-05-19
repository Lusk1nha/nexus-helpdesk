use crate::domain::entities::{TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::{PasswordHasher, UnitOfWorkManager};
use std::sync::Arc;

pub struct LoginCommand {
    pub email: String,
    pub plain_password: String,
}

pub struct LoginUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl LoginUseCase {
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
        name = "login",
        skip(self, command),
        fields(email = %command.email)
    )]
    pub async fn execute(&self, command: LoginCommand) -> Result<(User, TenantUser), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let user = uow
            .users()
            .find_by_email(&command.email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        let mut credential = uow
            .credentials()
            .find_by_user_id(user.id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if credential.is_locked() {
            tracing::warn!(user_id = %user.id, "login rejected: account locked");
            return Err(DomainError::InvalidCredentials);
        }

        if !self
            .password_hasher
            .verify(&command.plain_password, &credential.password_hash)?
        {
            credential.register_failed_attempt();
            uow.credentials().update(&credential).await?;
            uow.commit().await?;
            tracing::warn!(
                user_id = %user.id,
                failed_attempts = credential.failed_attempts,
                "login rejected: wrong password"
            );
            return Err(DomainError::InvalidCredentials);
        }

        let tenant_user = uow
            .tenants()
            .find_tenant_user_by_user_id(user.id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if !tenant_user.is_active {
            tracing::warn!(user_id = %user.id, "login rejected: user deactivated in tenant");
            return Err(DomainError::InvalidCredentials);
        }

        credential.reset_attempts();
        uow.credentials().update(&credential).await?;
        uow.commit().await?;

        tracing::info!(
            user_id = %user.id,
            tenant_id = %tenant_user.tenant_id,
            role = %tenant_user.role,
            "login successful"
        );
        Ok((user, tenant_user))
    }
}
