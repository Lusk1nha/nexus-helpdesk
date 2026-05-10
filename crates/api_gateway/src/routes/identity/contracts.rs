use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

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
