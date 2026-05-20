use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct IngestKnowledgePayload {
    #[validate(length(min = 10, message = "O conteúdo deve ter pelo menos 10 caracteres."))]
    #[schema(
        example = "Para redefinir a senha, acesse Configurações > Segurança > Alterar Senha e siga as instruções."
    )]
    pub content: String,
}

#[derive(Serialize, ToSchema)]
pub struct IngestKnowledgeResponse {
    pub message: String,
}
