use axum::{
    Json,
    extract::{Path, Query, State},
    http::StatusCode,
};
use validator::Validate;

use domain_identity::domain::entities::role::Role;

use crate::{
    app_state::AppState, error::ApiError, middleware::auth::AuthUser, response::ApiResponse,
};

use super::contracts::{
    IngestKnowledgePayload, IngestKnowledgeResponse, KnowledgeDocumentResponse, ListKnowledgeQuery,
    ListKnowledgeResponse, SearchKnowledgeQuery, SearchKnowledgeResponse, SearchResultItem,
};

#[utoipa::path(
    post, path = "/api/v1/knowledge",
    tag = "Knowledge",
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
            return Err(ApiError::Forbidden(
                "Apenas agentes e admins podem indexar documentos.".to_string(),
            ));
        }
    }

    payload.validate().map_err(ApiError::Validation)?;

    let document_id = state
        .ai_engine
        .index_document(
            &payload.content,
            &payload.title,
            claims.tenant_id,
            uuid::Uuid::new_v4(),
            "knowledge_article",
            &claims.sub.to_string(),
        )
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    tracing::info!(
        user_id = %claims.sub,
        tenant_id = %claims.tenant_id,
        document_id = %document_id,
        title = %payload.title,
        "knowledge article indexed"
    );

    Ok((
        StatusCode::ACCEPTED,
        Json(ApiResponse::success(IngestKnowledgeResponse {
            document_id,
            message: "Documento indexado com sucesso.".to_string(),
        })),
    ))
}

#[utoipa::path(
    get, path = "/api/v1/knowledge",
    tag = "Knowledge",
    params(ListKnowledgeQuery),
    responses(
        (status = 200, description = "Lista de documentos indexados", body = ListKnowledgeResponse),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado — apenas agentes e admins")
    ),
    security(("bearer_auth" = []))
)]
pub async fn list_knowledge_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Query(query): Query<ListKnowledgeQuery>,
) -> Result<Json<ApiResponse<ListKnowledgeResponse>>, ApiError> {
    match claims.role {
        Role::Agent | Role::Admin => {}
        _ => {
            return Err(ApiError::Forbidden(
                "Apenas agentes e admins podem visualizar a base de conhecimento.".to_string(),
            ));
        }
    }

    let limit = query.limit.unwrap_or(50).min(200);

    let entries = state
        .ai_engine
        .list_documents(claims.tenant_id, limit)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let items: Vec<KnowledgeDocumentResponse> = entries
        .into_iter()
        .map(|e| {
            let content_preview = if e.content.len() > 200 {
                format!("{}…", &e.content[..200])
            } else {
                e.content
            };

            KnowledgeDocumentResponse {
                id: e.id,
                title: e.title,
                content_preview,
                document_type: e.document_type,
                source_ticket_id: e.source_ticket_id,
                indexed_at: e.indexed_at,
                indexed_by: e.indexed_by,
            }
        })
        .collect();

    let count = items.len();

    Ok(Json(ApiResponse::success(ListKnowledgeResponse {
        items,
        count,
    })))
}

#[utoipa::path(
    delete, path = "/api/v1/knowledge/{id}",
    tag = "Knowledge",
    params(("id" = String, Path, description = "ID do documento a remover")),
    responses(
        (status = 200, description = "Documento removido com sucesso"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado — apenas admins")
    ),
    security(("bearer_auth" = []))
)]
pub async fn delete_knowledge_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Path(id): Path<String>,
) -> Result<Json<ApiResponse<serde_json::Value>>, ApiError> {
    if claims.role != Role::Admin {
        return Err(ApiError::Forbidden(
            "Apenas admins podem remover documentos da base de conhecimento.".to_string(),
        ));
    }

    state
        .ai_engine
        .delete_document(&id)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    tracing::info!(
        user_id = %claims.sub,
        tenant_id = %claims.tenant_id,
        document_id = %id,
        "knowledge document deleted"
    );

    Ok(Json(ApiResponse::success(serde_json::json!({
        "message": "Documento removido com sucesso."
    }))))
}

#[utoipa::path(
    get, path = "/api/v1/knowledge/search",
    tag = "Knowledge",
    params(SearchKnowledgeQuery),
    responses(
        (status = 200, description = "Resultados do RAG para a consulta", body = SearchKnowledgeResponse),
        (status = 400, description = "Parâmetro 'q' obrigatório"),
        (status = 401, description = "Não autorizado"),
        (status = 403, description = "Acesso negado — apenas agentes e admins")
    ),
    security(("bearer_auth" = []))
)]
pub async fn search_knowledge_handler(
    State(state): State<AppState>,
    AuthUser(claims): AuthUser,
    Query(query): Query<SearchKnowledgeQuery>,
) -> Result<Json<ApiResponse<SearchKnowledgeResponse>>, ApiError> {
    match claims.role {
        Role::Agent | Role::Admin => {}
        _ => {
            return Err(ApiError::Forbidden(
                "Apenas agentes e admins podem testar a base de conhecimento.".to_string(),
            ));
        }
    }

    if query.q.trim().is_empty() {
        return Err(ApiError::BadRequest(
            "O parâmetro 'q' não pode estar vazio.".to_string(),
        ));
    }

    let limit = query.limit.unwrap_or(5).min(20);

    let docs = state
        .ai_engine
        .retrieve_context(&query.q, claims.tenant_id, limit)
        .await
        .map_err(|e| ApiError::Internal(e.to_string()))?;

    let results = docs
        .into_iter()
        .map(|d| SearchResultItem {
            content: d.content,
            document_type: d.document_type,
            score: d.score,
        })
        .collect();

    Ok(Json(ApiResponse::success(SearchKnowledgeResponse {
        query: query.q,
        results,
    })))
}
