use super::role::Role;
use time::OffsetDateTime;
use uuid::Uuid;

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

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = OffsetDateTime::now_utc();
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

impl TenantUser {
    pub fn new(tenant_id: Uuid, user_id: Uuid, role: Role) -> Self {
        let now = OffsetDateTime::now_utc();
        Self {
            tenant_id,
            user_id,
            role,
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn change_role(&mut self, new_role: Role) {
        self.role = new_role;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn reactivate(&mut self) {
        self.is_active = true;
        self.updated_at = OffsetDateTime::now_utc();
    }
}
