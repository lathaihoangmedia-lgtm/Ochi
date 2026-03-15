//! Embedding Providers - Generate vector embeddings for text
//!
//! Supports:
//! - Noop (placeholder, 384-dim zeros)
//! - Mock (deterministic hash-based embeddings)
//! - OpenAI API (text-embedding-3-small)
//! - Custom HTTP endpoint

use serde::{Deserialize, Serialize};

/// Embedding model types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmbeddingModel {
    Noop,
    Mock,
    OpenAI(String),  // model name
    Custom(String),  // URL
}

impl Default for EmbeddingModel {
    fn default() -> Self {
        EmbeddingModel::Noop
    }
}

/// Embedding provider trait
pub trait EmbeddingProvider: Send + Sync {
    /// Generate embedding for text
    fn embed(&self, text: &str) -> Result<Vec<f32>, String>;
    
    /// Get embedding dimensions
    fn dimensions(&self) -> usize;
    
    /// Get model name
    fn model_name(&self) -> &str;
}

/// Noop embedding provider (zeros)
pub struct NoopEmbedding {
    dimensions: usize,
}

impl NoopEmbedding {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
}

impl EmbeddingProvider for NoopEmbedding {
    fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        // Return zeros (placeholder)
        Ok(vec![0.0f32; self.dimensions])
    }
    
    fn dimensions(&self) -> usize {
        self.dimensions
    }
    
    fn model_name(&self) -> &str {
        "noop"
    }
}

/// Mock embedding provider (hash-based, deterministic)
pub struct MockEmbedding {
    dimensions: usize,
}

impl MockEmbedding {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
    
    /// Simple hash-based embedding (for testing)
    fn hash_embedding(&self, text: &str) -> Vec<f32> {
        let mut embedding = vec![0.0f32; self.dimensions];
        
        // Simple character-based hash (NOT real embeddings, just for testing)
        for (i, ch) in text.chars().enumerate() {
            let idx = (ch as usize + i) % self.dimensions;
            embedding[idx] += 1.0 / (i as f32 + 1.0);
        }
        
        // Normalize
        let norm: f32 = embedding.iter().map(|x| x * x).sum::<f32>().sqrt();
        if norm > 0.0 {
            for x in embedding.iter_mut() {
                *x /= norm;
            }
        }
        
        embedding
    }
}

impl EmbeddingProvider for MockEmbedding {
    fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        Ok(self.hash_embedding(text))
    }
    
    fn dimensions(&self) -> usize {
        self.dimensions
    }
    
    fn model_name(&self) -> &str {
        "mock"
    }
}

/// OpenAI embedding provider
pub struct OpenAIEmbedding {
    api_key: String,
    model: String,
    dimensions: usize,
    client: reqwest::blocking::Client,
}

impl OpenAIEmbedding {
    pub fn new(api_key: String, model: Option<String>) -> Self {
        let model = model.unwrap_or_else(|| "text-embedding-3-small".to_string());
        let dimensions = if model.contains("3-small") { 1536 } else { 1536 };
        
        Self {
            api_key,
            model,
            dimensions,
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl EmbeddingProvider for OpenAIEmbedding {
    fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        #[derive(Serialize)]
        struct EmbeddingRequest {
            model: String,
            input: String,
        }
        
        #[derive(Deserialize)]
        struct EmbeddingResponse {
            data: Vec<EmbeddingData>,
        }
        
        #[derive(Deserialize)]
        struct EmbeddingData {
            embedding: Vec<f32>,
        }
        
        let request = EmbeddingRequest {
            model: self.model.clone(),
            input: text.to_string(),
        };
        
        let response = self.client
            .post("https://api.openai.com/v1/embeddings")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| format!("OpenAI API error: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("OpenAI API error: {}", response.status()));
        }
        
        let result: EmbeddingResponse = response
            .json()
            .map_err(|e| format!("JSON parse error: {}", e))?;
        
        result.data
            .first()
            .map(|d| d.embedding.clone())
            .ok_or_else(|| "No embedding returned".to_string())
    }
    
    fn dimensions(&self) -> usize {
        self.dimensions
    }
    
    fn model_name(&self) -> &str {
        &self.model
    }
}

/// Custom HTTP embedding provider
pub struct CustomEmbedding {
    url: String,
    dimensions: usize,
    client: reqwest::blocking::Client,
}

impl CustomEmbedding {
    pub fn new(url: String, dimensions: usize) -> Self {
        Self {
            url,
            dimensions,
            client: reqwest::blocking::Client::new(),
        }
    }
}

impl EmbeddingProvider for CustomEmbedding {
    fn embed(&self, text: &str) -> Result<Vec<f32>, String> {
        #[derive(Serialize)]
        struct EmbeddingRequest {
            text: String,
        }
        
        let request = EmbeddingRequest {
            text: text.to_string(),
        };
        
        let response = self.client
            .post(&self.url)
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .map_err(|e| format!("Custom API error: {}", e))?;
        
        if !response.status().is_success() {
            return Err(format!("Custom API error: {}", response.status()));
        }
        
        let embedding: Vec<f32> = response
            .json()
            .map_err(|e| format!("JSON parse error: {}", e))?;
        
        Ok(embedding)
    }
    
    fn dimensions(&self) -> usize {
        self.dimensions
    }
    
    fn model_name(&self) -> &str {
        "custom"
    }
}

/// Create embedding provider from config
pub fn create_provider(model: &EmbeddingModel, api_key: Option<&str>) -> Box<dyn EmbeddingProvider> {
    match model {
        EmbeddingModel::Noop => Box::new(NoopEmbedding::new(384)),
        EmbeddingModel::Mock => Box::new(MockEmbedding::new(384)),
        EmbeddingModel::OpenAI(model_name) => {
            let key = api_key.unwrap_or("").to_string();
            Box::new(OpenAIEmbedding::new(key, Some(model_name.clone())))
        }
        EmbeddingModel::Custom(url) => {
            Box::new(CustomEmbedding::new(url.clone(), 384))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_noop_embedding() {
        let provider = NoopEmbedding::new(384);
        let embedding = provider.embed("hello world").unwrap();
        assert_eq!(embedding.len(), 384);
        assert!(embedding.iter().all(|&x| x == 0.0));
    }
    
    #[test]
    fn test_mock_embedding() {
        let provider = MockEmbedding::new(384);
        let embedding1 = provider.embed("hello world").unwrap();
        let embedding2 = provider.embed("hello world").unwrap();
        let embedding3 = provider.embed("different text").unwrap();
        
        // Deterministic
        assert_eq!(embedding1, embedding2);
        
        // Different texts produce different embeddings
        assert_ne!(embedding1, embedding3);
        
        // Normalized (approximately unit length)
        let norm: f32 = embedding1.iter().map(|x| x * x).sum();
        assert!((norm - 1.0).abs() < 0.01);
    }
}
