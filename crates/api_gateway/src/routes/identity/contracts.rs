// crates/api_gateway/src/routes/identity/contracts.rs

use domain_identity::domain::entities::{Role, Tenant, TenantUser, User};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

use crate::utils::jwt::Claims;

#[derive(Serialize, ToSchema)]
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

#[derive(Deserialize, Validate, ToSchema)]
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

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginPayload {
    #[validate(email(message = "Formato de e-mail inválido."))]
    #[schema(example = "lucas@nexuscorp.com")]
    pub email: String,

    #[validate(length(min = 1, message = "A senha é obrigatória."))]
    #[schema(example = "SenhaForte123!")]
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct LoginResponse {
    pub token: String,

    pub user_id: Uuid,
    pub tenant_id: Uuid,

    #[schema(value_type = String, example = "admin")]
    pub role: Role,
}

// --- DTOs do Endpoint Administrativo de Desbloqueio/Reset ---
#[derive(Deserialize, Validate, ToSchema)]
pub struct AdminResetPasswordPayload {
    #[validate(custom(function = "validate_optional_password"))]
    #[schema(example = "NovaSenhaTemporaria123!", nullable = true)]
    pub temporary_password: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ResetPasswordResponse {
    #[schema(
        example = "Usuário desbloqueado e credenciais atualizadas pelo administrador com sucesso."
    )]
    pub message: String,
}

// A sua função de validação bate perfeitamente com a assinatura esperada (&str)
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

// ─── Invite User ─────────────────────────────────────────────────────────────

#[derive(Deserialize, Validate, ToSchema)]
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

    #[validate(length(min = 8, message = "A senha temporária deve ter no mínimo 8 caracteres."))]
    #[schema(example = "SenhaTemp123!")]
    pub temporary_password: String,
}

#[derive(Serialize, ToSchema)]
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

// ─── List Users ───────────────────────────────────────────────────────────────

#[derive(Serialize, ToSchema)]
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
