//! Scaffolding for Thiên Cơ Các manager.

/// Trait for a Thiên Cương strategic agent.
pub trait ThienCuongAgent {
    fn id(&self) -> &'static str;
}

/// Manager placeholder for 36 Thiên Cương agents.
#[derive(Default)]
pub struct ThienCoCacManager {
    agents: Vec<Box<dyn ThienCuongAgent + Send + Sync>>,
}

impl ThienCoCacManager {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn register_agent(&mut self, agent: Box<dyn ThienCuongAgent + Send + Sync>) {
        self.agents.push(agent);
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
}
