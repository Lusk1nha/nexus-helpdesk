use crate::domain::error::DomainError;
use crate::domain::ports::{PasswordHasher, UnitOfWorkManager};
use std::sync::Arc;
use uuid::Uuid;

pub struct ResetPasswordCommand {
    pub target_user_id: Uuid,
    pub operator_tenant_id: Uuid,
    pub is_admin_override: bool,
    pub new_plain_password: Option<String>,
}

pub struct ResetPasswordUseCase {
    uow_manager: Arc<dyn UnitOfWorkManager>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl ResetPasswordUseCase {
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
        name = "reset_password",
        skip(self, command),
        fields(
            operator_tenant_id = %command.operator_tenant_id,
            target_user_id = %command.target_user_id,
        )
    )]
    pub async fn execute(&self, command: ResetPasswordCommand) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        let tenant_user = uow
            .tenants()
            .find_tenant_user_by_user_id(command.target_user_id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if tenant_user.tenant_id != command.operator_tenant_id {
            tracing::warn!(
                target_user_id = %command.target_user_id,
                "password reset rejected: cross-tenant access attempt"
            );
            return Err(DomainError::InvalidCredentials);
        }

        if !command.is_admin_override && command.new_plain_password.is_none() {
            return Err(DomainError::InvalidCredentials);
        }

        let mut credential = uow
            .credentials()
            .find_by_user_id(command.target_user_id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if let Some(plain_pwd) = command.new_plain_password {
            credential.password_hash = self.password_hasher.hash(&plain_pwd)?;
        }

        credential.reset_attempts();
        uow.credentials().update(&credential).await?;

        // Senha trocada → toda sessão emitida antes deve cair: força re-login
        // em todos os dispositivos. Evita que uma sessão sequestrada continue
        // válida após o reset.
        uow.refresh_tokens()
            .revoke_all_for_user(command.target_user_id)
            .await?;

        uow.commit().await?;

        tracing::info!(target_user_id = %command.target_user_id, "password reset successful");
        Ok(())
    }
}
