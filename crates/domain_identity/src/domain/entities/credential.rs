use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Credential {
    pub user_id: Uuid,
    pub password_hash: String,
    pub failed_attempts: i32,
    pub last_login_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Credential {
    pub fn new(user_id: Uuid, password_hash: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            user_id,
            password_hash,
            failed_attempts: 0,
            last_login_at: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn register_failed_attempt(&mut self) {
        self.failed_attempts += 1;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn reset_attempts(&mut self) {
        self.failed_attempts = 0;
        self.last_login_at = Some(OffsetDateTime::now_utc());
        self.updated_at = OffsetDateTime::now_utc();
    }
}
