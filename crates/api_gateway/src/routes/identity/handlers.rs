use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use axum_extra::extract::cookie::CookieJar;
use uuid::Uuid;
use validator::Validate;

use super::contracts::{
    AdminResetPasswordPayload, ApiKeyResponse, ChangeUserRolePayload, CheckSlugQuery,
    CheckSlugResponse, CreateApiKeyPayload, CreateApiKeyResponse, GetMeResponse, InviteUserPayload,
    InviteUserResponse, LoginPayload, LoginResponse, LogoutPayload, RefreshTokenResponse,
    RegisterTenantPayload, RegisterTenantResponse, ResetPasswordResponse, SignupPayload,
    TenantBrandingQuery, TenantBrandingResponse, TenantMemberResponse, TenantResponse,
    UpdateTenantPayload, UpdateUserStatusPayload,
};
use crate::{
    app_state::AppState,
    error::ApiError,
    middleware::auth::{AdminUser, AuthUser},
    response::ApiResponse,
    utils::{
        cookies::{REFRESH_COOKIE_NAME, build_refresh_cookie, clear_refresh_cookie},
        jwt::{sign_access_token, sign_refresh_token, verify_jwt},
        secret::{generate_api_key, sha256_hex},
    },
};

use domain_identity::application::use_cases::{
    CheckSlugAvailabilityCommand, CreateApiKeyCommand, GetTenantBySlugCommand,
    IssueRefreshTokenCommand, ListApiKeysCommand, LoginCommand, LogoutCommand,
    RefreshSessionCommand, ResetPasswordCommand, RevokeApiKeyCommand,
    SelfRegisterCustomerCommand, change_user_role::ChangeUserRoleCommand,
    get_tenant::GetTenantCommand, invite_user::InviteUserCommand, list_users::ListUsersCommand,
    register_tenant::RegisterTenantCommand, update_tenant::UpdateTenantCommand,
    update_user_status::UpdateUserStatusCommand,
};

// ─── Tenant branding (public) ─────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/identity/tenant/branding",
    tag = "Identity",
    params(("slug" = String, Query, description = "Tenant slug")),
    responses(
        (status = 200, description = "Branding público do tenant", body = TenantBrandingResponse),
        (status = 404, description = "Tenant não encontrado")
    )
)]
pub async fn get_tenant_branding_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<TenantBrandingQuery>,
) -> Result<(StatusCode, Json<ApiResponse<TenantBrandingResponse>>), ApiError> {
    let tenant = state
        .identity
        .get_tenant_by_slug
        .execute(GetTenantBySlugCommand { slug: query.slug })
        .await
        .map_err(ApiError::from)?;

    Ok((StatusCode::OK, Json(ApiResponse::success(tenant.into()))))
}

// ─── Me ───────────────────────────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/identity/me",
    tag = "Identity",
    responses(
        (status = 200, description = "Dados do usuário autenticado", body = GetMeResponse),
        (status = 401, description = "Token ausente, inválido ou expirado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_me_handler(
    AuthUser(claims): AuthUser,
) -> Result<(StatusCode, Json<ApiResponse<GetMeResponse>>), ApiError> {
    tracing::info!(user_id = %claims.sub, tenant_id = %claims.tenant_id, "GET /me");
    Ok((StatusCode::OK, Json(ApiResponse::success(claims.into()))))
}

// ─── Register ─────────────────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/register",
    tag = "Identity",
    request_body = RegisterTenantPayload,
    responses(
        (status = 201, description = "Empresa e admin criados", body = RegisterTenantResponse),
        (status = 400, description = "Erro de validação"),
        (status = 409, description = "E-mail já em uso")
    )
)]
pub async fn register_tenant_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterTenantPayload>,
) -> Result<(StatusCode, Json<ApiResponse<RegisterTenantResponse>>), ApiError> {
    payload.validate()?;

    let result = state
        .identity
        .register_tenant
        .execute(RegisterTenantCommand {
            tenant_name: payload.tenant_name,
            tenant_slug: payload.tenant_slug,
            admin_full_name: payload.admin_full_name,
            admin_email: payload.admin_email,
            admin_plain_password: payload.admin_password,
        })
        .await?;

    let resp: RegisterTenantResponse = result.into();
    tracing::info!(tenant_id = %resp.tenant_id, slug = %resp.tenant_slug, "tenant registered via API");
    Ok((StatusCode::CREATED, Json(ApiResponse::success(resp))))
}

// ─── Signup (customer self-service) ────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/signup",
    tag = "Identity",
    request_body = SignupPayload,
    responses(
        (status = 200, description = "Cliente cadastrado e autenticado — retorna JWT", body = LoginResponse),
        (status = 400, description = "Erro de validação"),
        (status = 404, description = "Workspace (tenant) não encontrado"),
        (status = 409, description = "E-mail já em uso")
    )
)]
pub async fn signup_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<SignupPayload>,
) -> Result<(CookieJar, Json<ApiResponse<LoginResponse>>), ApiError> {
    payload.validate()?;

    let (tenant, user) = state
        .identity
        .self_register_customer
        .execute(SelfRegisterCustomerCommand {
            tenant_slug: payload.slug,
            email: payload.email,
            full_name: payload.full_name,
            plain_password: payload.password,
        })
        .await?;

    // Newly-created customers are always `Role::Customer` within this tenant.
    let role = domain_identity::domain::entities::Role::Customer;

    let access = sign_access_token(
        user.id,
        tenant.id,
        role.clone(),
        &state.config.jwt_secret,
        &state.config.jwt_issuer,
        state.config.access_token_ttl_minutes,
    )
    .map_err(|e| ApiError::Internal(format!("Falha ao assinar access token: {e}")))?;

    let refresh = sign_refresh_token(
        user.id,
        tenant.id,
        role.clone(),
        &state.config.jwt_secret,
        &state.config.jwt_issuer,
        state.config.refresh_token_ttl_days,
    )
    .map_err(|e| ApiError::Internal(format!("Falha ao assinar refresh token: {e}")))?;

    state
        .identity
        .issue_refresh_token
        .execute(IssueRefreshTokenCommand {
            jti: refresh.jti,
            user_id: user.id,
            tenant_id: tenant.id,
            token_hash: sha256_hex(&refresh.value),
            expires_at: refresh.expires_at,
        })
        .await?;

    let access_ttl_secs = (state.config.access_token_ttl_minutes as i64) * 60;

    tracing::info!(user_id = %user.id, tenant_id = %tenant.id, "customer signup + login via API");

    let updated_jar = jar.add(build_refresh_cookie(refresh.value, &state.config));

    Ok((
        updated_jar,
        Json(ApiResponse::success(LoginResponse {
            access_token: access.value,
            access_token_expires_in: access_ttl_secs,
            user_id: user.id,
            tenant_id: tenant.id,
            role,
        })),
    ))
}

// ─── Check slug availability ─────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/identity/check-slug",
    tag = "Identity",
    params(("slug" = String, Query, description = "Candidate slug to check")),
    responses(
        (status = 200, description = "Disponibilidade do slug", body = CheckSlugResponse),
    )
)]
pub async fn check_slug_handler(
    State(state): State<AppState>,
    axum::extract::Query(query): axum::extract::Query<CheckSlugQuery>,
) -> Result<(StatusCode, Json<ApiResponse<CheckSlugResponse>>), ApiError> {
    let availability = state
        .identity
        .check_slug
        .execute(CheckSlugAvailabilityCommand { slug: query.slug })
        .await?;

    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(CheckSlugResponse {
            slug: availability.slug,
            available: availability.available,
            reason: availability.reason,
        })),
    ))
}

// ─── Login ────────────────────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/login",
    tag = "Identity",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Login efetuado — retorna JWT", body = LoginResponse),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Credenciais inválidas")
    )
)]
pub async fn login_handler(
    State(state): State<AppState>,
    jar: CookieJar,
    Json(payload): Json<LoginPayload>,
) -> Result<(CookieJar, Json<ApiResponse<LoginResponse>>), ApiError> {
    payload.validate()?;

    let (user, tenant_user) = state
        .identity
        .login
        .execute(LoginCommand {
            email: payload.email,
            plain_password: payload.password,
        })
        .await?;

    let access = sign_access_token(
        user.id,
        tenant_user.tenant_id,
        tenant_user.role.clone(),
        &state.config.jwt_secret,
        &state.config.jwt_issuer,
        state.config.access_token_ttl_minutes,
    )
    .map_err(|e| ApiError::Internal(format!("Falha ao assinar access token: {e}")))?;

    let refresh = sign_refresh_token(
        user.id,
        tenant_user.tenant_id,
        tenant_user.role.clone(),
        &state.config.jwt_secret,
        &state.config.jwt_issuer,
        state.config.refresh_token_ttl_days,
    )
    .map_err(|e| ApiError::Internal(format!("Falha ao assinar refresh token: {e}")))?;

    state
        .identity
        .issue_refresh_token
        .execute(IssueRefreshTokenCommand {
            jti: refresh.jti,
            user_id: user.id,
            tenant_id: tenant_user.tenant_id,
            token_hash: sha256_hex(&refresh.value),
            expires_at: refresh.expires_at,
        })
        .await?;

    let access_ttl_secs = (state.config.access_token_ttl_minutes as i64) * 60;

    tracing::info!(user_id = %user.id, tenant_id = %tenant_user.tenant_id, "login successful via API");

    let updated_jar = jar.add(build_refresh_cookie(refresh.value, &state.config));

    Ok((
        updated_jar,
        Json(ApiResponse::success(LoginResponse {
            access_token: access.value,
            access_token_expires_in: access_ttl_secs,
            user_id: user.id,
            tenant_id: tenant_user.tenant_id,
            role: tenant_user.role,
        })),
    ))
}

// ─── Refresh ─────────────────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/refresh",
    tag = "Identity",
    responses(
        (status = 200, description = "Access token rotacionado (refresh token continua no cookie httpOnly)", body = RefreshTokenResponse),
        (status = 401, description = "Cookie ausente, refresh token inválido, expirado ou revogado")
    )
)]
pub async fn refresh_token_handler(
    State(state): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Json<ApiResponse<RefreshTokenResponse>>), ApiError> {
    let presented = jar
        .get(REFRESH_COOKIE_NAME)
        .map(|c| c.value().to_string())
        .ok_or_else(|| {
            ApiError::Identity(domain_identity::domain::error::DomainError::InvalidCredentials)
        })?;

    let claims = verify_jwt(
        &presented,
        &state.config.jwt_secret,
        &state.config.jwt_issuer,
    )
    .map_err(|_| {
        ApiError::Identity(domain_identity::domain::error::DomainError::InvalidCredentials)
    })?;

    let new_refresh = sign_refresh_token(
        claims.sub,
        claims.tenant_id,
        claims.role.clone(),
        &state.config.jwt_secret,
        &state.config.jwt_issuer,
        state.config.refresh_token_ttl_days,
    )
    .map_err(|e| ApiError::Internal(format!("Falha ao assinar refresh token: {e}")))?;

    let result = state
        .identity
        .refresh_session
        .execute(RefreshSessionCommand {
            presented_jti: claims.jti,
            presented_token_hash: sha256_hex(&presented),
            new_jti: new_refresh.jti,
            new_token_hash: sha256_hex(&new_refresh.value),
            new_expires_at: new_refresh.expires_at,
        })
        .await?;

    let access = sign_access_token(
        result.user.id,
        result.tenant_user.tenant_id,
        result.tenant_user.role,
        &state.config.jwt_secret,
        &state.config.jwt_issuer,
        state.config.access_token_ttl_minutes,
    )
    .map_err(|e| ApiError::Internal(format!("Falha ao assinar access token: {e}")))?;

    let updated_jar = jar.add(build_refresh_cookie(new_refresh.value, &state.config));

    Ok((
        updated_jar,
        Json(ApiResponse::success(RefreshTokenResponse {
            access_token: access.value,
            access_token_expires_in: (state.config.access_token_ttl_minutes as i64) * 60,
        })),
    ))
}

// ─── Logout ──────────────────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/logout",
    tag = "Identity",
    request_body = LogoutPayload,
    responses(
        (status = 204, description = "Sessão revogada"),
        (status = 401, description = "Não autenticado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn logout_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    jar: CookieJar,
    payload: Option<Json<LogoutPayload>>,
) -> Result<(CookieJar, StatusCode), ApiError> {
    let everywhere = payload.map(|p| p.0.everywhere).unwrap_or(false);

    // Resolve which session to revoke: the JTI from the current cookie (single-device logout)
    // OR all sessions when `everywhere=true`.
    let refresh_jti = jar.get(REFRESH_COOKIE_NAME).and_then(|c| {
        verify_jwt(
            c.value(),
            &state.config.jwt_secret,
            &state.config.jwt_issuer,
        )
        .ok()
        .filter(|claims_| claims_.sub == claims.sub)
        .map(|claims_| claims_.jti)
    });

    state
        .identity
        .logout
        .execute(LogoutCommand {
            refresh_jti,
            user_id: claims.sub,
            revoke_all: everywhere,
        })
        .await?;

    tracing::info!(user_id = %claims.sub, everywhere, "logout via API");

    let updated_jar = jar.add(clear_refresh_cookie(&state.config));
    Ok((updated_jar, StatusCode::NO_CONTENT))
}

// ─── API keys ────────────────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/api-keys",
    tag = "Identity",
    request_body = CreateApiKeyPayload,
    responses(
        (status = 201, description = "Chave criada — copie `plaintext`, ele não será exibido novamente", body = CreateApiKeyResponse),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Não autenticado"),
        (status = 403, description = "Somente admin")
    ),
    security(("bearer_auth" = []))
)]
pub async fn create_api_key_handler(
    State(state): State<AppState>,
    AdminUser(claims): AdminUser,
    Json(payload): Json<CreateApiKeyPayload>,
) -> Result<(StatusCode, Json<ApiResponse<CreateApiKeyResponse>>), ApiError> {
    payload.validate()?;

    let role = payload
        .role
        .parse::<domain_identity::domain::entities::Role>()
        .map_err(|_| {
            ApiError::Identity(domain_identity::domain::error::DomainError::InvalidRole(
                payload.role.clone(),
            ))
        })?;

    let expires_at = payload
        .expires_in_days
        .map(|days| time::OffsetDateTime::now_utc() + time::Duration::days(days as i64));

    let (plaintext, prefix, hash) = generate_api_key();

    let api_key = state
        .identity
        .create_api_key
        .execute(CreateApiKeyCommand {
            tenant_id: claims.tenant_id,
            created_by: claims.sub,
            name: payload.name,
            role,
            key_prefix: prefix.clone(),
            key_hash: hash,
            expires_at,
        })
        .await?;

    tracing::info!(
        api_key_id = %api_key.id,
        tenant_id = %api_key.tenant_id,
        created_by = %claims.sub,
        "api key created"
    );

    Ok((
        StatusCode::CREATED,
        Json(ApiResponse::success(CreateApiKeyResponse {
            id: api_key.id,
            name: api_key.name,
            key_prefix: prefix,
            role: api_key.role,
            plaintext,
            expires_at: api_key.expires_at,
            created_at: api_key.created_at,
        })),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/identity/api-keys",
    tag = "Identity",
    responses(
        (status = 200, description = "Chaves do tenant (sem o segredo)", body = Vec<ApiKeyResponse>),
        (status = 401, description = "Não autenticado"),
        (status = 403, description = "Somente admin")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_api_keys_handler(
    State(state): State<AppState>,
    AdminUser(claims): AdminUser,
) -> Result<(StatusCode, Json<ApiResponse<Vec<ApiKeyResponse>>>), ApiError> {
    let keys = state
        .identity
        .list_api_keys
        .execute(ListApiKeysCommand {
            tenant_id: claims.tenant_id,
        })
        .await?;

    let body: Vec<ApiKeyResponse> = keys.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(ApiResponse::success(body))))
}

#[utoipa::path(
    delete, path = "/api/v1/identity/api-keys/{id}",
    tag = "Identity",
    params(("id" = Uuid, Path, description = "ID da API key")),
    responses(
        (status = 204, description = "Chave revogada"),
        (status = 401, description = "Não autenticado"),
        (status = 403, description = "Somente admin"),
        (status = 404, description = "Chave não encontrada")
    ),
    security(("bearer_auth" = []))
)]
pub async fn revoke_api_key_handler(
    State(state): State<AppState>,
    AdminUser(claims): AdminUser,
    Path(api_key_id): Path<Uuid>,
) -> Result<StatusCode, ApiError> {
    state
        .identity
        .revoke_api_key
        .execute(RevokeApiKeyCommand {
            api_key_id,
            tenant_id: claims.tenant_id,
        })
        .await?;

    tracing::info!(api_key_id = %api_key_id, admin = %claims.sub, "api key revoked");
    Ok(StatusCode::NO_CONTENT)
}

// ─── Admin reset password ─────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/admin/users/{id}/unlock-and-reset",
    tag = "Identity",
    request_body = AdminResetPasswordPayload,
    params(("id" = Uuid, Path, description = "ID do usuário alvo")),
    responses(
        (status = 200, description = "Senha resetada com sucesso", body = ResetPasswordResponse),
        (status = 401, description = "Não autenticado"),
        (status = 403, description = "Acesso negado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn admin_reset_user_password_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
    axum::extract::Path(target_user_id): axum::extract::Path<Uuid>,
    Json(payload): Json<AdminResetPasswordPayload>,
) -> Result<(StatusCode, Json<ApiResponse<ResetPasswordResponse>>), ApiError> {
    payload.validate()?;

    state
        .identity
        .reset_password
        .execute(ResetPasswordCommand {
            target_user_id,
            operator_tenant_id: admin_claims.tenant_id,
            is_admin_override: true,
            new_plain_password: payload.temporary_password,
        })
        .await?;

    tracing::info!(
        admin_id = %admin_claims.sub,
        target_user_id = %target_user_id,
        "admin reset password via API"
    );
    Ok((
        StatusCode::OK,
        Json(ApiResponse::success(ResetPasswordResponse {
            message:
                "Usuário desbloqueado e credenciais atualizadas pelo administrador com sucesso."
                    .to_string(),
        })),
    ))
}

// ─── Invite user ─────────────────────────────────────────────────────────────

#[utoipa::path(
    post, path = "/api/v1/identity/users",
    tag = "Identity",
    request_body = InviteUserPayload,
    responses(
        (status = 201, description = "Usuário convidado", body = InviteUserResponse),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Somente admin"),
        (status = 409, description = "E-mail já em uso")
    ),
    security(("bearer_auth" = []))
)]
pub async fn invite_user_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
    Json(payload): Json<InviteUserPayload>,
) -> Result<(StatusCode, Json<ApiResponse<InviteUserResponse>>), ApiError> {
    payload.validate()?;

    let role = payload
        .role
        .parse::<domain_identity::domain::entities::Role>()
        .map_err(|_| {
            ApiError::Identity(domain_identity::domain::error::DomainError::InvalidRole(
                payload.role.clone(),
            ))
        })?;

    let user = state
        .identity
        .invite_user
        .execute(InviteUserCommand {
            operator_tenant_id: admin_claims.tenant_id,
            email: payload.email,
            full_name: payload.full_name,
            role,
            temporary_password: payload.temporary_password,
        })
        .await?;

    let resp: InviteUserResponse = user.into();
    tracing::info!(admin_id = %admin_claims.sub, user_id = %resp.user_id, "user invited via API");
    Ok((StatusCode::CREATED, Json(ApiResponse::success(resp))))
}

// ─── List users ───────────────────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/identity/users",
    tag = "Identity",
    responses(
        (status = 200, description = "Membros do tenant", body = Vec<TenantMemberResponse>),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Somente admin")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_users_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
) -> Result<(StatusCode, Json<ApiResponse<Vec<TenantMemberResponse>>>), ApiError> {
    let members = state
        .identity
        .list_users
        .execute(ListUsersCommand {
            tenant_id: admin_claims.tenant_id,
        })
        .await?;

    let body: Vec<TenantMemberResponse> = members.into_iter().map(Into::into).collect();
    Ok((StatusCode::OK, Json(ApiResponse::success(body))))
}

// ─── Change role ──────────────────────────────────────────────────────────────

#[utoipa::path(
    patch, path = "/api/v1/identity/users/{id}/role",
    tag = "Identity",
    request_body = ChangeUserRolePayload,
    params(("id" = Uuid, Path, description = "ID do usuário")),
    responses(
        (status = 204, description = "Role atualizado"),
        (status = 400, description = "Role inválido"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Somente admin"),
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

    state
        .identity
        .change_user_role
        .execute(ChangeUserRoleCommand {
            operator_tenant_id: admin_claims.tenant_id,
            target_user_id,
            new_role: role,
        })
        .await?;

    tracing::info!(admin_id = %admin_claims.sub, target_user_id = %target_user_id, "role changed via API");
    Ok(StatusCode::NO_CONTENT)
}

// ─── Update status ────────────────────────────────────────────────────────────

#[utoipa::path(
    patch, path = "/api/v1/identity/users/{id}/status",
    tag = "Identity",
    request_body = UpdateUserStatusPayload,
    params(("id" = Uuid, Path, description = "ID do usuário")),
    responses(
        (status = 204, description = "Status atualizado"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Somente admin"),
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
    state
        .identity
        .update_user_status
        .execute(UpdateUserStatusCommand {
            operator_tenant_id: admin_claims.tenant_id,
            target_user_id,
            active: payload.active,
        })
        .await?;

    tracing::info!(admin_id = %admin_claims.sub, target_user_id = %target_user_id, active = payload.active, "user status changed via API");
    Ok(StatusCode::NO_CONTENT)
}

// ─── Tenant info ──────────────────────────────────────────────────────────────

#[utoipa::path(
    get, path = "/api/v1/identity/tenant",
    tag = "Identity",
    responses(
        (status = 200, description = "Informações do tenant", body = TenantResponse),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Somente admin")
    ),
    security(("bearer_auth" = []))
)]
pub async fn get_tenant_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
) -> Result<(StatusCode, Json<ApiResponse<TenantResponse>>), ApiError> {
    let tenant = state
        .identity
        .get_tenant
        .execute(GetTenantCommand {
            tenant_id: admin_claims.tenant_id,
        })
        .await?;

    Ok((StatusCode::OK, Json(ApiResponse::success(tenant.into()))))
}

#[utoipa::path(
    patch, path = "/api/v1/identity/tenant",
    tag = "Identity",
    request_body = UpdateTenantPayload,
    responses(
        (status = 200, description = "Tenant atualizado", body = TenantResponse),
        (status = 400, description = "Erro de validação"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Somente admin"),
        (status = 404, description = "Tenant não encontrado")
    ),
    security(("bearer_auth" = []))
)]
pub async fn update_tenant_handler(
    State(state): State<AppState>,
    AdminUser(admin_claims): AdminUser,
    Json(payload): Json<UpdateTenantPayload>,
) -> Result<(StatusCode, Json<ApiResponse<TenantResponse>>), ApiError> {
    payload.validate()?;

    let tenant_updated = state
        .identity
        .update_tenant
        .execute(UpdateTenantCommand {
            tenant_id: admin_claims.tenant_id,
            new_name: payload.name,
            new_description: payload.description,
            new_theme: payload.theme,
        })
        .await?;

    tracing::info!(tenant_id = %tenant_updated.id, "tenant updated via API");
    Ok((StatusCode::OK, Json(ApiResponse::success(tenant_updated.into()))))
}