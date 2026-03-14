//! Âm Dương Smart Router.

use std::sync::Arc;
use ochi_core::Result;
use crate::storage::{DuckDbStore, TaskFlowLog};

/// Request envelope for routing.
#[derive(Debug, Clone)]
pub struct RouteRequest {
    pub intent: String,
    pub payload: String,
    pub tags: Vec<String>,
    pub required_checkpoints: Vec<i32>,
}

#[derive(Debug, Clone)]
pub struct TaskEnvelope {
    pub task_id: String,
    pub request: RouteRequest,
}

/// Routing decision output.
#[derive(Debug, Clone)]
pub struct RouteDecision {
    pub bat_quai: String,
    pub notes: Vec<String>,
}

/// Smart router that coordinates Âm (automation) and Dương (LLM).
pub struct AmDuongRouter {
    store: Option<Arc<DuckDbStore>>,
}

impl AmDuongRouter {
    pub fn new() -> Self {
        Self { store: None }
    }

    pub fn with_store(store: Arc<DuckDbStore>) -> Self {
        Self { store: Some(store) }
    }

    /// Placeholder routing logic.
    pub fn route(&self, request: RouteRequest) -> Result<RouteDecision> {
        let mut notes = Vec::new();
        notes.push("Routing placeholder - refine in Phase 1".to_string());

        let bat_quai = if let Some(store) = &self.store {
            store
                .find_bat_quai_by_5w1h(&request.intent, &request.tags)?
                .unwrap_or_else(|| select_bat_quai(&request))
        } else {
            select_bat_quai(&request)
        };

        if let Some(store) = &self.store {
            let db = store
                .db_for_bat_quai(&bat_quai)?
                .ok_or_else(|| ochi_core::Error::Custom(format!(
                    "No DB mapping for bat_quai: {}",
                    bat_quai
                )))?;

            let checkpoint = store
                .checkpoint_for_bat_quai(&bat_quai)?
                .ok_or_else(|| ochi_core::Error::Custom(format!(
                    "No checkpoint for bat_quai: {}",
                    bat_quai
                )))?;

            if !request.required_checkpoints.is_empty()
                && !request.required_checkpoints.contains(&checkpoint.checkpoint_number)
            {
                return Err(ochi_core::Error::Custom(format!(
                    "Checkpoint {} not allowed for request",
                    checkpoint.checkpoint_number
                )));
            }

            notes.push(format!(
                "Checkpoint {} ({}) -> DB {}",
                checkpoint.checkpoint_number, checkpoint.ngu_hanh, db
            ));

            let _ = store.record_route(
                "route-boot",
                &request.intent,
                &bat_quai,
                &notes.join(" | "),
            );
        }

        Ok(RouteDecision { bat_quai, notes })
    }

    pub fn execute_task(&self, task: TaskEnvelope) -> Result<RouteDecision> {
        let decision = self.route(task.request)?;
        if let Some(store) = &self.store {
            let checkpoint = store
                .checkpoint_for_bat_quai(&decision.bat_quai)?
                .ok_or_else(|| ochi_core::Error::Custom(format!(
                    "No checkpoint for bat_quai: {}",
                    decision.bat_quai
                )))?;

            store.log_task_flow(TaskFlowLog {
                flow_id: task.task_id.clone(),
                phase: "pre".to_string(),
                checkpoint_number: checkpoint.checkpoint_number,
                bat_quai: decision.bat_quai.clone(),
                status: "ok".to_string(),
            })?;

            store.log_task_flow(TaskFlowLog {
                flow_id: task.task_id.clone(),
                phase: "post".to_string(),
                checkpoint_number: checkpoint.checkpoint_number,
                bat_quai: decision.bat_quai.clone(),
                status: "ok".to_string(),
            })?;
        }

        Ok(decision)
    }
}

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
    use crate::storage::DuckDbStore;
    use std::sync::Arc;

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
    fn test_route_with_checkpoint_and_db() {
        let store = Arc::new(DuckDbStore::open_in_memory().expect("store"));
        let router = AmDuongRouter::with_store(store);
        let request = RouteRequest {
            intent: "stream upload".to_string(),
            payload: "file".to_string(),
            tags: vec!["stream".to_string()],
            required_checkpoints: vec![1],
        };
        let decision = router.route(request).expect("route");
        assert_eq!(decision.bat_quai, "Kham");
        assert!(decision.notes.iter().any(|n| n.contains("Checkpoint")));
    }

    #[test]
    fn test_execute_task_logs_pre_post() {
        let store = Arc::new(DuckDbStore::open_in_memory().expect("store"));
        let router = AmDuongRouter::with_store(store);
        let request = RouteRequest {
            intent: "chat with user".to_string(),
            payload: "hello".to_string(),
            tags: vec!["chat".to_string()],
            required_checkpoints: Vec::new(),
        };
        let task = TaskEnvelope {
            task_id: "task-1".to_string(),
            request,
        };
        let decision = router.execute_task(task).expect("execute");
        assert_eq!(decision.bat_quai, "Doai");
    }
}
