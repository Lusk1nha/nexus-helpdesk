use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::entities::{ApiKey, Credential, RefreshToken, Tenant, TenantUser, User};
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

    /// Atualiza o vínculo de um usuário com um tenant (role, is_active).
    async fn update_tenant_user(&self, relation: &TenantUser) -> Result<(), DomainError>;

    /// Busca o vínculo de um usuário com um tenant específico.
    async fn find_tenant_user(
        &self,
        tenant_id: Uuid,
        user_id: Uuid,
    ) -> Result<Option<TenantUser>, DomainError>;

    /// Lista todos os membros ativos de um tenant com seus dados de usuário.
    async fn list_members(&self, tenant_id: Uuid) -> Result<Vec<(User, TenantUser)>, DomainError>;
}

#[async_trait]
pub trait RefreshTokenRepository: Send + Sync {
    /// Persiste um novo refresh token.
    async fn create(&self, token: &RefreshToken) -> Result<(), DomainError>;

    /// Busca um refresh token pelo seu jti (id pública).
    async fn find_by_jti(&self, jti: Uuid) -> Result<Option<RefreshToken>, DomainError>;

    /// Marca como revogado um refresh token específico.
    async fn revoke(&self, jti: Uuid) -> Result<(), DomainError>;

    /// Revoga todos os refresh tokens ativos de um usuário (logout global).
    async fn revoke_all_for_user(&self, user_id: Uuid) -> Result<(), DomainError>;
}

#[async_trait]
pub trait ApiKeyRepository: Send + Sync {
    /// Cria uma nova API key.
    async fn create(&self, api_key: &ApiKey) -> Result<(), DomainError>;

    /// Busca uma API key pelo hash do segredo (usado na autenticação).
    async fn find_by_hash(&self, key_hash: &str) -> Result<Option<ApiKey>, DomainError>;

    /// Lista as API keys de um tenant.
    async fn list_by_tenant(&self, tenant_id: Uuid) -> Result<Vec<ApiKey>, DomainError>;

    /// Revoga uma API key.
    async fn revoke(&self, id: Uuid, tenant_id: Uuid) -> Result<(), DomainError>;

    /// Atualiza last_used_at — chamado a cada autenticação bem-sucedida.
    async fn touch_last_used(&self, id: Uuid) -> Result<(), DomainError>;
}
