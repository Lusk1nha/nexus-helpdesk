use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone)]
pub enum DatabaseConnection {
    Pool(PgPool),
    Transaction(Arc<Mutex<Option<Transaction<'static, Postgres>>>>),
}
