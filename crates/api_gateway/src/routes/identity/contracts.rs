// crates/api_gateway/src/routes/identity/contracts.rs

use domain_identity::domain::entities::{Role, Tenant, TenantUser, User};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::jwt::Claims;

// ─── Me ───────────────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct GetMeResponse {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    #[schema(value_type = String, example = "admin")]
    pub role: Role,
    #[schema(example = "Você está autenticado e acessando os dados da sua empresa!")]
    pub message: String,
}

impl From<Claims> for GetMeResponse {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.sub,
            tenant_id: claims.tenant_id,
            role: claims.role,
            message: "Você está autenticado e acessando os dados da sua empresa!".to_string(),
        }
    }
}

// ─── Register ─────────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterTenantPayload {
    #[validate(length(
        min = 3,
        message = "O nome da empresa deve ter no mínimo 3 caracteres."
    ))]
    #[schema(example = "Nexus Corp")]
    pub tenant_name: String,

    #[validate(length(min = 3, message = "O nome do administrador é muito curto."))]
    #[schema(example = "Lucas P.")]
    pub admin_full_name: String,

    #[validate(email(message = "O formato do e-mail é inválido."))]
    #[schema(example = "lucas@nexuscorp.com")]
    pub admin_email: String,

    #[validate(length(min = 8, message = "A senha deve ter no mínimo 8 caracteres."))]
    #[schema(example = "SenhaForte123!")]
    pub admin_password: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterTenantResponse {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
    #[schema(example = "Empresa registrada com sucesso!")]
    pub message: String,
}

impl From<(Tenant, User)> for RegisterTenantResponse {
    fn from((tenant, user): (Tenant, User)) -> Self {
        Self {
            tenant_id: tenant.id,
            user_id: user.id,
            message: "Empresa registrada com sucesso!".to_string(),
        }
    }
}

// ─── Login ────────────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginPayload {
    #[validate(email(message = "Formato de e-mail inválido."))]
    #[schema(example = "lucas@nexuscorp.com")]
    pub email: String,

    #[validate(length(min = 1, message = "A senha é obrigatória."))]
    #[schema(example = "SenhaForte123!")]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// Short-lived access token (default 15 min). Renamed from `token` —
    /// the legacy `token` alias is kept for clients still on the old payload.
    #[serde(alias = "token")]
    pub access_token: String,
    /// Long-lived refresh token. Send back to POST /refresh to mint a new pair.
    pub refresh_token: String,
    /// Seconds until `access_token` expires.
    pub access_token_expires_in: i64,
    /// Convenience copy of the access token under its legacy name. Allows
    /// existing tests/clients that read `body.token` to keep working.
    pub token: String,
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    #[schema(value_type = String, example = "admin")]
    pub role: Role,
}

// ─── Refresh ──────────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenPayload {
    #[validate(length(min = 1, message = "O refresh token é obrigatório."))]
    pub refresh_token: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct RefreshTokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub access_token_expires_in: i64,
}

// ─── Logout ──────────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogoutPayload {
    /// Token a ser revogado. Se omitido com `everywhere=false`, apenas o
    /// access token expira normalmente (logout local).
    pub refresh_token: Option<String>,
    /// Quando true, revoga todas as sessões do usuário (logout em todos os
    /// dispositivos).
    #[serde(default)]
    pub everywhere: bool,
}

// ─── Admin reset password ─────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct AdminResetPasswordPayload {
    #[validate(custom(function = "validate_optional_password"))]
    #[schema(example = "NovaSenhaTemporaria123!", nullable = true)]
    pub temporary_password: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ResetPasswordResponse {
    #[schema(
        example = "Usuário desbloqueado e credenciais atualizadas pelo administrador com sucesso."
    )]
    pub message: String,
}

fn validate_optional_password(password: &str) -> Result<(), validator::ValidationError> {
    if password.trim().len() < 8 {
        let mut error = validator::ValidationError::new("length");
        error.message = Some(std::borrow::Cow::Borrowed(
            "A nova senha deve ter no mínimo 8 caracteres.",
        ));
        return Err(error);
    }
    Ok(())
}

// ─── Invite user ─────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InviteUserPayload {
    #[validate(email(message = "Formato de e-mail inválido."))]
    #[schema(example = "agente@acme.com")]
    pub email: String,

    #[validate(length(min = 3, message = "O nome deve ter no mínimo 3 caracteres."))]
    #[schema(example = "Maria Silva")]
    pub full_name: String,

    #[validate(length(min = 1, message = "O papel (role) é obrigatório."))]
    #[schema(example = "agent")]
    pub role: String,

    #[validate(length(
        min = 8,
        message = "A senha temporária deve ter no mínimo 8 caracteres."
    ))]
    #[schema(example = "SenhaTemp123!")]
    pub temporary_password: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct InviteUserResponse {
    pub user_id: Uuid,
    #[schema(example = "Usuário convidado com sucesso.")]
    pub message: String,
}

impl From<User> for InviteUserResponse {
    fn from(u: User) -> Self {
        Self {
            user_id: u.id,
            message: "Usuário convidado com sucesso.".to_string(),
        }
    }
}

// ─── List users ───────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TenantMemberResponse {
    pub user_id: Uuid,
    pub email: String,
    pub full_name: String,
    #[schema(value_type = String, example = "agent")]
    pub role: Role,
    pub is_active: bool,
    #[schema(value_type = String)]
    pub joined_at: OffsetDateTime,
}

impl From<(User, TenantUser)> for TenantMemberResponse {
    fn from((user, tenant_user): (User, TenantUser)) -> Self {
        Self {
            user_id: user.id,
            email: user.email,
            full_name: user.full_name,
            role: tenant_user.role,
            is_active: tenant_user.is_active,
            joined_at: tenant_user.created_at,
        }
    }
}

// ─── Change role ──────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChangeUserRolePayload {
    #[validate(length(min = 1, message = "O papel (role) é obrigatório."))]
    #[schema(example = "agent")]
    pub role: String,
}

// ─── Update status ────────────────────────────────────────────────────────────

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUserStatusPayload {
    #[schema(example = true)]
    pub active: bool,
}

// ─── Tenant info ──────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TenantResponse {
    pub id: Uuid,
    pub name: String,
    pub slug: String,
    #[schema(example = "free")]
    pub plan: String,
    pub is_active: bool,
    #[schema(value_type = String)]
    pub created_at: OffsetDateTime,
}

// ─── API keys ────────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyPayload {
    #[validate(length(
        min = 3,
        max = 120,
        message = "O nome da chave deve ter entre 3 e 120 caracteres."
    ))]
    #[schema(example = "CI deploy bot")]
    pub name: String,

    /// Papel atribuído à chave. Restringe o que ela pode fazer.
    #[validate(length(min = 1, message = "O papel (role) é obrigatório."))]
    #[schema(example = "agent")]
    pub role: String,

    /// Dias até expirar. Se omitido, a chave não expira.
    #[schema(example = 90, nullable = true)]
    pub expires_in_days: Option<u32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,
    #[schema(value_type = String, example = "agent")]
    pub role: Role,
    /// **Valor completo da chave.** Mostrado uma única vez no momento da
    /// criação — não é possível recuperá-lo depois. Use no header
    /// `X-API-Key: <plaintext>`.
    pub plaintext: String,
    #[schema(value_type = String, nullable = true)]
    pub expires_at: Option<OffsetDateTime>,
    #[schema(value_type = String)]
    pub created_at: OffsetDateTime,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key_prefix: String,

    #[schema(value_type = String, example = "agent")]
    pub role: Role,
    pub is_active: bool,

    #[schema(value_type = String, nullable = true)]
    pub expires_at: Option<OffsetDateTime>,

    #[schema(value_type = String, nullable = true)]
    pub last_used_at: Option<OffsetDateTime>,

    #[schema(value_type = String, nullable = true)]
    pub revoked_at: Option<OffsetDateTime>,

    #[schema(value_type = String)]
    pub created_at: OffsetDateTime,
}

impl From<domain_identity::domain::entities::ApiKey> for ApiKeyResponse {
    fn from(k: domain_identity::domain::entities::ApiKey) -> Self {
        let is_active = k.is_active();
        Self {
            id: k.id,
            name: k.name,
            key_prefix: k.key_prefix,
            role: k.role,
            is_active,
            expires_at: k.expires_at,
            last_used_at: k.last_used_at,
            revoked_at: k.revoked_at,
            created_at: k.created_at,
        }
    }
}

impl From<Tenant> for TenantResponse {
    fn from(t: Tenant) -> Self {
        Self {
            id: t.id,
            name: t.name,
            slug: t.slug,
            plan: t.plan,
            is_active: t.is_active,
            created_at: t.created_at,
        }
    }
}
