use serde::{Deserialize, Serialize};
use std::env;
use tracing::{debug, info};

#[derive(Debug, thiserror::Error)]
pub enum ManusError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Serialization error: {0}")]
    Parse(String),
    #[error("Config error: {0}")]
    Config(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManusTaskRequest {
    pub prompt: String,
    #[serde(rename = "agentProfile")]
    pub agent_profile: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManusTaskResponse {
    pub task_id: String,
    pub task_title: String,
    pub task_url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManusTaskDetail {
    pub task_id: String,
    pub task_title: String,
    pub status: String,
    pub message: Option<String>,
    pub task_url: String,
    pub attachments: Vec<ManusAttachment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManusAttachment {
    pub file_name: String,
    pub url: String,
    pub size_bytes: u64,
}

pub struct ManusClient {
    api_key: String,
    client: reqwest::Client,
}

impl ManusClient {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::Client::new(),
        }
    }

    pub fn from_env() -> Result<Self, ManusError> {
        let key = env::var("MANUS_API_KEY")
            .map_err(|_| ManusError::Config("MANUS_API_KEY not set".to_string()))?;
        Ok(Self::new(key))
    }

    /// Create a new task in Manus.
    pub async fn create_task(&self, prompt: &str) -> Result<ManusTaskResponse, ManusError> {
        let url = "https://api.manus.ai/v1/tasks";
        
        let request = ManusTaskRequest {
            prompt: prompt.to_string(),
            agent_profile: "manus-1.6".to_string(),
            task_id: None,
        };

        debug!(prompt = %prompt, "Creating Manus task");

        let resp = self.client
            .post(url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await
            .map_err(|e| ManusError::Http(e.to_string()))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(ManusError::Api(body));
        }

        let result: ManusTaskResponse = resp.json().await.map_err(|e| ManusError::Parse(e.to_string()))?;
        info!(task_id = %result.task_id, "Manus task created");
        
        Ok(result)
    }

    /// Get details of an existing task.
    pub async fn get_task(&self, task_id: &str) -> Result<ManusTaskDetail, ManusError> {
        let url = format!("https://api.manus.ai/v1/tasks/{}", task_id);

        let resp = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await
            .map_err(|e| ManusError::Http(e.to_string()))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(ManusError::Api(body));
        }

        let result: ManusTaskDetail = resp.json().await.map_err(|e| ManusError::Parse(e.to_string()))?;
        Ok(result)
    }
}
