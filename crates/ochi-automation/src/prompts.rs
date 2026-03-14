//! Prompt template scaffolding.

#[derive(Debug, Clone)]
pub struct PromptTemplate {
    pub name: String,
    pub body: String,
}

#[derive(Default)]
pub struct PromptRegistry {
    prompts: Vec<PromptTemplate>,
}

impl PromptRegistry {
    pub fn new() -> Self {
        Self { prompts: Vec::new() }
    }

    pub fn register(&mut self, template: PromptTemplate) {
        self.prompts.push(template);
    }

    pub fn list(&self) -> Vec<&PromptTemplate> {
        self.prompts.iter().collect()
    }
}
