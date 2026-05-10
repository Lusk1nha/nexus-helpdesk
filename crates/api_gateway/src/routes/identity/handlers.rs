use axum::{Json, extract::State, http::StatusCode};
use validator::Validate;

use super::contracts::{RegisterTenantPayload, RegisterTenantResponse};
use crate::{app_state::AppState, error::ApiError};
use domain_identity::application::use_cases::register_tenant::RegisterTenantCommand;

pub async fn register_tenant_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterTenantPayload>,
) -> Result<(StatusCode, Json<RegisterTenantResponse>), ApiError> {
    // 1. Validação do Payload (Fail Fast)
    payload.validate().map_err(ApiError::Validation)?;

    // 2. Mapeamento para o Command do Domínio
    let command = RegisterTenantCommand {
        tenant_name: payload.tenant_name,
        admin_full_name: payload.admin_full_name,
        admin_email: payload.admin_email,
        admin_plain_password: payload.admin_password,
    };

    // 3. Execução do Use Case (Erros sobem automaticamente via '?')
    let (tenant, user) = state.register_tenant_use_case.execute(command).await?;

    // 4. Montagem da Resposta
    let response = RegisterTenantResponse {
        tenant_id: tenant.id,
        user_id: user.id,
        message: "Empresa registrada com sucesso!".to_string(),
    };

    Ok((StatusCode::CREATED, Json(response)))
}
