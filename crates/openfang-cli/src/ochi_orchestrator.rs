use std::path::PathBuf;

pub enum GrandAgent {
    Kim,
    Moc,
    Thuy,
    Hoa,
    Tho,
    ThaiCuc,
    Can,
    Khon,
    Nhan,
}

impl GrandAgent {
    pub fn name(&self) -> &str {
        match self {
            GrandAgent::Kim => "Ochi-KIM",
            GrandAgent::Moc => "Ochi-MỘC",
            GrandAgent::Thuy => "Ochi-THỦY",
            GrandAgent::Hoa => "Ochi-HỎA",
            GrandAgent::Tho => "Ochi-THỔ",
            GrandAgent::ThaiCuc => "Ochi-THÁI CỰC",
            GrandAgent::Can => "Ochi-CÀN",
            GrandAgent::Khon => "Ochi-KHÔN",
            GrandAgent::Nhan => "Ochi-NHÂN",
        }
    }

    pub fn element(&self) -> &str {
        match self {
            GrandAgent::Kim => "Kim",
            GrandAgent::Moc => "Mộc",
            GrandAgent::Thuy => "Thủy",
            GrandAgent::Hoa => "Hỏa",
            GrandAgent::Tho => "Thổ",
            GrandAgent::ThaiCuc => "Vô Cực",
            GrandAgent::Can => "Dương",
            GrandAgent::Khon => "Âm",
            GrandAgent::Nhan => "Nhân",
        }
    }
}

pub struct OchiOrchestrator {
    pub config_path: Option<PathBuf>,
}

impl OchiOrchestrator {
    pub fn new(config_path: Option<PathBuf>) -> Self {
        Self { config_path }
    }

    pub fn dispatch(&self, task: &str) -> String {
        // Logic điều phối cơ bản: Ánh xạ nhiệm vụ đến Đại Tác Tử phù hợp
        let task_lower = task.to_lowercase();
        let target = if task_lower.contains("logic") || task_lower.contains("security") {
            GrandAgent::Kim
        } else if task_lower.contains("create") || task_lower.contains("skill") {
            GrandAgent::Moc
        } else if task_lower.contains("memory") || task_lower.contains("data") {
            GrandAgent::Thuy
        } else if task_lower.contains("run") || task_lower.contains("process") {
            GrandAgent::Hoa
        } else if task_lower.contains("infra") || task_lower.contains("system") {
            GrandAgent::Tho
        } else if task_lower.contains("vision") || task_lower.contains("plan") {
            GrandAgent::Can
        } else if task_lower.contains("real") || task_lower.contains("exec") {
            GrandAgent::Khon
        } else if task_lower.contains("viet") || task_lower.contains("culture") {
            GrandAgent::Nhan
        } else {
            GrandAgent::ThaiCuc
        };

        format!(
            "Nhiệm vụ: '{}' đã được điều phối đến [{}] ({})",
            task,
            target.name(),
            target.element()
        )
    }
}
