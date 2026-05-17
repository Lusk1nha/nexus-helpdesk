use domain_identity::domain::entities::{Tenant, User};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::utils::jwt::Claims;

#[derive(Serialize)]
pub struct GetMeResponse {
    pub user_id: Uuid,
    pub tenant_id: Uuid,
    pub role: String,
    pub message: String,
}

// Ensinamos o Rust a converter Claims em GetMeResponse
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

#[derive(Deserialize, Validate)]
pub struct RegisterTenantPayload {
    #[validate(length(
        min = 3,
        message = "O nome da empresa deve ter no mínimo 3 caracteres."
    ))]
    pub tenant_name: String,

    #[validate(length(min = 3, message = "O nome do administrador é muito curto."))]
    pub admin_full_name: String,

    #[validate(email(message = "O formato do e-mail é inválido."))]
    pub admin_email: String,

    #[validate(length(min = 8, message = "A senha deve ter no mínimo 8 caracteres."))]
    pub admin_password: String,
}

#[derive(Serialize)]
pub struct RegisterTenantResponse {
    pub tenant_id: Uuid,
    pub user_id: Uuid,
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
