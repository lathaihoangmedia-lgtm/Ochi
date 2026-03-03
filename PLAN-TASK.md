# Ochi — Kế hoạch Phát triển & Onboarding AI

> **Cập nhật lần cuối:** 03-03-2026
> **Commit mới nhất:** `436d82d` (feat(orchestration): Phase 6 — wire 9 Grand Agents into ochi-kernel)

## 1. Bối cảnh & Mục tiêu Dự án

**Ochi** là một hệ điều hành Agent tự trị (Agent OS) được xây dựng trên nền tảng Rust, kế thừa và cải tiến từ kiến trúc của OpenClaw và Ophiuchus--agent. Dự án tập trung vào việc xây dựng một hệ thống AI mạnh mẽ, linh hoạt và có khả năng tự điều phối dựa trên triết lý Ngũ Hành và Âm Dương.

**Mục tiêu cốt lõi:**

1.  **Hoàn thiện Agent OS:** Xây dựng một nền tảng ổn định cho các agent tự trị hoạt động, bao gồm kernel, runtime, API, và các công cụ hỗ trợ.
2.  **Kiến trúc Điều phối Thông minh:** Triển khai hệ thống điều phối 9 Đại Tác Tử (Ngũ Hành + Bát Quái) để tự động phân tích và định tuyến nhiệm vụ đến agent phù hợp nhất.
3.  **Hệ sinh thái Agent Mở rộng:** Phát triển một hệ sinh thái gồm 36 Agent Chiến lược (Thiên Cương) và 72 Agent Thực thi (Địa Sát) để bao phủ một loạt các tác vụ phức tạp.
4.  **Tích hợp Sâu rộng:** Kết nối với các dịch vụ bên thứ ba (như aaPanel, Lạc Việt DB, các nhà cung cấp NLP) thông qua giao thức MCP (Model Context Protocol).

## 2. Trạng thái Hiện tại (Tính đến `436d82d`)

| Hạng mục | Trạng thái | Mô tả |
| :--- | :--- | :--- |
| **Phase 3: Đổi tên** | ✅ **Hoàn tất** | Đổi tên thương hiệu toàn diện từ **OpenFang → Ochi** trên 13 crates, CI/CD, Docker, Scripts, Web UI, Docs, và SDK. | 
| **Phase 6: Orchestration** | ✅ **Hoàn tất** | Triển khai logic điều phối 9 Đại Tác Tử vào `ochi-kernel`, tích hợp vào API với endpoint `/api/orchestrate`. 20/20 unit tests PASS. |
| **Launch Roadmap** | ⏳ **Đang tiến hành** | Hoàn thành 17/18 hạng mục trong 4 sprints. Chỉ còn **2.4 Install script domain** (hạ tầng) là PENDING. |
| **Workspace** | Ổn định | 13 crates `ochi-*` đã được đổi tên và build thành công (không có `desktop`). | 

## 3. Roadmap & Nhiệm vụ Tiếp theo

Dựa trên `docs/launch-roadmap.md` và `docs/ochi-brand-roadmap.md`, các nhiệm vụ ưu tiên tiếp theo được chia thành các luồng chính.

### Luồng 1: Hoàn thiện Hạ tầng & Triển khai

Đây là ưu tiên hàng đầu để đưa Ochi đến tay người dùng cuối.

| Task ID | Nhiệm vụ | Chi tiết | Files liên quan |
| :--- | :--- | :--- | :--- |
| `INFRA-01` | **Thiết lập domain `ochi.sh`** | Cấu hình GitHub Pages hoặc Cloudflare Worker để phục vụ script cài đặt tại `https://ochi.sh`. | `scripts/install.sh`, `scripts/install.ps1` |

### Luồng 2: Tích hợp Chức năng cho Đại Tác Tử

Đây là bước hiện thực hóa năng lực của các Đại Tác Tử đã được wiring trong Phase 6.

| Task ID | Nhiệm vụ | Đại Tác Tử | Chi tiết | Files liên quan |
| :--- | :--- | :--- | :--- | :--- |
| `FUNC-01` | **Tích hợp NLP tiếng Việt** | `Ochi-HOA` | Tích hợp Wit.ai và Zalo AI làm provider mặc định cho các tác vụ xử lý ngôn ngữ tự nhiên. | `crates/ochi-kernel/src/providers/` |
| `FUNC-02` | **Kết nối Lạc Việt DB** | `Ochi-THUY` | Kết nối với vector store thực tế để thực hiện các tác vụ tìm kiếm ngữ nghĩa và quản lý bộ nhớ dài hạn. | `crates/ochi-memory/`, `crates/ochi-kernel/src/memory.rs` |
| `FUNC-03` | **Tích hợp aaPanel MCP** | `Ochi-THO` | Kết nối với `aaPanel/mcp-server` để quản lý hạ tầng (deploy, monitor, backup). | `crates/ochi-skills/src/mcp/` |

### Luồng 3: Phát triển Hệ sinh thái Agent

Triển khai các agent chuyên trách để mở rộng khả năng của hệ thống.

| Task ID | Nhiệm vụ | Chi tiết | Files liên quan |
| :--- | :--- | :--- | :--- |
| `AGENT-01` | **Triển khai 5 Địa Sát đầu tiên** | Chọn và triển khai 5 agent thực thi từ danh sách 72 Địa Sát. Gợi ý: `DS-Web-Scraper`, `DS-File-Manager`, `DS-Git-Operator`, `DS-Shell-Scripting`, `DS-Docker-Manager`. | `agents/dia_sat/`, `crates/ochi-skills/` |
| `AGENT-02` | **Triển khai 2 Thiên Cương đầu tiên** | Chọn và triển khai 2 agent chiến lược từ danh sách 36 Thiên Cương. Gợi ý: `TC-Task-Decomposer`, `TC-Code-Reviewer`. | `agents/thien_cuong/`, `crates/ochi-runtime/` |

## 4. Hướng dẫn Onboarding cho AI mới

1.  **Clone Repository:**
    ```bash
    gh repo clone lathaihoangmedia-lgtm/Ochi
    cd Ochi
    ```
2.  **Đọc tài liệu quan trọng:**
    - `README.md`: Tổng quan dự án.
    - `docs/architecture.md`: Kiến trúc tổng thể.
    - `docs/ochi-full-rename-checklist.md`: Lịch sử đổi tên và các quyết định kỹ thuật.
    - `docs/launch-roadmap.md`: Lộ trình ra mắt sản phẩm.
    - `crates/ochi-kernel/src/orchestration.rs`: Logic điều phối 9 Đại Tác Tử.
3.  **Thiết lập môi trường:**
    - Cài đặt Rust toolchain theo file `rust-toolchain.toml`.
    - Cài đặt các dependency hệ thống (build-essential, libssl-dev, etc.).
4.  **Chạy kiểm tra:**
    ```bash
    # Kiểm tra build các crate cốt lõi
    cargo check -p ochi-kernel -p ochi-api -p ochi-cli

    # Chạy unit tests cho kernel
    cargo test -p ochi-kernel
    ```
5.  **Chọn một nhiệm vụ từ Roadmap (Luồng 2 hoặc 3) và bắt đầu phát triển.**

## 5. Phụ lục: Danh sách Agent

-   [Danh sách 9 Đại Tác Tử](agents/grand_agents/)
-   [Danh sách 36 Thiên Cương](agents/thien_cuong/LIST.md)
-   [Danh sách 72 Địa Sát](agents/dia_sat/LIST.md)
