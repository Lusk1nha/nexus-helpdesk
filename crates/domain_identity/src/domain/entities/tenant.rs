use super::role::Role;
use crate::domain::error::DomainError;
use time::OffsetDateTime;
use uuid::Uuid;

/// Subdomain names that must not be allowed as tenant slugs because they
/// collide with platform-level routes (onboarding, admin panel, API host, etc.).
const RESERVED_SLUGS: &[&str] = &[
    "www", "api", "admin", "app", "onboarding", "auth", "mail", "smtp", "ftp", "docs", "help",
    "support", "status", "blog", "billing", "stripe", "internal", "staff", "root", "system",
];

/// Validates a user-provided tenant slug. Returns `Ok(())` if the slug is
/// well-formed and not reserved. The actual uniqueness check (already taken by
/// another tenant) is done separately against the repository.
///
/// Rules:
/// - 3..=32 characters
/// - lowercase letters, digits, and hyphens only
/// - must start and end with a letter or digit
/// - no consecutive hyphens
/// - not in the reserved list
pub fn validate_slug(slug: &str) -> Result<(), DomainError> {
    let len = slug.len();
    if !(3..=32).contains(&len) {
        return Err(DomainError::InvalidSlug(
            "deve ter entre 3 e 32 caracteres.".into(),
        ));
    }

    let bytes = slug.as_bytes();
    let valid_char = |b: u8| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-';
    if !bytes.iter().all(|&b| valid_char(b)) {
        return Err(DomainError::InvalidSlug(
            "use apenas letras minúsculas, dígitos e hífens.".into(),
        ));
    }

    if bytes[0] == b'-' || bytes[len - 1] == b'-' {
        return Err(DomainError::InvalidSlug(
            "não pode começar ou terminar com hífen.".into(),
        ));
    }

    if slug.contains("--") {
        return Err(DomainError::InvalidSlug(
            "hífens consecutivos não são permitidos.".into(),
        ));
    }

    if RESERVED_SLUGS.contains(&slug) {
        return Err(DomainError::InvalidSlug(format!(
            "'{slug}' é um nome reservado da plataforma."
        )));
    }

    Ok(())
}

/// Valid theme identifiers — must stay in sync with packages/theme/src/themes.ts.
const VALID_THEMES: &[&str] = &[
    "midnight", "dawn", "dracula", "nord", "catppuccin", "rose-pine",
    "cyberpunk", "forest", "tokyo-night", "oled-black", "synthwave",
    "night-runner", "terminal", "outrun", "paper", "slate-light",
    "serene", "ice", "coffee", "solarized-light",
];

pub const DEFAULT_THEME: &str = "midnight";

#[derive(Debug, Clone)]
pub struct Tenant {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub slug: String,
    pub theme: String,
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
            description: None,
            slug,
            theme: DEFAULT_THEME.to_string(),
            plan: "free".to_string(),
            is_active: true,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update_name(&mut self, new_name: String) {
        self.name = new_name;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn update_description(&mut self, new_description: Option<String>) {
        self.description = new_description;
        self.updated_at = OffsetDateTime::now_utc();
    }

    pub fn update_theme(&mut self, theme: String) -> Result<(), DomainError> {
        if !VALID_THEMES.contains(&theme.as_str()) {
            return Err(DomainError::Validation(format!(
                "Tema '{theme}' inválido."
            )));
        }
        self.theme = theme;
        self.updated_at = OffsetDateTime::now_utc();
        Ok(())
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
