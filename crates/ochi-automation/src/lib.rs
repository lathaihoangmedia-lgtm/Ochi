//! Automation layer: tools, skills, prompts, and task glue.

pub mod tools;
pub mod skills;
pub mod prompts;

pub use tools::{AutomationTool, ToolRegistry};
pub use skills::{SkillBundle, SkillRegistry};
pub use prompts::{PromptTemplate, PromptRegistry};
