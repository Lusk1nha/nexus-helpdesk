use thiserror::Error;

#[derive(Error, Debug)]
pub enum AiEngineError {
    #[error("Embedding request failed: {0}")]
    EmbeddingRequest(String),

    #[error("Unexpected embedding response: {0}")]
    EmbeddingParse(String),

    #[error("Qdrant operation failed: {0}")]
    VectorStore(String),

    #[error("Qdrant collection setup failed: {0}")]
    CollectionSetup(String),
}

impl From<qdrant_client::QdrantError> for AiEngineError {
    fn from(e: qdrant_client::QdrantError) -> Self {
        AiEngineError::VectorStore(e.to_string())
    }
}
