use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RefreshToken {
    pub jti: Uuid,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub token_hash: String,
    pub expires_at: OffsetDateTime,
    pub revoked_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

impl RefreshToken {
    pub fn new(
        jti: Uuid,
        user_id: Uuid,
        tenant_id: Uuid,
        token_hash: String,
        expires_at: OffsetDateTime,
    ) -> Self {
        Self {
            jti,
            user_id,
            tenant_id,
            token_hash,
            expires_at,
            revoked_at: None,
            created_at: OffsetDateTime::now_utc(),
        }
    }

    pub fn is_active(&self) -> bool {
        self.revoked_at.is_none() && OffsetDateTime::now_utc() < self.expires_at
    }
}
