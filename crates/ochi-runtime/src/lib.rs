//! Runtime glue for automation + agents.

use ochi_automation::{PromptRegistry, SkillRegistry, ToolRegistry};
use ochi_trung_cung::{AmDuongRouter, RouteDecision, TaskEnvelope};

pub struct RuntimeEngine {
    router: AmDuongRouter,
    tools: ToolRegistry,
    skills: SkillRegistry,
    prompts: PromptRegistry,
}

impl Default for RuntimeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RuntimeEngine {
    pub fn new() -> Self {
        Self {
            router: AmDuongRouter::new(),
            tools: ToolRegistry::new(),
            skills: SkillRegistry::new(),
            prompts: PromptRegistry::new(),
        }
    }

    pub fn with_router(mut self, router: AmDuongRouter) -> Self {
        self.router = router;
        self
    }

    pub fn tools(&mut self) -> &mut ToolRegistry {
        &mut self.tools
    }

    pub fn skills(&mut self) -> &mut SkillRegistry {
        &mut self.skills
    }

    pub fn prompts(&mut self) -> &mut PromptRegistry {
        &mut self.prompts
    }

    #[cfg(feature = "duckdb")]
    pub fn run_task(&self, task: TaskEnvelope) -> ochi_core::Result<RouteDecision> {
        self.router.execute_task(task)
    }

    pub fn configure_ollama(&mut self, url: impl Into<String>, model: impl Into<String>) {
        self.router.set_ollama(url, model);
    }
}
