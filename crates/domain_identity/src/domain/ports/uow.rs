use super::{CredentialRepository, TenantRepository, UserRepository};
use crate::domain::error::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait UnitOfWorkManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn UnitOfWork>, DomainError>;
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    fn users(&mut self) -> Box<dyn UserRepository + '_>;
    fn tenants(&mut self) -> Box<dyn TenantRepository + '_>;
    fn credentials(&mut self) -> Box<dyn CredentialRepository + '_>;

    async fn commit(self: Box<Self>) -> Result<(), DomainError>;
    async fn rollback(self: Box<Self>) -> Result<(), DomainError>;
}
