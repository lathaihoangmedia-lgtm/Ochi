#![allow(dead_code)]
//! Ochi Grand Agent Orchestration Logic
//!
//! This module is responsible for dispatching tasks to the correct Grand Agent
//! based on the principles of the 5 Elements (Ngũ Hành) and the 8 Trigrams (Bát Quái).

use serde::{Deserialize, Serialize};

/// The 9 Grand Agents, representing the core functions of the Ochi Agent OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrandAgent {
    /// Kim (Metal): Logic, Structure, Security.
    Kim,
    /// Mộc (Wood): Creativity, Growth, R&D.
    Moc,
    /// Thủy (Water): Memory, Information Flow, Knowledge Base (LacViet DB).
    Thuy,
    /// Hỏa (Fire): Processing, Execution, NLP.
    Hoa,
    /// Thổ (Earth): Integration, Infrastructure, Stability (aaPanel MCP).
    Tho,
    /// Thái Cực (Ultimate): Central Dispatcher, Balancer.
    ThaiCuc,
    /// Càn (Heaven): Vision, Long-term Planning, Strategic Agents (Thien Cuong).
    Can,
    /// Khôn (Earthly): Practicality, Real-world Interaction, Execution Agents (Dia Sat).
    Khon,
    /// Nhân (Humanity): Interface, Culture, Vietnamese Localization.
    Nhan,
}

impl std::fmt::Display for GrandAgent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            GrandAgent::Kim => "Ochi-KIM",
            GrandAgent::Moc => "Ochi-MOC",
            GrandAgent::Thuy => "Ochi-THUY",
            GrandAgent::Hoa => "Ochi-HOA",
            GrandAgent::Tho => "Ochi-THO",
            GrandAgent::ThaiCuc => "Ochi-THAI-CUC",
            GrandAgent::Can => "Ochi-CAN",
            GrandAgent::Khon => "Ochi-KHON",
            GrandAgent::Nhan => "Ochi-NHAN",
        };
        write!(f, "{}", name)
    }
}

/// The central orchestrator that dispatches tasks to Grand Agents.
pub struct Orchestrator;

impl Orchestrator {
    /// Creates a new Orchestrator.
    pub fn new() -> Self {
        Self
    }

    /// Dispatches a task to the appropriate Grand Agent based on keywords.
    ///
    /// This is a simple keyword-based routing mechanism that mimics the logic
    /// from the initial Python PoC.
    pub fn dispatch(&self, task_description: &str) -> GrandAgent {
        let task_lower = task_description.to_lowercase();

        if task_lower.contains("bảo mật") || task_lower.contains("lỗ hổng") || task_lower.contains("logic") || task_lower.contains("cấu trúc") {
            GrandAgent::Kim
        } else if task_lower.contains("sáng tạo") || task_lower.contains("r&d") || task_lower.contains("skill mới") {
            GrandAgent::Moc
        } else if task_lower.contains("dữ liệu") || task_lower.contains("lạc việt") || task_lower.contains("trí nhớ") || task_lower.contains("vector") {
            GrandAgent::Thuy
        } else if task_lower.contains("xử lý") || task_lower.contains("thực thi") || task_lower.contains("nlp") {
            GrandAgent::Hoa
        } else if task_lower.contains("hạ tầng") || task_lower.contains("aapanel") || task_lower.contains("server") || task_lower.contains("deploy") {
            GrandAgent::Tho
        } else if task_lower.contains("tầm nhìn") || task_lower.contains("chiến lược") || task_lower.contains("kế hoạch") {
            GrandAgent::Can
        } else if task_lower.contains("thực tế") || task_lower.contains("người dùng") || task_lower.contains("chi tiết kỹ thuật") {
            GrandAgent::Khon
        } else if task_lower.contains("việt hóa") || task_lower.contains("văn hóa") || task_lower.contains("giao diện") || task_lower.contains("dịch thuật") {
            GrandAgent::Nhan
        } else {
            // Default to Thai Cuc for general or unclassified tasks
            GrandAgent::ThaiCuc
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dispatch_logic() {
        let orchestrator = Orchestrator::new();

        // Test Kim
        assert_eq!(orchestrator.dispatch("Kiểm tra lỗ hổng bảo mật hệ thống"), GrandAgent::Kim);
        assert_eq!(orchestrator.dispatch("Phân tích logic mã nguồn"), GrandAgent::Kim);

        // Test Thuy
        assert_eq!(orchestrator.dispatch("Xử lý dữ liệu lớn từ Lạc Việt DB"), GrandAgent::Thuy);
        assert_eq!(orchestrator.dispatch("Cập nhật trí nhớ dài hạn"), GrandAgent::Thuy);

        // Test Tho
        assert_eq!(orchestrator.dispatch("Cấu hình aaPanel cho server mới (hạ tầng)"), GrandAgent::Tho);
        assert_eq!(orchestrator.dispatch("Deploy ứng dụng lên server"), GrandAgent::Tho);

        // Test Can
        assert_eq!(orchestrator.dispatch("Lập kế hoạch tầm nhìn dài hạn cho Ophiuchus"), GrandAgent::Can);

        // Test Nhan
        assert_eq!(orchestrator.dispatch("Dịch tài liệu sang tiếng Việt và tối ưu văn hóa"), GrandAgent::Nhan);

        // Test Thai Cuc (Default)
        assert_eq!(orchestrator.dispatch("Một nhiệm vụ chung chung"), GrandAgent::ThaiCuc);
    }
}
