use std::collections::HashMap;

use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, Distance, Filter, PointStruct, QueryPointsBuilder,
    UpsertPointsBuilder, Value, VectorParamsBuilder,
};
use qdrant_client::Qdrant;
use uuid::Uuid;

use crate::error::AiEngineError;

pub const COLLECTION_NAME: &str = "nexus_helpdesk";
pub const VECTOR_SIZE: u64 = 768;

pub struct RetrievedDocument {
    pub content: String,
    pub score: f32,
    pub document_type: String,
}

pub struct QdrantVectorStore {
    client: Qdrant,
}

impl QdrantVectorStore {
    pub fn new(qdrant_url: &str) -> Result<Self, AiEngineError> {
        let client = Qdrant::from_url(qdrant_url)
            .build()
            .map_err(|e| AiEngineError::CollectionSetup(e.to_string()))?;
        Ok(Self { client })
    }

    pub async fn ensure_collection(&self) -> Result<(), AiEngineError> {
        if self
            .client
            .collection_exists(COLLECTION_NAME)
            .await
            .map_err(|e| AiEngineError::CollectionSetup(e.to_string()))?
        {
            tracing::info!(
                collection = COLLECTION_NAME,
                "Qdrant collection already exists"
            );
            return Ok(());
        }

        self.client
            .create_collection(
                CreateCollectionBuilder::new(COLLECTION_NAME)
                    .vectors_config(VectorParamsBuilder::new(VECTOR_SIZE, Distance::Cosine)),
            )
            .await
            .map_err(|e| AiEngineError::CollectionSetup(e.to_string()))?;

        tracing::info!(
            collection = COLLECTION_NAME,
            vector_size = VECTOR_SIZE,
            "Qdrant collection created"
        );

        Ok(())
    }

    pub async fn upsert(
        &self,
        vector: Vec<f32>,
        content: &str,
        tenant_id: Uuid,
        source_ticket_id: Uuid,
        document_type: &str,
    ) -> Result<(), AiEngineError> {
        let point_id = Uuid::new_v4().to_string();

        let mut payload: HashMap<String, Value> = HashMap::new();

        payload.insert("content".to_string(), content.into());
        payload.insert("tenant_id".to_string(), tenant_id.to_string().into());
        payload.insert(
            "source_ticket_id".to_string(),
            source_ticket_id.to_string().into(),
        );
        payload.insert("document_type".to_string(), document_type.into());

        let point = PointStruct::new(point_id, vector, payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, vec![point]).wait(true))
            .await
            .map_err(AiEngineError::from)?;

        tracing::debug!(
            tenant_id = %tenant_id,
            source_ticket_id = %source_ticket_id,
            document_type,
            "document indexed in Qdrant"
        );
        Ok(())
    }

    pub async fn search(
        &self,
        query_vector: Vec<f32>,
        tenant_id: Uuid,
        limit: u64,
    ) -> Result<Vec<RetrievedDocument>, AiEngineError> {
        let tenant_filter = Filter::must([Condition::matches("tenant_id", tenant_id.to_string())]);

        let results = self
            .client
            .query(
                QueryPointsBuilder::new(COLLECTION_NAME)
                    .query(query_vector)
                    .filter(tenant_filter)
                    .limit(limit)
                    .with_payload(true),
            )
            .await
            .map_err(AiEngineError::from)?;

        let docs = results
            .result
            .into_iter()
            .map(|hit| {
                let content = hit
                    .payload
                    .get("content")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_default();

                let document_type = hit
                    .payload
                    .get("document_type")
                    .and_then(|v| v.as_str())
                    .map(String::from)
                    .unwrap_or_default();

                RetrievedDocument {
                    content,
                    score: hit.score,
                    document_type,
                }
            })
            .collect();

        Ok(docs)
    }
}
