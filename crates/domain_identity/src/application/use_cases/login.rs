use crate::domain::entities::{TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::{
    CredentialRepository, PasswordHasher, TenantRepository, UserRepository,
};
use std::sync::Arc;

pub struct LoginCommand {
    pub email: String,
    pub plain_password: String,
}

pub struct LoginUseCase {
    user_repo: Arc<dyn UserRepository>,
    credential_repo: Arc<dyn CredentialRepository>,
    tenant_repo: Arc<dyn TenantRepository>, // Precisamos saber de qual empresa ele é
    password_hasher: Arc<dyn PasswordHasher>,
}

impl LoginUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        credential_repo: Arc<dyn CredentialRepository>,
        tenant_repo: Arc<dyn TenantRepository>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repo,
            credential_repo,
            tenant_repo,
            password_hasher,
        }
    }

    pub async fn execute(&self, command: LoginCommand) -> Result<(User, TenantUser), DomainError> {
        // 1. Busca o usuário
        let user = self
            .user_repo
            .find_by_email(&command.email)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // 2. Busca a credencial
        let credential = self
            .credential_repo
            .find_by_user_id(user.id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?;

        // 3. Verifica a senha
        if !self
            .password_hasher
            .verify(&command.plain_password, &credential.password_hash)?
        {
            return Err(DomainError::InvalidCredentials);
        }

        // 4. Busca o vínculo com o Tenant (Assumindo que você adicione esse método no TenantRepository)
        // Nota: Você precisará adicionar `find_tenant_user_by_user_id` no seu trait TenantRepository
        let tenant_user = self
            .tenant_repo
            .find_tenant_user_by_user_id(user.id)
            .await?
            .ok_or(DomainError::InvalidCredentials)?; // Sem empresa? Não loga.

        Ok((user, tenant_user))
    }
}
