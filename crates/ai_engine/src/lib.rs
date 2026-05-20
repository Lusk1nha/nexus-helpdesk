pub mod embedding;
pub mod error;
pub mod vector_store;

pub use error::AiEngineError;
pub use vector_store::{KnowledgeEntry, RetrievedDocument};

use std::sync::Arc;
use uuid::Uuid;

use embedding::OllamaEmbeddingService;
use vector_store::QdrantVectorStore;

#[derive(Clone)]
pub struct AiEngine {
    embedder: Arc<OllamaEmbeddingService>,
    store: Arc<QdrantVectorStore>,
}

impl AiEngine {
    pub fn new(
        http_client: reqwest::Client,
        ollama_url: String,
        qdrant_url: &str,
    ) -> Result<Self, AiEngineError> {
        let embedder = Arc::new(OllamaEmbeddingService::new(http_client, ollama_url));
        let store = Arc::new(QdrantVectorStore::new(qdrant_url)?);

        Ok(Self { embedder, store })
    }

    pub async fn initialize(&self) -> Result<(), AiEngineError> {
        self.store.ensure_collection().await
    }

    pub async fn retrieve_context(
        &self,
        text: &str,
        tenant_id: Uuid,
        limit: u64,
    ) -> Result<Vec<RetrievedDocument>, AiEngineError> {
        let vector = self.embedder.embed(text).await?;
        self.store.search(vector, tenant_id, limit).await
    }

    pub async fn index_document(
        &self,
        content: &str,
        title: &str,
        tenant_id: Uuid,
        source_ticket_id: Uuid,
        document_type: &str,
        indexed_by: &str,
    ) -> Result<String, AiEngineError> {
        let vector = self.embedder.embed(content).await?;
        self.store
            .upsert(vector, content, title, tenant_id, source_ticket_id, document_type, indexed_by)
            .await
    }

    pub async fn list_documents(
        &self,
        tenant_id: Uuid,
        limit: u32,
    ) -> Result<Vec<KnowledgeEntry>, AiEngineError> {
        self.store.list(tenant_id, limit).await
    }

    pub async fn delete_document(&self, point_id: &str) -> Result<(), AiEngineError> {
        self.store.delete(point_id).await
    }
}
