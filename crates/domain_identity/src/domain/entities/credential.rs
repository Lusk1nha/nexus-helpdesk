use time::OffsetDateTime;
use uuid::Uuid;

const MAX_FAILED_ATTEMPTS: i32 = 5;
const LOCKOUT_DURATION_MINUTES: i64 = 15;

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

    pub fn is_locked(&self) -> bool {
        if self.failed_attempts < MAX_FAILED_ATTEMPTS {
            return false;
        }

        // Se estourou as tentativas, verifica se o tempo de punição já passou
        let now = OffsetDateTime::now_utc();
        let lockout_expires_at =
            self.updated_at + time::Duration::minutes(LOCKOUT_DURATION_MINUTES);

        // Se o horário atual for MENOR que o horário de expiração, continua BLOQUEADO
        now < lockout_expires_at
    }

    pub fn register_failed_attempt(&mut self) {
        if self.failed_attempts < MAX_FAILED_ATTEMPTS {
            self.failed_attempts += 1;
        }
    }

    pub fn reset_attempts(&mut self) {
        self.failed_attempts = 0;
        self.last_login_at = Some(OffsetDateTime::now_utc());
    }
}
