use time::OffsetDateTime;
use uuid::Uuid;
use super::role::Role;

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    pub plan: String,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

impl Tenant {
    pub fn new(name: String, slug: String) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            id: Uuid::new_v4(),
            name,
            slug,
            plan: "free".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TenantUser {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    pub role: Role,
    pub is_active: bool,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}