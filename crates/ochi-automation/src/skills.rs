//! Skill registry scaffolding.

#[derive(Debug, Clone)]
pub struct SkillBundle {
    pub name: String,
    pub description: String,
}

#[derive(Default)]
pub struct SkillRegistry {
    skills: Vec<SkillBundle>,
}

impl SkillRegistry {
    pub fn new() -> Self {
        Self { skills: Vec::new() }
    }

    pub fn register(&mut self, skill: SkillBundle) {
        self.skills.push(skill);
    }

    pub fn list(&self) -> Vec<&SkillBundle> {
        self.skills.iter().collect()
    }
}
