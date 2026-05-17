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

    pub async fn execute(&self, command: ResetPasswordCommand) -> Result<(), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        // 1. Busca o vínculo do usuário alvo com o Tenant para garantir o isolamento B2B
        let tenant_user = uow
            .tenants()
            .find_tenant_user_by_user_id(command.target_user_id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // 🛡️ BARREIRA MULTI-TENANT: O usuário alvo pertence ao mesmo Tenant do operador?
        if tenant_user.tenant_id != command.operator_tenant_id {
            return Err(DomainError::InvalidCredentials); // 403/401 disfarçado
        }

        // 2. Se for um usuário comum tentando resetar sem passar nova senha, barra
        if !command.is_admin_override && command.new_plain_password.is_none() {
            return Err(DomainError::InvalidCredentials);
        }

        // 3. Busca a credencial
        let mut credential = uow
            .credentials()
            .find_by_user_id(command.target_user_id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // 4. Atualiza a senha se fornecida
        if let Some(plain_pwd) = command.new_plain_password {
            credential.password_hash = self.password_hasher.hash(&plain_pwd)?;
        }

        // 🎉 Destranca o usuário e zera os contadores de erro
        credential.reset_attempts();

        uow.credentials().update(&credential).await?;
        uow.commit().await?;

        Ok(())
    }
}
