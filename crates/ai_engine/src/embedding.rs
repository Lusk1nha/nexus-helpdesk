use serde::{Deserialize, Serialize};

use crate::error::AiEngineError;

#[derive(Serialize)]
struct EmbeddingRequest<'a> {
    model: &'a str,
    prompt: &'a str,
}

#[derive(Deserialize)]
struct EmbeddingResponse {
    embedding: Vec<f32>,
}

pub struct OllamaEmbeddingService {
    client: reqwest::Client,
    ollama_url: String,
}

impl OllamaEmbeddingService {
    pub fn new(client: reqwest::Client, ollama_url: String) -> Self {
        Self { client, ollama_url }
    }

    pub async fn embed(&self, text: &str) -> Result<Vec<f32>, AiEngineError> {
        let url = format!("{}/api/embeddings", self.ollama_url);
        let body = EmbeddingRequest {
            model: "nomic-embed-text",
            prompt: text,
        };

        let response = self
            .client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| AiEngineError::EmbeddingRequest(e.to_string()))?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            return Err(AiEngineError::EmbeddingRequest(format!(
                "HTTP {status}: {text}"
            )));
        }

        let parsed: EmbeddingResponse = response
            .json()
            .await
            .map_err(|e| AiEngineError::EmbeddingParse(e.to_string()))?;

        Ok(parsed.embedding)
    }
}
