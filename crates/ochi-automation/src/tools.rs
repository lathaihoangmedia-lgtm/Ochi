//! Tool registry scaffolding.

pub trait AutomationTool {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
}

#[derive(Default)]
pub struct ToolRegistry {
    tools: Vec<Box<dyn AutomationTool + Send + Sync>>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self { tools: Vec::new() }
    }

    pub fn register(&mut self, tool: Box<dyn AutomationTool + Send + Sync>) {
        self.tools.push(tool);
    }

    pub fn list(&self) -> Vec<&'static str> {
        self.tools.iter().map(|t| t.name()).collect()
    }
}
