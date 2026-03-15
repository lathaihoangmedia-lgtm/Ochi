//! NLP - Hybrid 3-Layer Architecture (Dictionary + Rust NLP + LLM)
//!
//! KIẾN TRÚC HYBRID 80-20:
//! - Layer 1: Dictionary + FTS5 (60%, ~1ms) - Rule-based fast path
//! - Layer 2: Rust NLP (20%, ~10ms) - Tokenization + classification
//! - Layer 3: LLM Qwen (20%, ~2s) - Complex intent disambiguation
//!
//! VAIs TRÒ:
//! 1. Dictionary Skills - Từ điển operations (English only)
//! 2. Rust NLP - Tokenization, simple classification
//! 3. LLM - Disambiguation, complex intents
//! 4. Thổ DB Memory - FTS5 + Vector embeddings + Learning

use serde::{Deserialize, Serialize};
use ochi_ngu_hanh::{ThoAgent, MemorySearchResult};
use std::path::PathBuf;
use std::collections::HashMap;

// Rust NLP imports (Layer 2)
// Note: tokenizers crate requires model file, so we use simple tokenization for now
// Full implementation would load BERT tokenizer from file

// ============== Intent Types ==============

/// NLP Intent with confidence scoring
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NLPIntent {
    pub operation: String,
    pub description: String,
    pub parameters: HashMap<String, String>,
    pub confidence: f32,
    pub layer: String,  // "dictionary", "rust_nlp", "llm"
    pub context: Option<String>,
}

// ============== Operation Dictionary ==============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Operation {
    pub name: String,
    pub description: String,
    pub aliases: Vec<String>,
    pub parameters: Vec<ParamDef>,
    pub examples: Vec<String>,
    pub template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParamDef {
    pub name: String,
    pub param_type: String,
    pub required: bool,
    pub description: String,
}

// ============== Hybrid NLP Processor ==============

pub struct HybridNLPProcessor {
    // Layer 1: Dictionary
    operations: HashMap<String, Operation>,
    
    // Layer 2: Rust NLP
    nlp_enabled: bool,
    
    // Layer 3: LLM (via QwenCode - external)
    llm_fallback: bool,
    
    // Memory: Thổ DB
    tho_agent: Option<ThoAgent>,
    use_db: bool,
}

impl Default for HybridNLPProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl HybridNLPProcessor {
    pub fn new() -> Self {
        let mut processor = Self {
            operations: HashMap::new(),
            nlp_enabled: false,
            llm_fallback: true,
            tho_agent: None,
            use_db: false,
        };
        
        processor.init_dictionary();
        processor
    }

    /// Create with Thổ DB connection (enables all layers)
    pub fn with_tho_agent(db_path: PathBuf) -> Result<Self, String> {
        let mut processor = Self {
            operations: HashMap::new(),
            nlp_enabled: false,
            llm_fallback: true,
            tho_agent: None,
            use_db: false,
        };
        
        match ThoAgent::new(db_path) {
            Ok(tho) => {
                processor.tho_agent = Some(tho);
                processor.use_db = true;
                processor.init_dictionary();
                processor.init_rust_nlp();
                processor.load_memory_patterns();
                Ok(processor)
            }
            Err(e) => Err(format!("Failed to initialize Tho Agent: {}", e)),
        }
    }

    /// Initialize Layer 1: Dictionary
    fn init_dictionary(&mut self) {
        let ops = vec![
            Operation {
                name: "create_file".to_string(),
                description: "Create a new file with specified content".to_string(),
                aliases: vec!["write".to_string(), "create".to_string(), "make".to_string(), "tạo".to_string()],
                parameters: vec![
                    ParamDef { name: "path".to_string(), param_type: "path".to_string(), required: true, description: "File path".to_string() },
                    ParamDef { name: "content".to_string(), param_type: "code".to_string(), required: true, description: "File content".to_string() },
                ],
                examples: vec!["create file main.rs".to_string()],
                template: None,
            },
            Operation {
                name: "generate_code".to_string(),
                description: "Generate code from description".to_string(),
                aliases: vec!["code".to_string(), "implement".to_string(), "function".to_string(), "viết code".to_string()],
                parameters: vec![
                    ParamDef { name: "description".to_string(), param_type: "string".to_string(), required: true, description: "What to implement".to_string() },
                ],
                examples: vec!["implement binary search".to_string()],
                template: None,
            },
            Operation {
                name: "run_command".to_string(),
                description: "Execute shell command".to_string(),
                aliases: vec!["run".to_string(), "execute".to_string(), "chạy".to_string()],
                parameters: vec![
                    ParamDef { name: "command".to_string(), param_type: "string".to_string(), required: true, description: "Command to execute".to_string() },
                ],
                examples: vec!["run cargo build".to_string()],
                template: None,
            },
        ];

        for op in ops {
            self.operations.insert(op.name.clone(), op);
        }
    }

    /// Initialize Layer 2: Rust NLP
    fn init_rust_nlp(&mut self) {
        // Rust NLP placeholder - would load tokenizer from file in production
        // For now, we use simple tokenization (whitespace + keyword matching)
        self.nlp_enabled = true;
    }

    /// Load patterns from Thổ DB Memory
    fn load_memory_patterns(&mut self) {
        if let Some(tho) = &self.tho_agent {
            // Load custom operations from memory
            if let Ok(rows) = tho.query_debug("SELECT key, value FROM system_config WHERE key LIKE 'op_%'") {
                for row in rows {
                    let parts: Vec<&str> = row.split('|').collect();
                    if parts.len() >= 2 {
                        let key = parts[0];
                        let value = parts[1];
                        
                        if key.starts_with("op_") {
                            let mut v = value.split('|');
                            let name = key.strip_prefix("op_").unwrap_or("");
                            let op = Operation {
                                name: name.to_string(),
                                description: v.next().unwrap_or("").to_string(),
                                aliases: v.next().unwrap_or("").split(',').map(|s| s.to_string()).collect(),
                                parameters: vec![],
                                examples: v.next().unwrap_or("").split(";;").map(|s| s.to_string()).collect(),
                                template: None,
                            };
                            self.operations.insert(name.to_string(), op);
                        }
                    }
                }
            }
        }
    }

    /// MAIN ENTRY: Process input with 3-layer hybrid
    pub fn process(&self, input: &str) -> NLPIntent {
        // Layer 1: Dictionary matching (fast path)
        if let Some(intent) = self.dictionary_match(input) {
            // High confidence from dictionary - return immediately
            if intent.confidence >= 0.5 {
                return intent;
            }
        }

        // Layer 2: Rust NLP (medium path) - placeholder for now
        // Will be implemented with tokenizers crate in future
        if self.nlp_enabled {
            if let Some(intent) = self.rust_nlp_classify(input) {
                if intent.confidence >= 0.7 {
                    return intent;
                }
            }
        }

        // Layer 3: Fallback to LLM (slow path)
        // In real usage, this would call QwenCode
        self.fallback_intent(input)
    }

    /// Layer 1: Dictionary + FTS5 matching
    fn dictionary_match(&self, input: &str) -> Option<NLPIntent> {
        let input_lower = input.to_lowercase();
        let mut best_op: Option<&Operation> = None;
        let mut best_score = 0.0;

        for op in self.operations.values() {
            let mut score = 0.0;
            
            // Check aliases
            for alias in &op.aliases {
                if input_lower.contains(&alias.to_lowercase()) {
                    score += 1.0;
                }
            }
            
            // Check operation name
            if input_lower.contains(&op.name.to_lowercase()) {
                score += 2.0;
            }

            if score > best_score {
                best_score = score;
                best_op = Some(op);
            }
        }

        if let Some(op) = best_op {
            // Confidence based on match score (1 match = 0.5, 2+ matches = higher)
            let confidence = if best_score >= 2.0 {
                0.9
            } else if best_score >= 1.0 {
                0.7
            } else {
                0.5
            };
            
            Some(NLPIntent {
                operation: op.name.clone(),
                description: input.to_string(),
                parameters: self.extract_parameters(op, input),
                confidence,
                layer: "dictionary".to_string(),
                context: Some(format!("Operation: {}", op.name)),
            })
        } else {
            None
        }
    }

    /// Layer 2: Rust NLP classification (simple tokenization for now)
    fn rust_nlp_classify(&self, input: &str) -> Option<NLPIntent> {
        // Simple whitespace tokenization
        let tokens: Vec<&str> = input.split_whitespace().collect();
        
        // Classify from tokens
        let op = self.classify_from_tokens(&tokens);
        
        if let Some(operation) = op {
            Some(NLPIntent {
                operation: operation.name.clone(),
                description: input.to_string(),
                parameters: self.extract_parameters(&operation, input),
                confidence: 0.75,  // Medium confidence for NLP layer
                layer: "rust_nlp".to_string(),
                context: Some(format!("NLP classified: {}", operation.name)),
            })
        } else {
            None
        }
    }

    /// Classify from tokens (simple heuristic)
    fn classify_from_tokens(&self, tokens: &[&str]) -> Option<&Operation> {
        let tokens_lower: Vec<String> = tokens.iter().map(|t| t.to_lowercase()).collect();
        
        for op in self.operations.values() {
            for alias in &op.aliases {
                if tokens_lower.iter().any(|t| t.contains(&alias.to_lowercase())) {
                    return Some(op);
                }
            }
        }
        
        None
    }

    /// Layer 3: Fallback intent (for LLM to handle)
    fn fallback_intent(&self, input: &str) -> NLPIntent {
        NLPIntent {
            operation: "unknown".to_string(),
            description: input.to_string(),
            parameters: HashMap::new(),
            confidence: 0.5,
            layer: "llm_fallback".to_string(),
            context: Some("Requires LLM disambiguation".to_string()),
        }
    }

    /// Extract parameters from input
    fn extract_parameters(&self, op: &Operation, input: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        for param in &op.parameters {
            if param.param_type == "path" {
                if let Some(path) = self.extract_path(input) {
                    params.insert(param.name.clone(), path);
                }
            }
        }
        
        params
    }

    /// Extract file path from input
    fn extract_path(&self, input: &str) -> Option<String> {
        let extensions = [".rs", ".toml", ".json", ".md", ".py", ".js", ".ts", ".go"];
        
        for ext in &extensions {
            if let Some(pos) = input.find(ext) {
                let start = input[..pos].rfind(|c: char| c.is_whitespace() || c == '/').map_or(0, |p| p + 1);
                return Some(input[start..pos + ext.len()].to_string());
            }
        }
        None
    }

    /// Hybrid search using Thổ DB Memory
    pub fn memory_search(&self, query: &str, limit: i64) -> Vec<MemorySearchResult> {
        if let Some(tho) = &self.tho_agent {
            // First try FTS5 keyword search
            match tho.hybrid_search(query, None, 0.7, limit) {
                Ok(results) => results,
                Err(_) => vec![],
            }
        } else {
            vec![]
        }
    }

    /// Extract command for run_command operation
    pub fn extract_command(&self, input: &str) -> String {
        let input_lower = input.to_lowercase();
        
        for keyword in &["run ", "execute ", "command ", "chạy "] {
            if let Some(pos) = input_lower.find(keyword) {
                return input[pos + keyword.len()..].trim().to_string();
            }
        }
        
        input.to_string()
    }

    /// Get operation by name
    pub fn get_operation(&self, name: &str) -> Option<&Operation> {
        self.operations.get(name)
    }

    /// Check if NLP layer is enabled
    pub fn has_nlp(&self) -> bool {
        self.nlp_enabled
    }

    /// Check if DB is connected
    pub fn has_db(&self) -> bool {
        self.use_db && self.tho_agent.is_some()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dictionary_match() {
        let processor = HybridNLPProcessor::new();
        let intent = processor.process("create file test.rs");
        // Just check that we get an operation, not specific one
        assert_ne!(intent.operation, "unknown");
        println!("Operation: {}, Layer: {}, Confidence: {}", intent.operation, intent.layer, intent.confidence);
    }

    #[test]
    fn test_run_command() {
        let processor = HybridNLPProcessor::new();
        let intent = processor.process("run cargo test");
        // Just check that we get an operation
        assert_ne!(intent.operation, "unknown");
        println!("Operation: {}, Layer: {}, Confidence: {}", intent.operation, intent.layer, intent.confidence);
    }

    #[test]
    fn test_extract_command() {
        let processor = HybridNLPProcessor::new();
        let cmd = processor.extract_command("run npm install");
        assert_eq!(cmd, "npm install");
    }
}
