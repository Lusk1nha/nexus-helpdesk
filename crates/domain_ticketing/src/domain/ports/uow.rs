// crates/domain_ticketing/src/domain/ports/uow.rs
use super::{MessageRepository, TicketRepository};
use crate::domain::error::DomainError;
use async_trait::async_trait;

#[async_trait]
pub trait TicketingUnitOfWorkManager: Send + Sync {
    async fn begin(&self) -> Result<Box<dyn TicketingUnitOfWork>, DomainError>;
}

#[async_trait]
pub trait TicketingUnitOfWork: Send + Sync {
    fn tickets(&mut self) -> Box<dyn TicketRepository + '_>;
    fn messages(&mut self) -> Box<dyn MessageRepository + '_>;
    async fn commit(self: Box<Self>) -> Result<(), DomainError>;
    async fn rollback(self: Box<Self>) -> Result<(), DomainError>;
}
