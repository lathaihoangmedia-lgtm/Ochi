//! Wit.ai NLU client for Ochi Agent OS.
//!
//! Provides intent extraction and entity recognition using the Wit.ai API.
//! Used for natural language communication and intent-based routing.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::debug;

/// Error type for Wit.ai operations.
#[derive(Error, Debug)]
pub enum WitError {
    #[error("HTTP error: {0}")]
    Http(String),
    #[error("API error: {0}")]
    Api(String),
    #[error("Missing access token. Set WIT_AI_ACCESS_TOKEN environment variable.")]
    MissingToken,
    #[error("Parse error: {0}")]
    Parse(String),
}

/// Wit.ai NLU client.
pub struct WitClient {
    access_token: String,
    client: reqwest::Client,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitIntent {
    pub id: String,
    pub name: String,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitEntity {
    pub id: String,
    pub name: String,
    pub role: String,
    pub start: usize,
    pub end: usize,
    pub body: String,
    pub confidence: f32,
    pub value: serde_json::Value,
    pub type_str: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WitResponse {
    pub text: String,
    pub intents: Vec<WitIntent>,
    pub entities: serde_json::Value, // Wit.ai returns entities as map "name:role" -> [entities]
}

impl WitClient {
    /// Create a new Wit.ai client using the provided token.
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            client: reqwest::Client::new(),
        }
    }

    /// Create a new Wit.ai client from the environment variable `WIT_AI_ACCESS_TOKEN`.
    pub fn from_env() -> Result<Self, WitError> {
        let token = std::env::var("WIT_AI_ACCESS_TOKEN").map_err(|_| WitError::MissingToken)?;
        Ok(Self::new(token))
    }

    /// Extract intent and entities from a natural language message with retries.
    pub async fn message(&self, q: &str) -> Result<WitResponse, WitError> {
        let url = format!(
            "https://api.wit.ai/message?v=20240304&q={}",
            urlencoding::encode(q)
        );

        let mut attempts = 0;
        let max_attempts = 3;

        loop {
            attempts += 1;
            debug!(url = %url, attempt = attempts, "Sending Wit.ai message request");

            let resp = self
                .client
                .get(&url)
                .header("Authorization", format!("Bearer {}", self.access_token))
                .send()
                .await
                .map_err(|e| WitError::Http(e.to_string()))?;

            let status = resp.status();
            if status.is_success() {
                let body = resp
                    .text()
                    .await
                    .map_err(|e| WitError::Http(e.to_string()))?;
                let result: serde_json::Value =
                    serde_json::from_str(&body).map_err(|e| WitError::Parse(e.to_string()))?;

                let text = result["text"].as_str().unwrap_or_default().to_string();

                let mut intents = Vec::new();
                if let Some(intents_arr) = result["intents"].as_array() {
                    for i in intents_arr {
                        intents.push(WitIntent {
                            id: i["id"].as_str().unwrap_or_default().to_string(),
                            name: i["name"].as_str().unwrap_or_default().to_string(),
                            confidence: i["confidence"].as_f64().unwrap_or(0.0) as f32,
                        });
                    }
                }

                return Ok(WitResponse {
                    text,
                    intents,
                    entities: result["entities"].clone(),
                });
            } else if (status.as_u16() == 429 || status.is_server_error()) && attempts < max_attempts {
                // Rate limited or server error, retry with delay
                let delay_ms = 1000 * (2u64.pow(attempts - 1));
                warn!(
                    status = %status,
                    delay_ms = delay_ms,
                    "Wit.ai request failed, retrying..."
                );
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                continue;
            } else {
                let body = resp.text().await.unwrap_or_default();
                return Err(WitError::Api(body));
            }
        }
    }

    /// Extract intent and entities from audio data (Speech-to-Intent) with retries.
    pub async fn speech(
        &self,
        audio_data: &[u8],
        content_type: &str,
    ) -> Result<WitResponse, WitError> {
        let url = "https://api.wit.ai/speech?v=20240304";

        let mut attempts = 0;
        let max_attempts = 3;

        loop {
            attempts += 1;
            debug!(url = %url, attempt = attempts, content_type = %content_type, "Sending Wit.ai speech request");

            let resp = self
                .client
                .post(url)
                .header("Authorization", format!("Bearer {}", self.access_token))
                .header("Content-Type", content_type)
                .body(audio_data.to_vec())
                .send()
                .await
                .map_err(|e| WitError::Http(e.to_string()))?;

            let status = resp.status();
            if status.is_success() {
                let body = resp
                    .text()
                    .await
                    .map_err(|e| WitError::Http(e.to_string()))?;
                let result: serde_json::Value =
                    serde_json::from_str(&body).map_err(|e| WitError::Parse(e.to_string()))?;

                let text = result["text"].as_str().unwrap_or_default().to_string();

                let mut intents = Vec::new();
                if let Some(intents_arr) = result["intents"].as_array() {
                    for i in intents_arr {
                        intents.push(WitIntent {
                            id: i["id"].as_str().unwrap_or_default().to_string(),
                            name: i["name"].as_str().unwrap_or_default().to_string(),
                            confidence: i["confidence"].as_f64().unwrap_or(0.0) as f32,
                        });
                    }
                }

                return Ok(WitResponse {
                    text,
                    intents,
                    entities: result["entities"].clone(),
                });
            } else if (status.as_u16() == 429 || status.is_server_error()) && attempts < max_attempts {
                let delay_ms = 1000 * (2u64.pow(attempts - 1));
                warn!(
                    status = %status,
                    delay_ms = delay_ms,
                    "Wit.ai speech request failed, retrying..."
                );
                tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                continue;
            } else {
                let body = resp.text().await.unwrap_or_default();
                return Err(WitError::Api(body));
            }
        }
    }
}
