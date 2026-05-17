use std::sync::Arc;

use crate::domain::entities::{Credential, Role, Tenant, TenantUser, User};
use crate::domain::error::DomainError;
use crate::domain::ports::{PasswordHasher, UnitOfWorkManager, UserRepository};

pub struct RegisterTenantCommand {
    pub tenant_name: String,
    pub admin_full_name: String,
    pub admin_email: String,
    pub admin_plain_password: String,
}

pub struct RegisterTenantUseCase {
    user_repo: Arc<dyn UserRepository>,
    uow_manager: Arc<dyn UnitOfWorkManager>,
    password_hasher: Arc<dyn PasswordHasher>,
}

impl RegisterTenantUseCase {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        uow_manager: Arc<dyn UnitOfWorkManager>,
        password_hasher: Arc<dyn PasswordHasher>,
    ) -> Self {
        Self {
            user_repo,
            uow_manager,
            password_hasher,
        }
    }

    pub async fn execute(
        &self,
        command: RegisterTenantCommand,
    ) -> Result<(Tenant, User), DomainError> {
        // 1. Regra de Negócio: E-mail único (Validação rápida, fora da transação)
        if self
            .user_repo
            .find_by_email(&command.admin_email)
            .await?
            .is_some()
        {
            return Err(DomainError::UserAlreadyExists);
        }

        // 2. Geração do Slug (Ex: "Minha Empresa LTDA" -> "minha-empresa-ltda")
        let slug = slug::slugify(&command.tenant_name);

        // 3. Construção das Entidades do Domínio
        let tenant = Tenant::new(command.tenant_name, slug);
        let user = User::new(command.admin_email, command.admin_full_name);

        // 4. Segurança e Vínculos
        let hashed_password = self.password_hasher.hash(&command.admin_plain_password)?;
        let credential = Credential::new(user.id, hashed_password);

        let relation = TenantUser::new(tenant.id, user.id, Role::Admin);

        // ==========================================
        // 5. O VERDADEIRO UNIT OF WORK EM AÇÃO
        // ==========================================

        // Inicia a transação genérica no banco de dados
        let mut uow = self.uow_manager.begin().await?;

        // Acessa os repositórios que estão "amarrados" a essa transação específica
        uow.tenants().create(&tenant).await?;
        uow.users().create(&user).await?;
        uow.credentials().create(&credential).await?;

        uow.tenants().add_user_to_tenant(&relation).await?;

        // Se tudo deu certo, efetiva as mudanças no banco de dados
        uow.commit().await?;

        Ok((tenant, user))
    }
}
