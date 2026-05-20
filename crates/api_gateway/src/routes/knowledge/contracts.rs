use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IngestKnowledgePayload {
    #[validate(length(min = 3, message = "O título deve ter pelo menos 3 caracteres."))]
    #[schema(example = "Como redefinir a senha")]
    pub title: String,

    #[validate(length(min = 10, message = "O conteúdo deve ter pelo menos 10 caracteres."))]
    #[schema(
        example = "Para redefinir a senha, acesse Configurações > Segurança > Alterar Senha e siga as instruções."
    )]
    pub content: String,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IngestKnowledgeResponse {
    pub document_id: String,
    pub message: String,
}

#[derive(Deserialize, IntoParams)]
pub struct ListKnowledgeQuery {
    #[param(example = 50)]
    pub limit: Option<u32>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct KnowledgeDocumentResponse {
    pub id: String,
    pub title: String,
    pub content_preview: String,
    pub document_type: String,
    pub source_ticket_id: String,
    pub indexed_at: u64,
    pub indexed_by: String,
}

#[derive(Serialize, ToSchema)]
pub struct ListKnowledgeResponse {
    pub items: Vec<KnowledgeDocumentResponse>,
    pub count: usize,
}

#[derive(Deserialize, IntoParams)]
pub struct SearchKnowledgeQuery {
    #[param(example = "como redefinir senha")]
    pub q: String,
    #[param(example = 5)]
    pub limit: Option<u64>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultItem {
    pub content: String,
    pub document_type: String,
    pub score: f32,
}

#[derive(Serialize, ToSchema)]
pub struct SearchKnowledgeResponse {
    pub query: String,
    pub results: Vec<SearchResultItem>,
}
