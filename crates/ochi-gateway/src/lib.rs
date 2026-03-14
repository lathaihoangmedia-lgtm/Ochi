//! Gateway/API routing scaffolding.

use ochi_runtime::RuntimeEngine;
use ochi_trung_cung::{RouteRequest, TaskEnvelope};

pub struct GatewayRouter {
    runtime: RuntimeEngine,
}

impl Default for GatewayRouter {
    fn default() -> Self {
        Self::new()
    }
}

impl GatewayRouter {
    pub fn new() -> Self {
        Self {
            runtime: RuntimeEngine::new(),
        }
    }

    pub fn with_runtime(runtime: RuntimeEngine) -> Self {
        Self { runtime }
    }

    pub fn configure_ollama(&mut self, url: impl Into<String>, model: impl Into<String>) {
        self.runtime.configure_ollama(url, model);
    }

    #[cfg(feature = "duckdb")]
    pub fn route_request(
        &self,
        task_id: impl Into<String>,
        intent: impl Into<String>,
        payload: impl Into<String>,
    ) -> ochi_core::Result<()> {
        let request = RouteRequest::new(intent, payload);
        let task = TaskEnvelope::new(task_id, request);
        let _decision = self.runtime.run_task(task)?;
        Ok(())
    }
}
