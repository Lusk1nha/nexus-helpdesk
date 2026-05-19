use axum::{Json, extract::{Path, State}, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use super::contracts::{
    AdminResetPasswordPayload, ChangeUserRolePayload, GetMeResponse, InviteUserPayload,
    InviteUserResponse, LoginPayload, LoginResponse, RegisterTenantPayload, RegisterTenantResponse,
    ResetPasswordResponse, TenantMemberResponse, TenantResponse, UpdateUserStatusPayload,
};
use crate::{
    app_state::AppState,
    error::ApiError,
    middleware::auth::{AdminUser, AuthUser},
    utils::jwt::sign_jwt,
};

use domain_identity::application::use_cases::{
    change_user_role::ChangeUserRoleCommand, get_tenant::GetTenantCommand,
    invite_user::InviteUserCommand, list_users::ListUsersCommand,
    update_user_status::UpdateUserStatusCommand, LoginCommand, ResetPasswordCommand,
    register_tenant::RegisterTenantCommand,
};

#[utoipa::path(
    get,
    path = "/api/v1/identity/me",
    responses(
        (status = 200, description = "Retorna os dados do usuário e da empresa baseados no Token JWT", body = GetMeResponse),
        (status = 401, description = "Token ausente, inválido ou expirado")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_me_handler(
    AuthUser(claims): AuthUser,
) -> Result<(StatusCode, Json<GetMeResponse>), ApiError> {
    Ok((StatusCode::OK, Json(claims.into())))
}

#[utoipa::path(
    post,
    path = "/api/v1/identity/register",
    request_body = RegisterTenantPayload,
    responses(
        (status = 201, description = "Empresa e usuário administrador criados com sucesso", body = RegisterTenantResponse),
        (status = 400, description = "Erro de validação nos dados enviados"),
        (status = 409, description = "O e-mail informado já está em uso")
    )
)]
pub async fn register_tenant_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterTenantPayload>,
) -> Result<(StatusCode, Json<RegisterTenantResponse>), ApiError> {
    payload.validate()?;

    let command = RegisterTenantCommand {
        tenant_name: payload.tenant_name,
        admin_full_name: payload.admin_full_name,
        admin_email: payload.admin_email,
        admin_plain_password: payload.admin_password,
    };

    let result_tuple = state.identity.register_tenant.execute(command).await?;

    Ok((StatusCode::CREATED, Json(result_tuple.into())))
}

#[utoipa::path(
    post,
    path = "/api/v1/identity/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login efetuado com sucesso (Retorna o JWT)", body = LoginResponse),
        (status = 400, description = "Erro de validação (Ex: e-mail mal formatado)"),
        (status = 401, description = "Credenciais inválidas")
    )
)]
pub async fn login_handler(
    State(state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> Result<(StatusCode, Json<LoginResponse>), ApiError> {
    payload.validate()?;

    let command = LoginCommand {
        email: payload.email,
        plain_password: payload.password,
    };

    let (user, tenant_user) = state.identity.login.execute(command).await?;

    let token = sign_jwt(
        user.id,
        tenant_user.tenant_id,
        tenant_user.role.clone(),
        &state.config.jwt_secret,
    )
    .map_err(|e| ApiError::Internal(format!("Falha ao assinar JWT: {}", e)))?;

    let response = LoginResponse {
        token,
        user_id: user.id,
        tenant_id: tenant_user.tenant_id,
        role: tenant_user.role,
    };

    Ok((StatusCode::OK, Json(response)))
}

#[utoipa::path(
    post,
    path = "/api/v1/identity/admin/users/{id}/unlock-and-reset",
    request_body = AdminResetPasswordPayload,
    responses(
        (status = 200, description = "Admin resetou a senha/desbloqueou o usuário com sucesso", body = ResetPasswordResponse),
        (status = 401, description = "Não autenticado"),
        (status = 403, description = "Acesso negado (Não é admin ou o usuário alvo é de outra empresa)")
    ),
    params(
        ("id" = Uuid, Path, description = "ID do usuário que o Administrador deseja desbloquear/resetar")
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_reset_user_password_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
    axum::extract::Path(target_user_id): axum::extract::Path<Uuid>,
    Json(payload): Json<AdminResetPasswordPayload>,
) -> Result<(StatusCode, Json<ResetPasswordResponse>), ApiError> {
    payload.validate()?;

    let command = ResetPasswordCommand {
        target_user_id,
        operator_tenant_id: admin_claims.tenant_id,

        is_admin_override: true,
        new_plain_password: payload.temporary_password,
    };

    state.identity.reset_password.execute(command).await?;

    Ok((
        StatusCode::OK,
        Json(ResetPasswordResponse {
            message:
                "Usuário desbloqueado e credenciais atualizadas pelo administrador com sucesso."
                    .to_string(),
        }),
    ))
}

#[utoipa::path(
    post,
    path = "/api/v1/identity/users",
    request_body = InviteUserPayload,
    responses(
        (status = 201, description = "Usuário convidado com sucesso", body = InviteUserResponse),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado (somente admin)"),
        (status = 409, description = "E-mail já em uso")
    ),
    security(("bearer_auth" = []))
)]
pub async fn invite_user_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
    Json(payload): Json<InviteUserPayload>,
) -> Result<(StatusCode, Json<InviteUserResponse>), ApiError> {
    payload.validate()?;

    let role = payload
        .role
        .parse::<domain_identity::domain::entities::Role>()
        .map_err(|_| ApiError::Identity(domain_identity::domain::error::DomainError::InvalidRole(payload.role.clone())))?;

    let command = InviteUserCommand {
        operator_tenant_id: admin_claims.tenant_id,
        email: payload.email,
        full_name: payload.full_name,
        role,
        temporary_password: payload.temporary_password,
    };

    let user = state.identity.invite_user.execute(command).await?;
    Ok((StatusCode::CREATED, Json(user.into())))
}

#[utoipa::path(
    get,
    path = "/api/v1/identity/users",
    responses(
        (status = 200, description = "Lista de membros do tenant", body = Vec<TenantMemberResponse>),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado (somente admin)")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_users_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
) -> Result<(StatusCode, Json<Vec<TenantMemberResponse>>), ApiError> {
    let command = ListUsersCommand {
        tenant_id: admin_claims.tenant_id,
    };

    let members = state.identity.list_users.execute(command).await?;
    let body: Vec<TenantMemberResponse> = members.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(body)))
}

#[utoipa::path(
    patch,
    path = "/api/v1/identity/users/{id}/role",
    request_body = ChangeUserRolePayload,
    params(("id" = Uuid, Path, description = "ID do usuário")),
    responses(
        (status = 204, description = "Role atualizado com sucesso"),
        (status = 400, description = "Role inválido"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado (somente admin)"),
        (status = 404, description = "Usuário não encontrado no tenant")
    ),
    security(("bearer_auth" = []))
)]
pub async fn change_user_role_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
    Path(target_user_id): Path<Uuid>,
    Json(payload): Json<ChangeUserRolePayload>,
) -> Result<StatusCode, ApiError> {
    let role = payload
        .role
        .parse::<domain_identity::domain::entities::Role>()
        .map_err(|_| {
            ApiError::Identity(domain_identity::domain::error::DomainError::InvalidRole(
                payload.role.clone(),
            ))
        })?;

    let command = ChangeUserRoleCommand {
        operator_tenant_id: admin_claims.tenant_id,
        target_user_id,
        new_role: role,
    };

    state.identity.change_user_role.execute(command).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    patch,
    path = "/api/v1/identity/users/{id}/status",
    request_body = UpdateUserStatusPayload,
    params(("id" = Uuid, Path, description = "ID do usuário")),
    responses(
        (status = 204, description = "Status atualizado com sucesso"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado (somente admin)"),
        (status = 404, description = "Usuário não encontrado no tenant")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_user_status_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
    Path(target_user_id): Path<Uuid>,
    Json(payload): Json<UpdateUserStatusPayload>,
) -> Result<StatusCode, ApiError> {
    let command = UpdateUserStatusCommand {
        operator_tenant_id: admin_claims.tenant_id,
        target_user_id,
        active: payload.active,
    };

    state.identity.update_user_status.execute(command).await?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/api/v1/identity/tenant",
    responses(
        (status = 200, description = "Informações do tenant atual", body = TenantResponse),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado (somente admin)")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_tenant_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
) -> Result<(StatusCode, Json<TenantResponse>), ApiError> {
    let command = GetTenantCommand {
        tenant_id: admin_claims.tenant_id,
    };

    let tenant = state.identity.get_tenant.execute(command).await?;
    Ok((StatusCode::OK, Json(tenant.into())))
}
