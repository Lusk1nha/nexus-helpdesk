use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use super::contracts::{RegisterTenantPayload, RegisterTenantResponse};
use crate::{
    app_state::AppState, error::ApiError, middleware::auth::AuthUser,
    routes::identity::contracts::GetMeResponse,
};
use domain_identity::application::use_cases::register_tenant::RegisterTenantCommand;

pub async fn get_me_handler(
    AuthUser(claims): AuthUser,
) -> Result<(StatusCode, Json<GetMeResponse>), ApiError> {
    Ok((StatusCode::OK, Json(claims.into())))
}

pub async fn register_tenant_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterTenantPayload>,
) -> Result<(StatusCode, Json<RegisterTenantResponse>), ApiError> {
    // 1. Validação do Payload (Fail Fast)
    payload.validate()?;

    // 2. Mapeamento para o Command do Domínio
    let command = RegisterTenantCommand {
        tenant_name: payload.tenant_name,
        admin_full_name: payload.admin_full_name,
        admin_email: payload.admin_email,
        admin_plain_password: payload.admin_password,
    };

    // 3. Execução do Use Case (Erros sobem automaticamente via '?')
    let result_tuple = state.register_tenant_use_case.execute(command).await?;

    // 4. Retorno usando .into() para converter (Tenant, User) na Resposta tipada
    Ok((StatusCode::CREATED, Json(result_tuple.into())))
}
