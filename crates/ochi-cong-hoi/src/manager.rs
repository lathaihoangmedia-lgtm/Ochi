//! Scaffolding for Công Hội manager.

/// Trait for a Địa Sát execution agent.
pub trait DiaSatAgent {
    fn id(&self) -> &'static str;
}

/// Manager placeholder for 72 Địa Sát agents.
#[derive(Default)]
pub struct CongHoiManager {
    agents: Vec<Box<dyn DiaSatAgent + Send + Sync>>,
}

impl CongHoiManager {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn register_agent(&mut self, agent: Box<dyn DiaSatAgent + Send + Sync>) {
        self.agents.push(agent);
    }

    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }
}
