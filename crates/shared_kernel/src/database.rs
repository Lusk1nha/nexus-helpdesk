use sqlx::{PgPool, Postgres, Transaction};
use std::sync::Arc;
use tokio::sync::Mutex;

// Essa abstração agora vive no Shared Kernel e pertence ao sistema inteiro
pub enum DatabaseConnection {
    Pool(PgPool),
    Transaction(Arc<Mutex<Option<Transaction<'static, Postgres>>>>),
}
