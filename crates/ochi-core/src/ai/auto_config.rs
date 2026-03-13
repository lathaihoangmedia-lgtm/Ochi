//! Model Auto-Configuration & Loop Prevention
//!
//! Automatically detects model issues, applies fixes, and generates reports

use serde::{Deserialize, Serialize};
use std::path::Path;
use crate::ai::model::GGUFConfig;
use crate::ai::loop_detector::LoopDetector;
use crate::hardware::HardwareInfo;

/// Auto-configuration result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoConfigResult {
    pub model_name: String,
    pub model_path: String,
    pub config: GGUFConfig,
    pub issues_detected: Vec<Issue>,
    pub fixes_applied: Vec<Fix>,
    pub recommendations: Vec<String>,
    pub performance_estimate: PerformanceEstimate,
}

/// Detected issue
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub code: String,
    pub severity: Severity,
    pub description: String,
    pub suggestion: String,
}

/// Severity level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Severity {
    Info,
    Warning,
    Critical,
}

/// Applied fix
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fix {
    pub issue_code: String,
    pub description: String,
    pub old_value: String,
    pub new_value: String,
}

/// Performance estimate
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceEstimate {
    pub estimated_speed_tps: f32,
    pub estimated_vram_mb: u32,
    pub estimated_ram_mb: u32,
    pub quality_rating: u8,
}

/// Model issue detector and auto-fixer
pub struct AutoConfigurator {
    hardware: HardwareInfo,
    loop_detector: LoopDetector,
}

impl AutoConfigurator {
    /// Create new auto-configurator
    pub fn new(hardware: HardwareInfo) -> Self {
        Self {
            hardware,
            loop_detector: LoopDetector::new(10, 0.7),
        }
    }

    /// Auto-configure model with all fixes
    pub fn auto_configure<P: AsRef<Path>>(&mut self, model_path: P) -> AutoConfigResult {
        let path = model_path.as_ref();
        let model_name = self.extract_model_name(path);
        
        let mut issues = Vec::new();
        let mut fixes = Vec::new();
        let mut recommendations = Vec::new();

        // Start with default config
        let mut config = GGUFConfig::default();
        config.model_path = path.to_string_lossy().to_string();

        // Detect and fix issues
        self.detect_model_specific_issues(&model_name, &mut config, &mut issues, &mut fixes);
        self.apply_hardware_optimizations(&mut config, &mut recommendations);
        self.enable_loop_prevention(&mut config, &mut recommendations);
        self.add_stop_sequences(&mut config, &model_name);

        // Estimate performance
        let perf = self.estimate_performance(&model_name, &config);

        AutoConfigResult {
            model_name,
            model_path: path.to_string_lossy().to_string(),
            config,
            issues_detected: issues,
            fixes_applied: fixes,
            recommendations,
            performance_estimate: perf,
        }
    }

    /// Extract model name from path
    fn extract_model_name(&self, path: &Path) -> String {
        path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string()
    }

    /// Detect model-specific issues
    fn detect_model_specific_issues(
        &self,
        model_name: &str,
        config: &mut GGUFConfig,
        issues: &mut Vec<Issue>,
        fixes: &mut Vec<Fix>,
    ) {
        // Qwen3.5-0.8B specific fixes
        if model_name.contains("qwen3.5") || model_name.contains("qwen_3.5") {
            // Issue: Tends to loop on short responses
            issues.push(Issue {
                code: "QWEN_LOOP_TENDENCY".to_string(),
                severity: Severity::Warning,
                description: "Qwen3.5-0.8B has tendency to loop on repetitive prompts".to_string(),
                suggestion: "Enable loop detection and increase temperature".to_string(),
            });

            // Fix: Increase temperature
            if config.temperature < 0.75 {
                fixes.push(Fix {
                    issue_code: "QWEN_LOOP_TENDENCY".to_string(),
                    description: "Increased temperature to reduce looping".to_string(),
                    old_value: format!("{:.2}", config.temperature),
                    new_value: "0.75".to_string(),
                });
                config.temperature = 0.75;
            }

            // Fix: Add repetition penalty
            if config.repetition_penalty