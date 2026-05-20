use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use qdrant_client::qdrant::point_id::PointIdOptions;
use qdrant_client::qdrant::PointId;
use qdrant_client::qdrant::{
    Condition, CreateCollectionBuilder, DeletePointsBuilder, Distance, Filter, PointStruct,
    PointsIdsList, QueryPointsBuilder, ScrollPointsBuilder, UpsertPointsBuilder, Value,
    VectorParamsBuilder,
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

pub struct KnowledgeEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub document_type: String,
    pub source_ticket_id: String,
    pub indexed_at: u64,
    pub indexed_by: String,
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
        title: &str,
        tenant_id: Uuid,
        source_ticket_id: Uuid,
        document_type: &str,
        indexed_by: &str,
    ) -> Result<String, AiEngineError> {
        let point_id = Uuid::new_v4().to_string();

        let indexed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let mut payload: HashMap<String, Value> = HashMap::new();
        payload.insert("content".to_string(), content.into());
        payload.insert("title".to_string(), title.into());
        payload.insert("tenant_id".to_string(), tenant_id.to_string().into());
        payload.insert(
            "source_ticket_id".to_string(),
            source_ticket_id.to_string().into(),
        );
        payload.insert("document_type".to_string(), document_type.into());
        payload.insert("indexed_at".to_string(), (indexed_at as i64).into());
        payload.insert("indexed_by".to_string(), indexed_by.into());

        let point = PointStruct::new(point_id.clone(), vector, payload);

        self.client
            .upsert_points(UpsertPointsBuilder::new(COLLECTION_NAME, vec![point]).wait(true))
            .await
            .map_err(AiEngineError::from)?;

        tracing::debug!(
            tenant_id = %tenant_id,
            source_ticket_id = %source_ticket_id,
            document_type,
            point_id = %point_id,
            "document indexed in Qdrant"
        );

        Ok(point_id)
    }

    pub async fn list(
        &self,
        tenant_id: Uuid,
        limit: u32,
    ) -> Result<Vec<KnowledgeEntry>, AiEngineError> {
        let tenant_filter = Filter::must([Condition::matches("tenant_id", tenant_id.to_string())]);

        let response = self
            .client
            .scroll(
                ScrollPointsBuilder::new(COLLECTION_NAME)
                    .filter(tenant_filter)
                    .limit(limit)
                    .with_payload(true),
            )
            .await
            .map_err(AiEngineError::from)?;

        let entries = response
            .result
            .into_iter()
            .map(|point| {
                let id = point
                    .id
                    .and_then(|pid| match pid.point_id_options {
                        Some(PointIdOptions::Uuid(s)) => Some(s),
                        Some(PointIdOptions::Num(n)) => Some(n.to_string()),
                        None => None,
                    })
                    .unwrap_or_default();

                let get_str = |key: &str| {
                    point
                        .payload
                        .get(key)
                        .and_then(|v| v.as_str())
                        .map(String::from)
                        .unwrap_or_default()
                };

                let indexed_at = point
                    .payload
                    .get("indexed_at")
                    .and_then(|v| v.as_integer())
                    .map(|n| n as u64)
                    .unwrap_or(0);

                KnowledgeEntry {
                    id,
                    title: get_str("title"),
                    content: get_str("content"),
                    document_type: get_str("document_type"),
                    source_ticket_id: get_str("source_ticket_id"),
                    indexed_at,
                    indexed_by: get_str("indexed_by"),
                }
            })
            .collect();

        Ok(entries)
    }

    pub async fn delete(&self, point_id: &str) -> Result<(), AiEngineError> {
        let pid = PointId {
            point_id_options: Some(PointIdOptions::Uuid(point_id.to_string())),
        };

        let ids_list: PointsIdsList = vec![pid].into();

        self.client
            .delete_points(
                DeletePointsBuilder::new(COLLECTION_NAME)
                    .points(ids_list)
                    .wait(true),
            )
            .await
            .map_err(AiEngineError::from)?;

        tracing::debug!(point_id, "document deleted from Qdrant");
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
