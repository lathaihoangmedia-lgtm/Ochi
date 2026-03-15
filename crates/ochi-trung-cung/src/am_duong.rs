//! Âm Dương Smart Router.
//!
//! Trung Cung coordinates the routing of requests between Âm (Automation) 
//! and Dương (LLM) agents using Bát Quái logic.

use ochi_core::Result;

/// Request envelope for routing.
#[derive(Debug, Clone)]
pub struct RouteRequest {
    pub intent: String,
    pub payload: String,
    pub tags: Vec<String>,
    pub required_checkpoints: Vec<i32>,
}

impl RouteRequest {
    pub fn new(intent: impl Into<String>, payload: impl Into<String>) -> Self {
        Self {
            intent: intent.into(),
            payload: payload.into(),
            tags: Vec::new(),
            required_checkpoints: Vec::new(),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }

    pub fn require_checkpoints(mut self, checkpoints: Vec<i32>) -> Self {
        self.required_checkpoints = checkpoints;
        self
    }
}

#[derive(Debug, Clone)]
pub struct TaskEnvelope {
    pub task_id: String,
    pub request: RouteRequest,
}

impl TaskEnvelope {
    pub fn new(task_id: impl Into<String>, request: RouteRequest) -> Self {
        Self {
            task_id: task_id.into(),
            request,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OllamaConfig {
    pub url: String,
    pub model: String,
}

/// Routing decision output.
#[derive(Debug, Clone)]
pub struct RouteDecision {
    pub bat_quai: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RouterStatus {
    pub ready: bool,
    pub notes: Vec<String>,
}

/// Smart router that coordinates Âm (automation) and Dương (LLM).
pub struct AmDuongRouter {
    ollama: Option<OllamaConfig>,
}

impl Default for AmDuongRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl AmDuongRouter {
    pub fn new() -> Self {
        Self {
            ollama: None,
        }
    }

    /// Configure Ollama as primary local LLM (scaffold).
    pub fn with_ollama(mut self, url: impl Into<String>, model: impl Into<String>) -> Self {
        self.ollama = Some(OllamaConfig {
            url: url.into(),
            model: model.into(),
        });
        self
    }

    pub fn set_ollama(&mut self, url: impl Into<String>, model: impl Into<String>) {
        self.ollama = Some(OllamaConfig {
            url: url.into(),
            model: model.into(),
        });
    }

    /// Routing logic that selects a Bát Quái module.
    pub fn route(&self, request: RouteRequest) -> Result<RouteDecision> {
        let mut notes = Vec::new();
        notes.push("Routing placeholder - currently using rule-based selection".to_string());

        // Select Bat Quai based on request details
        let bat_quai = select_bat_quai(&request);

        // Note: Future integration will use ThoAgent for metadata-driven routing
        Ok(RouteDecision { bat_quai, notes })
    }

    /// Execute a flow-tracked task.
    pub fn execute_task(&self, task: TaskEnvelope) -> Result<RouteDecision> {
        // Route the request
        let decision = self.route(task.request)?;
        
        // Note: Logging to ThoAgent (Audit logs) would happen here
        Ok(decision)
    }

    /// Lightweight startup check to validate routing readiness.
    pub fn start(&self) -> RouterStatus {
        let mut notes = Vec::new();
        notes.push("AmDuongRouter: Ready (Static Mode)".to_string());

        if let Some(cfg) = &self.ollama {
            notes.push(format!("Ollama configured: {} ({})", cfg.url, cfg.model));
        } else {
            notes.push("Ollama not configured".to_string());
        }

        RouterStatus {
            ready: true,
            notes,
        }
    }
}

/// Rule-based fallback/initial selection for Bát Quái agents.
fn select_bat_quai(request: &RouteRequest) -> String {
    let intent = request.intent.to_lowercase();
    let tags = request
        .tags
        .iter()
        .map(|t| t.to_lowercase())
        .collect::<Vec<_>>();

    if intent.contains("chat") || tags.iter().any(|t| t.contains("chat")) {
        return "Doai".to_string();
    }
    if intent.contains("stream") || tags.iter().any(|t| t.contains("stream")) {
        return "Kham".to_string();
    }
    if intent.contains("cache") || tags.iter().any(|t| t.contains("cache")) {
        return "Khon".to_string();
    }
    if intent.contains("webhook") || tags.iter().any(|t| t.contains("webhook")) {
        return "Chan".to_string();
    }
    if intent.contains("balance") || tags.iter().any(|t| t.contains("balance")) {
        return "Ton".to_string();
    }
    if intent.contains("session") || tags.iter().any(|t| t.contains("session")) {
        return "Can Gua".to_string();
    }
    if intent.contains("search") || tags.iter().any(|t| t.contains("search")) {
        return "Ly".to_string();
    }

    "Can".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_selection() {
        let router = AmDuongRouter::new();
        let request = RouteRequest {
            intent: "chat with user".to_string(),
            payload: "hello".to_string(),
            tags: vec!["chat".to_string()],
            required_checkpoints: Vec::new(),
        };
        let decision = router.route(request).expect("route");
        assert_eq!(decision.bat_quai, "Doai");
    }

    #[test]
    fn test_route_kham() {
        let router = AmDuongRouter::new();
        let request = RouteRequest {
            intent: "stream upload".to_string(),
            payload: "file".to_string(),
            tags: vec!["stream".to_string()],
            required_checkpoints: Vec::new(),
        };
        let decision = router.route(request).expect("route");
        assert_eq!(decision.bat_quai, "Kham");
    }
}

