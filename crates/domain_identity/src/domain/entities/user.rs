use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub full_name: String,
    pub avatar_url: Option<String>,
    pub timezone: Option<String>,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl User {
    pub fn new(email: String, full_name: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            email,
            full_name,
            avatar_url: None,
            timezone: Some("UTC".to_string()),
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }
}
