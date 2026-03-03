//! # Ochi Grand Agent Orchestration
//!
//! Điều phối trung tâm của Ochi Agent OS theo nguyên lý **Ngũ Hành – Bát Quái – Âm Dương**.
//!
//! ## 9 Đại Tác Tử
//!
//! | Tác Tử   | Hành            | Vai trò                                   |
//! |----------|-----------------|-------------------------------------------|
//! | KIM      | Kim (Metal)     | Logic, Security, Code Quality             |
//! | MỘC      | Mộc (Wood)      | Creativity, Growth, R&D                   |
//! | THỦY     | Thủy (Water)    | Memory, Knowledge, LacViet DB             |
//! | HỎA      | Hỏa (Fire)      | Processing, Execution, NLP                |
//! | THỔ      | Thổ (Earth)     | Infrastructure, Integration, aaPanel      |
//! | THÁI CỰC | Vô Cực          | Central Dispatcher, Balancer              |
//! | CÀN      | Càn (Heaven)    | Vision, Long-term Planning (Thiên Cương)  |
//! | KHÔN     | Khôn (Earth)    | Practicality, Execution (Địa Sát)         |
//! | NHÂN     | Nhân (Humanity) | Interface, Culture, Vietnamese L10n       |

use serde::{Deserialize, Serialize};
use std::fmt;

// ---------------------------------------------------------------------------
// Core Types
// ---------------------------------------------------------------------------

/// 9 Đại Tác Tử — đại diện cho 9 chức năng cốt lõi của Ochi Agent OS.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum GrandAgent {
    /// **Kim (Metal):** Logic, Structure, Security, Code Quality.
    Kim,
    /// **Mộc (Wood):** Creativity, Growth, R&D, Skill Development.
    Moc,
    /// **Thủy (Water):** Memory, Information Flow, Knowledge Base (LacViet DB), Vector Search.
    Thuy,
    /// **Hỏa (Fire):** Processing, Execution, NLP, Real-time Tasks.
    Hoa,
    /// **Thổ (Earth):** Integration, Infrastructure, Stability (aaPanel MCP), DevOps.
    Tho,
    /// **Thái Cực (Ultimate):** Central Dispatcher, Balancer, Multi-agent Coordinator.
    ThaiCuc,
    /// **Càn (Heaven):** Vision, Long-term Planning, Strategic Agents (Thiên Cương).
    Can,
    /// **Khôn (Earth/Practical):** Practicality, Real-world Interaction, Execution Agents (Địa Sát).
    Khon,
    /// **Nhân (Humanity):** Interface, Culture, Vietnamese Localization, Communication.
    Nhan,
}

impl fmt::Display for GrandAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            GrandAgent::Kim     => "Ochi-KIM",
            GrandAgent::Moc     => "Ochi-MOC",
            GrandAgent::Thuy    => "Ochi-THUY",
            GrandAgent::Hoa     => "Ochi-HOA",
            GrandAgent::Tho     => "Ochi-THO",
            GrandAgent::ThaiCuc => "Ochi-THAI-CUC",
            GrandAgent::Can     => "Ochi-CAN",
            GrandAgent::Khon    => "Ochi-KHON",
            GrandAgent::Nhan    => "Ochi-NHAN",
        };
        write!(f, "{}", name)
    }
}

impl GrandAgent {
    /// Mô tả ngắn gọn về vai trò của Đại Tác Tử.
    pub fn description(&self) -> &'static str {
        match self {
            GrandAgent::Kim     => "Logic, cấu trúc, bảo mật, chất lượng mã nguồn",
            GrandAgent::Moc     => "Sáng tạo, phát triển, R&D, kỹ năng mới",
            GrandAgent::Thuy    => "Trí nhớ, tri thức, Lạc Việt DB, tìm kiếm ngữ nghĩa",
            GrandAgent::Hoa     => "Xử lý, thực thi, NLP, tác vụ thời gian thực",
            GrandAgent::Tho     => "Hạ tầng, tích hợp, ổn định, aaPanel, DevOps",
            GrandAgent::ThaiCuc => "Điều phối trung tâm, cân bằng Âm Dương, đa nhiệm vụ",
            GrandAgent::Can     => "Tầm nhìn, kế hoạch dài hạn, chiến lược (Thiên Cương)",
            GrandAgent::Khon    => "Thực tiễn, tương tác thực tế, thực thi (Địa Sát)",
            GrandAgent::Nhan    => "Giao diện, văn hóa, Việt hóa, giao tiếp",
        }
    }

    /// Hành (element) tương ứng theo Ngũ Hành.
    pub fn element(&self) -> &'static str {
        match self {
            GrandAgent::Kim     => "Kim (Metal)",
            GrandAgent::Moc     => "Mộc (Wood)",
            GrandAgent::Thuy    => "Thủy (Water)",
            GrandAgent::Hoa     => "Hỏa (Fire)",
            GrandAgent::Tho     => "Thổ (Earth)",
            GrandAgent::ThaiCuc => "Vô Cực (Void)",
            GrandAgent::Can     => "Càn (Heaven)",
            GrandAgent::Khon    => "Khôn (Earth/Yin)",
            GrandAgent::Nhan    => "Nhân (Humanity)",
        }
    }
}

/// Phân cực Âm Dương của nhiệm vụ.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Polarity {
    /// **Dương (Yang):** Hành động, tạo ra, triển khai, giao tiếp.
    Yang,
    /// **Âm (Yin):** Phân tích, học hỏi, lưu trữ, đánh giá, nghiên cứu.
    Yin,
    /// **Trung tính:** Cân bằng, không thiên về Âm hay Dương.
    Neutral,
}

impl fmt::Display for Polarity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Polarity::Yang    => write!(f, "Duong (Yang)"),
            Polarity::Yin     => write!(f, "Am (Yin)"),
            Polarity::Neutral => write!(f, "Trung tinh"),
        }
    }
}

/// Gợi ý Sub-agent (Thiên Cương hoặc Địa Sát) cho nhiệm vụ cụ thể.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubAgentSuggestion {
    /// Tên sub-agent (ví dụ: "TC-Security-Auditor", "DS-Web-Scraper").
    pub name: String,
    /// Loại: "thien_cuong" hoặc "dia_sat".
    pub kind: String,
    /// Lý do gợi ý.
    pub reason: String,
}

/// Kết quả điều phối đầy đủ từ Orchestrator.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DispatchDecision {
    /// Đại Tác Tử chính được chọn.
    pub primary: GrandAgent,
    /// Độ tin cậy của quyết định (0.0 – 1.0).
    pub confidence: f32,
    /// Phân cực Âm Dương của nhiệm vụ.
    pub polarity: Polarity,
    /// Danh sách sub-agent được gợi ý.
    pub suggested_sub_agents: Vec<SubAgentSuggestion>,
    /// Có cần leo thang lên Thái Cực để phân tích sâu hơn không?
    pub escalate_to_thai_cuc: bool,
    /// Lý do điều phối (để debug và audit).
    pub reasoning: String,
}

impl DispatchDecision {
    fn default_to_thai_cuc(input: &str) -> Self {
        let preview = &input[..input.len().min(60)];
        Self {
            primary: GrandAgent::ThaiCuc,
            confidence: 0.3,
            polarity: Polarity::Neutral,
            suggested_sub_agents: vec![SubAgentSuggestion {
                name: "TC-Context-Manager".to_string(),
                kind: "thien_cuong".to_string(),
                reason: "Nhiem vu chua ro rang, can phan tich ngu canh truoc".to_string(),
            }],
            escalate_to_thai_cuc: true,
            reasoning: format!(
                "Khong tim thay tin hieu ro rang trong: '{}'. Chuyen ve Thai Cuc.",
                preview
            ),
        }
    }
}

// ---------------------------------------------------------------------------
// Routing Rules Table
// ---------------------------------------------------------------------------

struct RoutingRule {
    agent: GrandAgent,
    keywords: &'static [&'static str],
    base_score: f32,
    polarity: Polarity,
    thien_cuong: &'static [&'static str],
    dia_sat: &'static [&'static str],
}

static ROUTING_RULES: &[RoutingRule] = &[
    // KIM: Logic, Security, Code Quality
    RoutingRule {
        agent: GrandAgent::Kim,
        keywords: &[
            "bao mat", "security", "lo hong", "vulnerability", "exploit",
            "logic", "cau truc", "structure", "kien truc", "architecture",
            "audit", "kiem tra ma", "code review", "refactor",
            "unit test", "kiem thu", "ci/cd", "pipeline",
            "dependency", "phu thuoc", "cargo", "package", "version",
            "git", "github", "commit", "branch", "merge",
            "secret", "token", "api key", "credential",
            "permission", "quyen truy cap", "rbac", "authentication",
        ],
        base_score: 0.75,
        polarity: Polarity::Yin,
        thien_cuong: &["TC-Security-Auditor", "TC-Compliance-Officer", "TC-Quality-Assurance"],
        dia_sat: &["DS-Security-Scanner", "DS-Git-Operator", "DS-Unit-Tester", "DS-Dependency-Checker", "DS-Secret-Vault"],
    },
    // MOC: Creativity, R&D, Skill Development
    RoutingRule {
        agent: GrandAgent::Moc,
        keywords: &[
            "sang tao", "creative", "r&d", "nghien cuu", "research",
            "skill moi", "new skill", "plugin", "extension", "tinh nang moi",
            "feature", "prototype", "thu nghiem", "experiment",
            "hoc may", "machine learning", "fine-tune", "huan luyen",
            "thiet ke", "design", "ux", "ui design", "mockup",
            "brainstorm", "y tuong", "idea", "de xuat", "proposal",
            "innovation", "doi moi", "cai tien",
        ],
        base_score: 0.72,
        polarity: Polarity::Yang,
        thien_cuong: &["TC-Innovation-Scout", "TC-Agent-Trainer", "TC-Visionary-Agent"],
        dia_sat: &["DS-Code-Executor", "DS-Benchmark-Tool", "DS-Integration-Tester"],
    },
    // THUY: Memory, Knowledge, LacViet DB, Vector Search
    RoutingRule {
        agent: GrandAgent::Thuy,
        keywords: &[
            "du lieu", "data", "lac viet", "lacviet", "lac viet db",
            "tri nho", "memory", "knowledge base", "co so tri thuc",
            "vector", "embedding", "semantic search", "tim kiem ngu nghia",
            "database", "co so du lieu", "sql", "query", "truy van",
            "luu tru", "storage", "backup", "sao luu",
            "thu thap", "scrape", "crawl", "web scraping",
            "pdf", "extract", "trich xuat", "parse",
            "triet hoc", "philosophy", "van hoa viet", "lich su",
            "knowledge graph", "ontology",
        ],
        base_score: 0.78,
        polarity: Polarity::Yin,
        thien_cuong: &["TC-Data-Strategist", "TC-Knowledge-Curator", "TC-Research-Lead"],
        dia_sat: &["DS-Vector-Search", "DS-SQL-Expert", "DS-Web-Scraper", "DS-PDF-Extractor", "DS-Data-Cleaner"],
    },
    // HOA: Processing, Execution, NLP, Real-time
    RoutingRule {
        agent: GrandAgent::Hoa,
        keywords: &[
            "xu ly", "process", "thuc thi", "execute", "run",
            "nlp", "natural language", "ngon ngu tu nhien",
            "wit.ai", "zalo ai", "speech", "giong noi", "tts", "stt",
            "phan tich van ban", "text analysis", "sentiment",
            "real-time", "thoi gian thuc", "streaming", "stream",
            "tac vu nhanh", "quick task", "automation", "tu dong hoa",
            "browser", "trinh duyet", "playwright", "selenium",
            "image", "hinh anh", "video", "media", "xu ly anh",
            "api call", "goi api", "webhook", "event",
        ],
        base_score: 0.73,
        polarity: Polarity::Yang,
        thien_cuong: &["TC-Performance-Analyst", "TC-Workflow-Optimizer", "TC-Predictive-Modeler"],
        dia_sat: &["DS-WitAI-Integrator", "DS-ZaloAI-Integrator", "DS-Code-Executor", "DS-Browser-Automator", "DS-Image-Processor"],
    },
    // THO: Infrastructure, aaPanel, DevOps, Integration
    RoutingRule {
        agent: GrandAgent::Tho,
        keywords: &[
            "ha tang", "infrastructure", "aapanel", "server",
            "deploy", "trien khai", "deployment", "vps", "cloud",
            "docker", "container", "kubernetes", "k8s",
            "nginx", "apache", "reverse proxy", "load balancer",
            "tich hop", "integration", "api gateway",
            "mcp", "mcp server", "tool server",
            "cron", "lap lich", "schedule", "job",
            "monitor", "giam sat", "alert", "canh bao",
            "network", "mang", "firewall", "ssl", "certificate",
            "file system", "he thong tep", "disk",
            "systemd", "service", "daemon",
        ],
        base_score: 0.76,
        polarity: Polarity::Yang,
        thien_cuong: &["TC-Infrastructure-Lead", "TC-System-Integrator", "TC-Scalability-Expert"],
        dia_sat: &["DS-aaPanel-Controller", "DS-Docker-Manager", "DS-Shell-Scripting", "DS-Network-Monitor", "DS-File-Manager", "DS-Cron-Scheduler"],
    },
    // CAN: Vision, Strategy, Long-term Planning
    RoutingRule {
        agent: GrandAgent::Can,
        keywords: &[
            "tam nhin", "vision", "chien luoc", "strategy", "strategic",
            "ke hoach dai han", "long-term", "roadmap", "lo trinh",
            "muc tieu", "goal", "objective", "okr",
            "du an", "project", "milestone", "giai doan",
            "ophiuchus", "neuro ai", "vibe press",
            "kien truc he thong", "system design", "blueprint",
            "phan tich thi truong", "market analysis",
            "triet hoc lac viet", "mam viet", "vu tru", "cosmos",
        ],
        base_score: 0.74,
        polarity: Polarity::Yin,
        thien_cuong: &["TC-Visionary-Agent", "TC-Strategic-Planner", "TC-Predictive-Modeler", "TC-Risk-Assessor"],
        dia_sat: &["DS-Research-Aggregator", "DS-Report-Generator"],
    },
    // KHON: Practical Execution, Real-world Interaction
    RoutingRule {
        agent: GrandAgent::Khon,
        keywords: &[
            "thuc te", "practical", "thuc hanh", "hands-on",
            "nguoi dung", "user", "khach hang", "customer",
            "chi tiet ky thuat", "technical detail", "implementation",
            "debug", "sua loi", "fix bug", "troubleshoot",
            "bao cao", "report", "log", "nhat ky",
            "phan tich loi", "error analysis", "root cause",
            "tai chinh", "cost", "chi phi", "budget", "token cost",
            "audit trail", "lich su", "history", "trace",
            "compliance", "tuan thu", "policy", "quy dinh",
        ],
        base_score: 0.71,
        polarity: Polarity::Yang,
        thien_cuong: &["TC-Risk-Assessor", "TC-Financial-Planner", "TC-Audit-Logger", "TC-Performance-Analyst"],
        dia_sat: &["DS-Log-Analyzer", "DS-Backup-Worker", "DS-Benchmark-Tool"],
    },
    // NHAN: Interface, Culture, Vietnamese Localization
    RoutingRule {
        agent: GrandAgent::Nhan,
        keywords: &[
            "viet hoa", "localization", "tieng viet", "vietnamese",
            "van hoa", "culture", "cultural", "phong tuc", "truyen thong",
            "giao dien", "interface", "ui", "ux", "frontend",
            "dich thuat", "translation", "translate", "dich",
            "giao tiep", "communication", "email", "tin nhan",
            "telegram", "slack", "discord", "whatsapp", "zalo",
            "tai lieu", "documentation", "huong dan", "tutorial",
            "markdown", "bao cao van ban",
            "cam xuc", "emotion", "phan hoi", "feedback",
            "nguoi dung cuoi", "end user", "accessibility",
        ],
        base_score: 0.72,
        polarity: Polarity::Yang,
        thien_cuong: &["TC-Cultural-Advisor", "TC-Language-Expert", "TC-User-Experience", "TC-Legal-Advisor"],
        dia_sat: &["DS-Translation-Agent", "DS-Markdown-Generator", "DS-Email-Sender", "DS-Telegram-Bot", "DS-Sentiment-Analyst"],
    },
];

// ---------------------------------------------------------------------------
// Polarity Detection
// ---------------------------------------------------------------------------

static YANG_SIGNALS: &[&str] = &[
    "tao", "create", "build", "deploy", "trien khai", "gui", "send",
    "chay", "run", "execute", "thuc thi", "publish",
    "cai dat", "install", "setup", "khoi dong", "start", "launch",
    "viet", "write", "generate", "implement", "lap trinh",
    "fix", "sua", "update", "cap nhat", "migrate",
];

static YIN_SIGNALS: &[&str] = &[
    "phan tich", "analyze", "analysis", "danh gia", "evaluate",
    "nghien cuu", "research", "tim hieu", "investigate", "study",
    "kiem tra", "check", "verify", "audit", "review",
    "luu tru", "store", "save", "backup", "archive",
    "hoc", "learn", "training", "fine-tune",
    "bao cao", "report", "document", "ghi lai", "log",
    "so sanh", "compare", "benchmark", "do luong", "measure",
];

fn detect_polarity(input: &str) -> Polarity {
    let lower = input.to_lowercase();
    // Remove diacritics for matching (simple approach: match both original and ascii)
    let yang = YANG_SIGNALS.iter().filter(|&&s| lower.contains(s)).count();
    let yin  = YIN_SIGNALS.iter().filter(|&&s| lower.contains(s)).count();
    match yang.cmp(&yin) {
        std::cmp::Ordering::Greater => Polarity::Yang,
        std::cmp::Ordering::Less    => Polarity::Yin,
        std::cmp::Ordering::Equal   => Polarity::Neutral,
    }
}

// ---------------------------------------------------------------------------
// Orchestrator
// ---------------------------------------------------------------------------

/// Bộ điều phối trung tâm của Ochi — hiện thân của **Thái Cực**.
///
/// Phân tích đầu vào tự nhiên, tính điểm theo bảng routing rules Ngũ Hành,
/// phát hiện phân cực Âm Dương, và trả về [`DispatchDecision`].
pub struct Orchestrator {
    confidence_threshold: f32,
    ambiguity_gap: f32,
}

impl Default for Orchestrator {
    fn default() -> Self { Self::new() }
}

impl Orchestrator {
    /// Tạo Orchestrator với tham số mặc định.
    pub fn new() -> Self {
        Self { confidence_threshold: 0.45, ambiguity_gap: 0.10 }
    }

    /// Tạo Orchestrator với tham số tùy chỉnh.
    pub fn with_params(confidence_threshold: f32, ambiguity_gap: f32) -> Self {
        Self { confidence_threshold, ambiguity_gap }
    }

    /// Điều phối nhiệm vụ — API chính của Orchestrator.
    pub fn route(&self, task_description: &str) -> DispatchDecision {
        if task_description.trim().is_empty() {
            return DispatchDecision::default_to_thai_cuc(task_description);
        }

        let lower = task_description.to_lowercase();

        // Bước 1: Tính điểm cho từng rule
        let mut scores: Vec<(usize, f32, usize)> = ROUTING_RULES
            .iter()
            .enumerate()
            .map(|(idx, rule)| {
                let match_count = rule.keywords
                    .iter()
                    .filter(|&&kw| lower.contains(kw))
                    .count();
                let score = if match_count == 0 {
                    0.0_f32
                } else {
                    let boost = 1.0 + (match_count as f32 - 1.0) * 0.15;
                    (rule.base_score * boost).min(0.98)
                };
                (idx, score, match_count)
            })
            .collect();

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (top_idx, top_score, top_matches) = scores[0];
        let second_score = scores.get(1).map(|s| s.1).unwrap_or(0.0);

        // Bước 2: Phân cực Âm Dương
        let task_polarity = detect_polarity(task_description);
        let rule_polarity = ROUTING_RULES[top_idx].polarity;
        let polarity_penalty = if task_polarity != rule_polarity
            && task_polarity != Polarity::Neutral { 0.05_f32 } else { 0.0_f32 };
        let final_confidence = (top_score - polarity_penalty).max(0.0);

        // Bước 3: Kiểm tra mơ hồ
        let is_ambiguous = top_score > 0.0
            && (top_score - second_score) < self.ambiguity_gap
            && second_score > 0.0;

        // Bước 4: Leo thang
        let escalate = final_confidence < self.confidence_threshold || is_ambiguous;

        if top_score == 0.0 {
            return DispatchDecision::default_to_thai_cuc(task_description);
        }

        let selected_rule = &ROUTING_RULES[top_idx];

        // Bước 5: Sub-agent gợi ý
        let mut sub_agents: Vec<SubAgentSuggestion> = Vec::new();
        for &tc in selected_rule.thien_cuong {
            sub_agents.push(SubAgentSuggestion {
                name: tc.to_string(),
                kind: "thien_cuong".to_string(),
                reason: format!("Chuyen gia chien luoc cho {}", selected_rule.agent),
            });
        }
        for &ds in selected_rule.dia_sat {
            sub_agents.push(SubAgentSuggestion {
                name: ds.to_string(),
                kind: "dia_sat".to_string(),
                reason: format!("Agent thuc thi cho {}", selected_rule.agent),
            });
        }
        if is_ambiguous && scores.len() > 1 {
            let second_rule = &ROUTING_RULES[scores[1].0];
            for &ds in second_rule.dia_sat.iter().take(2) {
                sub_agents.push(SubAgentSuggestion {
                    name: ds.to_string(),
                    kind: "dia_sat".to_string(),
                    reason: format!(
                        "Goi y bo sung tu {} (nhiem vu co the thuoc ca hai linh vuc)",
                        second_rule.agent
                    ),
                });
            }
        }

        // Bước 6: Reasoning
        let reasoning = format!(
            "Khop {} tu khoa voi {} (score={:.2}, confidence={:.2}). Phan cuc: {}.{}{}",
            top_matches,
            selected_rule.agent,
            top_score,
            final_confidence,
            task_polarity,
            if is_ambiguous {
                format!(" [Mo ho: top-2={} score={:.2}]", ROUTING_RULES[scores[1].0].agent, second_score)
            } else { String::new() },
            if escalate { " -> Leo thang Thai Cuc." } else { "" },
        );

        DispatchDecision {
            primary: selected_rule.agent,
            confidence: final_confidence,
            polarity: task_polarity,
            suggested_sub_agents: sub_agents,
            escalate_to_thai_cuc: escalate,
            reasoning,
        }
    }

    /// API tương thích ngược — trả về GrandAgent trực tiếp.
    #[deprecated(since = "0.2.0", note = "Dung `route()` de nhan DispatchDecision day du")]
    pub fn dispatch(&self, task_description: &str) -> GrandAgent {
        self.route(task_description).primary
    }

    /// Kiểm tra xem nhiệm vụ có cần leo thang về Thái Cực không.
    pub fn needs_thai_cuc(&self, task_description: &str) -> bool {
        self.route(task_description).escalate_to_thai_cuc
    }

    /// Danh sách tất cả 9 Đại Tác Tử.
    pub fn all_grand_agents() -> &'static [GrandAgent] {
        &[
            GrandAgent::Kim, GrandAgent::Moc, GrandAgent::Thuy,
            GrandAgent::Hoa, GrandAgent::Tho, GrandAgent::ThaiCuc,
            GrandAgent::Can, GrandAgent::Khon, GrandAgent::Nhan,
        ]
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn orc() -> Orchestrator { Orchestrator::new() }

    #[test]
    fn test_kim_security() {
        let d = orc().route("Kiem tra lo hong bao mat he thong");
        assert_eq!(d.primary, GrandAgent::Kim);
        assert!(d.confidence > 0.5, "confidence={}", d.confidence);
    }

    #[test]
    fn test_kim_logic() {
        let d = orc().route("Phan tich logic va cau truc ma nguon Rust");
        assert_eq!(d.primary, GrandAgent::Kim);
    }

    #[test]
    fn test_thuy_lacviet() {
        let d = orc().route("Xu ly du lieu lon tu Lac Viet DB va tim kiem vector");
        assert_eq!(d.primary, GrandAgent::Thuy);
        assert!(d.confidence > 0.6);
    }

    #[test]
    fn test_thuy_memory() {
        let d = orc().route("Cap nhat tri nho dai han va luu tru knowledge base");
        assert_eq!(d.primary, GrandAgent::Thuy);
    }

    #[test]
    fn test_tho_aapanel() {
        let d = orc().route("Cau hinh aapanel cho server moi va deploy ung dung");
        assert_eq!(d.primary, GrandAgent::Tho);
        assert!(d.confidence > 0.6);
    }

    #[test]
    fn test_tho_docker() {
        let d = orc().route("Quan ly docker container va cai dat nginx reverse proxy");
        assert_eq!(d.primary, GrandAgent::Tho);
    }

    #[test]
    fn test_can_strategy() {
        let d = orc().route("Lap ke hoach tam nhin dai han cho du an ophiuchus");
        assert_eq!(d.primary, GrandAgent::Can);
    }

    #[test]
    fn test_nhan_localization() {
        let d = orc().route("Dich tai lieu sang tieng viet va toi uu van hoa dia phuong");
        assert_eq!(d.primary, GrandAgent::Nhan);
    }

    #[test]
    fn test_hoa_nlp() {
        let d = orc().route("Tich hop wit.ai de xu ly ngon ngu tu nhien tieng viet");
        assert_eq!(d.primary, GrandAgent::Hoa);
    }

    #[test]
    fn test_moc_research() {
        let d = orc().route("Nghien cuu va tao skill moi cho he thong AI");
        assert_eq!(d.primary, GrandAgent::Moc);
    }

    #[test]
    fn test_thai_cuc_default() {
        let d = orc().route("mot nhiem vu chung chung khong ro rang xyz");
        assert_eq!(d.primary, GrandAgent::ThaiCuc);
        assert!(d.escalate_to_thai_cuc);
    }

    #[test]
    fn test_empty_input() {
        let d = orc().route("");
        assert_eq!(d.primary, GrandAgent::ThaiCuc);
        assert!(d.escalate_to_thai_cuc);
    }

    #[test]
    fn test_multi_keyword_boosts_confidence() {
        let single = orc().route("bao mat he thong");
        let multi  = orc().route("kiem tra lo hong bao mat, audit code, dependency check, secret vault");
        assert!(multi.confidence >= single.confidence,
            "multi={}, single={}", multi.confidence, single.confidence);
    }

    #[test]
    fn test_confidence_in_range() {
        for task in &[
            "bao mat he thong",
            "lac viet db vector search",
            "deploy docker aapanel",
            "dich tieng viet",
            "tam nhin chien luoc dai han",
        ] {
            let d = orc().route(task);
            assert!(d.confidence >= 0.0 && d.confidence <= 1.0,
                "confidence={} for '{}'", d.confidence, task);
        }
    }

    #[test]
    fn test_sub_agents_not_empty() {
        let d = orc().route("kiem tra lo hong bao mat va audit code");
        assert!(!d.suggested_sub_agents.is_empty());
    }

    #[test]
    fn test_sub_agents_include_dia_sat() {
        let d = orc().route("deploy ung dung len server aapanel voi docker");
        assert!(d.suggested_sub_agents.iter().any(|s| s.kind == "dia_sat"));
    }

    #[test]
    fn test_sub_agents_include_thien_cuong() {
        let d = orc().route("lap ke hoach chien luoc dai han cho du an ophiuchus");
        assert!(d.suggested_sub_agents.iter().any(|s| s.kind == "thien_cuong"));
    }

    #[test]
    fn test_all_grand_agents_have_metadata() {
        for agent in Orchestrator::all_grand_agents() {
            assert!(!agent.description().is_empty(), "{} thieu description", agent);
            assert!(!agent.element().is_empty(), "{} thieu element", agent);
        }
    }

    #[test]
    fn test_display_format() {
        assert_eq!(GrandAgent::Kim.to_string(), "Ochi-KIM");
        assert_eq!(GrandAgent::ThaiCuc.to_string(), "Ochi-THAI-CUC");
        assert_eq!(GrandAgent::Nhan.to_string(), "Ochi-NHAN");
    }

    #[test]
    #[allow(deprecated)]
    fn test_dispatch_backward_compat() {
        let orc = Orchestrator::new();
        assert_eq!(orc.dispatch("kiem tra lo hong bao mat"), GrandAgent::Kim);
        assert_eq!(orc.dispatch("lac viet db vector search"), GrandAgent::Thuy);
        assert_eq!(orc.dispatch("deploy aapanel server"), GrandAgent::Tho);
        assert_eq!(orc.dispatch("nhiem vu khong ro xyz"), GrandAgent::ThaiCuc);
    }
}
