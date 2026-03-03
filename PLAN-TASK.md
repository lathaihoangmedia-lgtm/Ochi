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


## 6. Giao thức AI Builder (Bắt buộc)

Để đảm bảo sự phối hợp nhịp nhàng và tiến độ dự án được minh bạch, tất cả các AI builder tham gia vào dự án Ochi **BẮT BUỘC** phải tuân thủ các quy tắc sau:

### 6.1. Tôn trọng Kế hoạch

-   **Không tự ý thay đổi Roadmap:** Luôn bám sát các nhiệm vụ đã được định nghĩa trong **Mục 3: Roadmap & Nhiệm vụ Tiếp theo**. 
-   **Không làm ngoài phạm vi:** Chỉ thực hiện các công việc được mô tả trong Task ID đã chọn. Nếu phát hiện vấn đề hoặc có ý tưởng cải tiến, hãy tạo một commit riêng với tiền tố `chore(suggestion):` và mô tả rõ ràng, sau đó tiếp tục công việc chính.
-   **Ưu tiên theo luồng:** Hoàn thành các nhiệm vụ trong Luồng 1 trước khi chuyển sang Luồng 2 và 3, trừ khi có chỉ định khác.

### 6.2. Tự động Cập nhật Tiến độ

-   **Commit cho mỗi Task:** Mỗi Task ID hoàn thành phải tương ứng với một commit duy nhất. Commit message phải có định dạng:
    ```
    feat(scope): [Task ID] Mô tả ngắn gọn

    Mô tả chi tiết các thay đổi, lý do, và kết quả.

    Refs: #<issue_number_nếu_có>
    ```
    *Ví dụ:* `feat(infra): [INFRA-01] Set up ochi.sh domain`

-   **Tự động cập nhật PLAN-TASK.md:** Sau khi push commit thành công, AI builder **PHẢI** tự động đọc lại file `PLAN-TASK.md` này, đánh dấu `✅ Hoàn tất` vào Task ID tương ứng, và cập nhật commit mới nhất ở đầu file. Đây là một phần không thể thiếu của quy trình.

### 6.3. Phối hợp Đa AI

-   **Kiểm tra trước khi làm:** Trước khi bắt đầu một Task, luôn `git pull` và đọc lại `PLAN-TASK.md` để đảm bảo không có AI nào khác đang thực hiện cùng nhiệm vụ.
-   **Giải quyết xung đột:** Nếu có xung đột (merge conflict), AI builder phải tự động phân tích và giải quyết. Ưu tiên giữ lại các thay đổi logic và cấu trúc mới nhất. Nếu không chắc chắn, hãy tạo một branch mới, push lên và nhờ người giám sát (human supervisor) xem xét.


## 7. Pre-release Checklist (v0.1.0)

Đây là danh sách các hạng mục **BẮT BUỘC** phải hoàn thành trước khi có thể tag `v0.1.0` và phát hành bản pre-release đầu tiên. Các AI builder phải ưu tiên hoàn thành các mục này trước khi thực hiện các task trong Luồng 2 và 3.

| Task ID | Hạng mục | Trạng thái | Chi tiết | Files liên quan |
| :--- | :--- | :--- | :--- | :--- |
| `PRE-01` | **Tauri Signing Keypair** | ⏳ **PENDING** | Tạo keypair Ed25519 để ký các bản build desktop, nếu không auto-updater sẽ không hoạt động. | `cargo tauri signer generate` |
| `PRE-02` | **Set Public Key** | ⏳ **PENDING** | Chèn public key đã tạo vào `tauri.conf.json`. | `crates/ochi-desktop/tauri.conf.json` |
| `PRE-03` | **GitHub Secrets** | ⏳ **PENDING** | Thêm `TAURI_SIGNING_PRIVATE_KEY` vào secrets của repo GitHub để CI/CD có thể ký release. | GitHub Repo Settings |
| `PRE-04` | **Setup `ochi.sh` domain** | ⏳ **PENDING** | Cấu hình domain để người dùng có thể cài đặt bằng `curl`. Đây là hạng mục `INFRA-01` được nâng lên ưu tiên cao nhất. | `scripts/install.sh` |
| `PRE-05` | **Update `CHANGELOG.md`** | ⏳ **PENDING** | Cập nhật file `CHANGELOG.md` để phản ánh tất cả các thay đổi lớn từ khi bắt đầu dự án. | `CHANGELOG.md` |
| `PRE-06` | **Version Bump** | ⏳ **PENDING** | Tăng version trong `Cargo.toml` và `tauri.conf.json` lên `0.1.0`. | `Cargo.toml`, `crates/ochi-desktop/tauri.conf.json` |
| `PRE-07` | **Final Verification** | ⏳ **PENDING** | Sau khi tag, kiểm tra lại tất cả các artifacts trên trang GitHub Release (installer, binary, checksum). | GitHub Release Page |


## 8. Chiến lược Pre-release (v0.1.0)

**Cập nhật quan trọng:** Do vấn đề với build desktop (Tauri), chúng ta sẽ pre-release theo chiến lược 2 giai đoạn để đưa sản phẩm đến tay người dùng sớm nhất có thể.

### Giai đoạn 1: Pre-release Web-only (Ưu tiên cao nhất)

- **Mục tiêu:** Phát hành phiên bản `v0.1.0-web` cho phép người dùng cài đặt qua `curl` hoặc `cargo` và sử dụng toàn bộ tính năng qua giao diện web tại `http://localhost:4200`.
- **Các mục cần hoàn thành (từ checklist trên):**
  - `PRE-04`: Setup `ochi.sh` domain.
  - `PRE-05`: Update `CHANGELOG.md`.
  - `PRE-06`: Version Bump lên `0.1.0`.
- **Các mục được hoãn lại:** `PRE-01`, `PRE-02`, `PRE-03` (liên quan đến Tauri desktop).

### Giai đoạn 2: Desktop Release (Sau)

- **Mục tiêu:** Phát hành phiên bản desktop đầy đủ (`v0.1.0-desktop`) sau khi đã giải quyết các vấn đề về signing key và build trên Windows.
- **Các mục cần hoàn thành:** `PRE-01`, `PRE-02`, `PRE-03`.
