use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{Credential, Tenant, TenantUser, User};
use crate::domain::error::DomainError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Salva um novo usuário no banco de dados.
    async fn create(&self, user: &User) -> Result<(), DomainError>;

    /// Atualiza os dados de um usuário existente (ex: mudar o nome).
    async fn update(&self, user: &User) -> Result<(), DomainError>;

    /// Busca um usuário pelo e-mail (útil para login e validação de duplicidade).
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;

    /// Busca um usuário pelo ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
}

#[async_trait]
pub trait CredentialRepository: Send + Sync {
    /// Cria a credencial inicial do usuário no momento do registro.
    async fn create(&self, credential: &Credential) -> Result<(), DomainError>;

    /// Atualiza a credencial (útil para fluxos de redefinição de senha).
    async fn update(&self, credential: &Credential) -> Result<(), DomainError>;

    /// Busca a credencial de um usuário específico para verificação no Login.
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<Credential>, DomainError>;
}

#[async_trait]
pub trait TenantRepository: Send + Sync {
    /// Registra uma nova empresa (Tenant).
    async fn create(&self, tenant: &Tenant) -> Result<(), DomainError>;

    /// Atualiza os dados da empresa (ex: mudar o nome).
    async fn update(&self, tenant: &Tenant) -> Result<(), DomainError>;

    /// Vincula um usuário a um Tenant com uma função (Role) específica.
    async fn add_user_to_tenant(&self, relation: &TenantUser) -> Result<(), DomainError>;

    /// Busca as informações de um Tenant pelo seu ID.
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Tenant>, DomainError>;

    /// Busca o vínculo de um usuário com qualquer tenant (usado no login).
    async fn find_tenant_user_by_user_id(
        &self,
        id: Uuid,
    ) -> Result<Option<TenantUser>, DomainError>;

    /// Busca o vínculo de um usuário com um tenant específico.
    async fn find_tenant_user(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TenantUser>, DomainError>;

    /// Lista todos os membros ativos de um tenant com seus dados de usuário.
    async fn list_members(
        &self,
        tenant_id: Uuid,
    ) -> Result<Vec<(User, TenantUser)>, DomainError>;
}
