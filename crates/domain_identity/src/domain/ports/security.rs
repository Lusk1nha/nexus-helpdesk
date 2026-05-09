use async_trait::async_trait;

use crate::domain::DomainError;

#[async_trait]
pub trait PasswordHasher: Send + Sync {
    /// Recebe a senha em texto plano e retorna o Hash
    fn hash(&self, plain_password: &str) -> Result<String, DomainError>;

    /// Compara uma senha em texto plano com um hash guardado no banco
    fn verify(&self, plain_password: &str, hash: &str) -> Result<bool, DomainError>;
}
