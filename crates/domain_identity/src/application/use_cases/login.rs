// crates/domain_identity/src/application/use_cases/login.rs

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

    pub async fn execute(&self, command: LoginCommand) -> Result<(User, TenantUser), DomainError> {
        let mut uow = self.uow_manager.begin().await?;

        // 1. Busca o usuário
        let user = uow
            .users()
            .find_by_email(&command.email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // 2. Busca a credencial
        let mut credential = uow
            .credentials()
            .find_by_user_id(user.id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        if credential.is_locked() {
            return Err(DomainError::InvalidCredentials);
        }

        // 3. Verifica a senha
        if !self
            .password_hasher
            .verify(&command.plain_password, &credential.password_hash)?
        {
            // 🔒 Incrementa falha e salva no banco antes de retornar erro
            credential.register_failed_attempt();
            uow.credentials().update(&credential).await?;
            uow.commit().await?;

            return Err(DomainError::InvalidCredentials);
        }

        // 4. Se a senha está correta, busca o vínculo com o Tenant
        let tenant_user = uow
            .tenants()
            .find_tenant_user_by_user_id(user.id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // 🛡️ Regra de Negócio: Impede login se o vínculo com o tenant estiver inativo
        if !tenant_user.is_active {
            return Err(DomainError::InvalidCredentials);
        }

        credential.reset_attempts();
        uow.credentials().update(&credential).await?;

        uow.commit().await?;

        Ok((user, tenant_user))
    }
}
