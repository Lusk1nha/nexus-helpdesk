use axum::{Json, extract::State, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use domain_identity::domain::entities::role::Role;

use crate::{
    app_state::AppState, error::ApiError, middleware::auth::AuthUser, response::ApiResponse,
};

use super::contracts::{IngestKnowledgePayload, IngestKnowledgeResponse};

#[utoipa::path(
    post, path = "/api/v1/knowledge",
    request_body = IngestKnowledgePayload,
    responses(
        (status = 202, description = "Documento indexado com sucesso", body = IngestKnowledgeResponse),
        (status = 400, description = "Conteúdo inválido"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado — apenas agentes e admins")
    ),
    security(("bearer_auth" = []))
)]
pub async fn ingest_knowledge_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Json(payload): Json<IngestKnowledgePayload>,
) -> Result<(StatusCode, Json<ApiResponse<IngestKnowledgeResponse>>), ApiError> {
    match claims.role {
        Role::Agent | Role::Admin => {}
        _ => {
            return Err(ApiError::Internal(
                "Acesso negado — apenas agentes e admins podem indexar documentos.".to_string(),
            ));
        }
    }

    payload.validate().map_err(ApiError::Validation)?;

    state
        .ai_engine
        .index_document(
            &payload.content,
            claims.tenant_id,
            Uuid::new_v4(),
            "knowledge_article",
        )
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    tracing::info!(
        user_id = %claims.sub,
        tenant_id = %claims.tenant_id,
        "knowledge article indexed in Qdrant"
    );

    Ok((
        StatusCode::ACCEPTED,
        Json(ApiResponse::success(IngestKnowledgeResponse {
            message: "Documento indexado com sucesso.".to_string(),
        })),
    ))
}
