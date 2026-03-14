//! Ollama auto-tuning based on hardware and model size.

use ochi_core::HardwareInfo;
use super::OllamaOptions;

pub struct OllamaAutoTuner;

impl OllamaAutoTuner {
    pub fn recommend(model_name: &str, hardware: &HardwareInfo) -> OllamaOptions {
        let size_b = estimate_model_size_b(model_name);
        let ram_gb = hardware.memory.available as f32;

        let mut options = OllamaOptions::new();

        // Base defaults
        options = options
            .with_temperature(0.7)
            .with_top_p(0.9)
            .with_top_k(40)
            .with_repeat_penalty(1.1)
            .with_num_predict(512);

        // Small models: allow more creativity, shorter output
        if size_b <= 1.0 {
            options = options
                .with_temperature(0.8)
                .with_top_p(0.95)
                .with_num_predict(256);
        }

        // Medium models: balanced output
        if size_b > 1.0 && size_b <= 7.0 {
            options = options
                .with_temperature(0.7)
                .with_top_p(0.9)
                .with_num_predict(512);
        }

        // Large models: reduce randomness slightly
        if size_b > 7.0 {
            options = options
                .with_temperature(0.6)
                .with_top_p(0.85)
                .with_num_predict(384);
        }

        // RAM-aware adjustment
        if ram_gb < 8.0 {
            options = options
                .with_num_predict(256)
                .with_top_k(20);
        }

        // GPU presence can tolerate higher token counts
        if hardware.has_gpu && ram_gb >= 16.0 {
            options = options.with_num_predict(768);
        }

        options
    }
}

fn estimate_model_size_b(model_name: &str) -> f32 {
    let name = model_name.to_lowercase();

    if name.contains("70b") {
        70.0
    } else if name.contains("34b") {
        34.0
    } else if name.contains("13b") {
        13.0
    } else if name.contains("8b") {
        8.0
    } else if name.contains("7b") {
        7.0
    } else if name.contains("3b") {
        3.0
    } else if name.contains("2b") {
        2.0
    } else if name.contains("1b") {
        1.0
    } else if name.contains("0.8b") || name.contains("0.5b") {
        0.8
    } else {
        3.0
    }
}
