# Ochi — Kế hoạch Phát triển & Onboarding AI

> **Cập nhật lần cuối:** 07-03-2026
> **Commit mới nhất:** AGENT-01 (5 Địa Sát agents), AGENT-02 (2 Thiên Cương agents), DEBT-01-M (sdk-migration.md), DEBT-01-N (legacy deprecation warnings), DEBT-02-A (production unwrap fix in kernel)

## 1. Bối cảnh & Mục tiêu Dự án

**Ochi** là một hệ điều hành Agent tự trị (Agent OS) được xây dựng trên nền tảng Rust, kế thừa và cải tiến từ kiến trúc của OpenClaw và Ophiuchus--agent. Dự án tập trung vào việc xây dựng một hệ thống AI mạnh mẽ, linh hoạt và có khả năng tự điều phối dựa trên triết lý Ngũ Hành và Âm Dương.

**Mục tiêu cốt lõi:**

1. **Hoàn thiện Agent OS:** Xây dựng một nền tảng ổn định cho các agent tự trị hoạt động, bao gồm kernel, runtime, API, và các công cụ hỗ trợ.
2. **Kiến trúc Điều phối Thông minh:** Triển khai hệ thống điều phối 9 Đại Tác Tử (Ngũ Hành + Bát Quái) để tự động phân tích và định tuyến nhiệm vụ đến agent phù hợp nhất.
3. **Hệ sinh thái Agent Mở rộng:** Phát triển một hệ sinh thái gồm 36 Agent Chiến lược (Thiên Cương) và 72 Agent Thực thi (Địa Sát) để bao phủ một loạt các tác vụ phức tạp.
4. **Tích hợp Sâu rộng:** Kết nối với các dịch vụ bên thứ ba (như aaPanel, Lạc Việt DB, các nhà cung cấp NLP) thông qua giao thức MCP (Model Context Protocol).

## 2. Trạng thái Hiện tại (Tính đến `5c48ae4`)

| Hạng mục | Trạng thái | Mô tả |
| :--- | :--- | :--- |
| **Phase 3: Đổi tên** | ✅ **Hoàn tất** | Đổi tên thương hiệu toàn diện từ **OpenFang → Ochi** trên 13 crates, CI/CD, Docker, Scripts, Web UI, Docs, và SDK. |
| **Phase 6: Orchestration** | ✅ **Hoàn tất** | Triển khai logic điều phối 9 Đại Tác Tử vào `ochi-kernel`, tích hợp vào API với endpoint `/api/orchestrate`. 20/20 unit tests PASS. |
| **Launch Roadmap** | ⏳ **Đang tiến hành** | Hoàn thành 17/18 hạng mục trong 4 sprints. Chỉ còn **2.4 Install script domain** (hạ tầng) là PENDING. |
| **Workspace** | Ổn định | 14 crates `ochi-*` đã được đổi tên và build thành công (bao gồm `ochi-desktop`). |
| **DEBT-01: Đổi tên triệt để** | ✅ **Hoàn tất** | Đã xóa hoàn toàn mọi tham chiếu `openfang`/`OpenFang`/`OPENFANG` trong toàn bộ codebase (`.rs`, `.toml`, `.js`, `.py`). Chỉ còn `ochi-types-legacy/Cargo.toml` giữ `name = "openfang-types"` làm backward-compat shim. `cargo check --workspace --lib` PASS. |
| **DEBT-02: Xử lý `unwrap()`/`expect()`** | 🔴 **Cần làm** | ~1,500 lệnh `.unwrap()` và 80 lệnh `.expect()` trong toàn workspace. Nguy cơ `panic` trong production cao. |
| **IMPROVE-01: CI/CD Quality Gates** | ✅ **Hoàn tất** | Nâng cấp workflow CI để chạy `cargo check --workspace`, `cargo test --workspace`, và `cargo clippy --workspace -- -D warnings` trên mỗi PR. |
| **IMPROVE-03: Giảm kích thước Repo** | ✅ **Hoàn tất** | Xóa ~4.1MB ảnh logo cũ (`openfang-*`), gỡ schema Tauri tự sinh, cập nhật `.gitignore` + thêm `.gitattributes`. |

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
| `AGENT-01` | **Triển khai 5 Địa Sát đầu tiên** | ✅ **Hoàn tất** | `agents/dia_sat/ds-web-scraper/`, `agents/dia_sat/ds-file-manager/`, `agents/dia_sat/ds-git-operator/`, `agents/dia_sat/ds-shell-scripting/`, `agents/dia_sat/ds-docker-manager/` — Mỗi agent có `agent.toml` đầy đủ với system prompt, capabilities, fallback models. |
| `AGENT-02` | **Triển khai 2 Thiên Cương đầu tiên** | ✅ **Hoàn tất** | `agents/thien_cuong/tc-task-decomposer/`, `agents/thien_cuong/tc-quality-assurance/` — TC-Task-Decomposer phân rã nhiệm vụ và phân công cho Địa Sát; TC-Quality-Assurance review code và enforce standards. |

## 4. Hướng dẫn Onboarding cho AI mới

1. **Clone Repository:**

    ```bash
    gh repo clone lathaihoangmedia-lgtm/Ochi
    cd Ochi
    ```

2. **Đọc tài liệu quan trọng:**
    - `README.md`: Tổng quan dự án.
    - `docs/architecture.md`: Kiến trúc tổng thể.
    - `docs/ochi-full-rename-checklist.md`: Lịch sử đổi tên và các quyết định kỹ thuật.
    - `docs/launch-roadmap.md`: Lộ trình ra mắt sản phẩm.
    - `crates/ochi-kernel/src/orchestration.rs`: Logic điều phối 9 Đại Tác Tử.
3. **Thiết lập môi trường:**
    - Cài đặt Rust toolchain theo file `rust-toolchain.toml`.
    - Cài đặt các dependency hệ thống (build-essential, libssl-dev, etc.).
4. **Chạy kiểm tra:**

    ```bash
    # Kiểm tra build các crate cốt lõi
    cargo check -p ochi-kernel -p ochi-api -p ochi-cli

    # Chạy unit tests cho kernel
    cargo test -p ochi-kernel
    ```

5. **Chọn một nhiệm vụ từ Roadmap (Luồng 2 hoặc 3) và bắt đầu phát triển.**

## 5. Phụ lục: Danh sách Agent

- [Danh sách 9 Đại Tác Tử](agents/grand_agents/)
- [Danh sách 36 Thiên Cương](agents/thien_cuong/LIST.md)
- [Danh sách 72 Địa Sát](agents/dia_sat/LIST.md)

## 6. Giao thức AI Builder (Bắt buộc)

Để đảm bảo sự phối hợp nhịp nhàng và tiến độ dự án được minh bạch, tất cả các AI builder tham gia vào dự án Ochi **BẮT BUỘC** phải tuân thủ các quy tắc sau:

### 6.1. Tôn trọng Kế hoạch

- **Không tự ý thay đổi Roadmap:** Luôn bám sát các nhiệm vụ đã được định nghĩa trong **Mục 3: Roadmap & Nhiệm vụ Tiếp theo**.
- **Không làm ngoài phạm vi:** Chỉ thực hiện các công việc được mô tả trong Task ID đã chọn. Nếu phát hiện vấn đề hoặc có ý tưởng cải tiến, hãy tạo một commit riêng với tiền tố `chore(suggestion):` và mô tả rõ ràng, sau đó tiếp tục công việc chính.
- **Ưu tiên theo luồng:** Hoàn thành các nhiệm vụ trong Luồng 1 trước khi chuyển sang Luồng 2 và 3, trừ khi có chỉ định khác.

### 6.2. Tự động Cập nhật Tiến độ

- **Commit cho mỗi Task:** Mỗi Task ID hoàn thành phải tương ứng với một commit duy nhất. Commit message phải có định dạng:

    ```text
    feat(scope): [Task ID] Mô tả ngắn gọn

    Mô tả chi tiết các thay đổi, lý do, và kết quả.

    Refs: #<issue_number_nếu_có>
    ```

    *Ví dụ:* `feat(infra): [INFRA-01] Set up ochi.sh domain`

- **Tự động cập nhật PLAN-TASK.md:** Sau khi push commit thành công, AI builder **PHẢI** tự động đọc lại file `PLAN-TASK.md` này, đánh dấu `✅ Hoàn tất` vào Task ID tương ứng, và cập nhật commit mới nhất ở đầu file. Đây là một phần không thể thiếu của quy trình.

### 6.3. Phối hợp Đa AI

- **Kiểm tra trước khi làm:** Trước khi bắt đầu một Task, luôn `git pull` và đọc lại `PLAN-TASK.md` để đảm bảo không có AI nào khác đang thực hiện cùng nhiệm vụ.
- **Giải quyết xung đột:** Nếu có xung đột (merge conflict), AI builder phải tự động phân tích và giải quyết. Ưu tiên giữ lại các thay đổi logic và cấu trúc mới nhất. Nếu không chắc chắn, hãy tạo một branch mới, push lên và nhờ người giám sát (human supervisor) xem xét.

## 7. Pre-release Checklist (v0.1.0)

Đây là danh sách các hạng mục **BẮT BUỘC** phải hoàn thành trước khi có thể tag `v0.1.0` và phát hành bản pre-release đầu tiên. Các AI builder phải ưu tiên hoàn thành các mục này trước khi thực hiện các task trong Luồng 2 và 3.

| Task ID | Hạng mục | Trạng thái | Chi tiết | Files liên quan |
| :--- | :--- | :--- | :--- | :--- |
| `PRE-01` | **Tauri Signing Keypair** | ⏳ **PENDING** | Tạo keypair Ed25519 để ký các bản build desktop, nếu không auto-updater sẽ không hoạt động. | `cargo tauri signer generate` |
| `PRE-02` | **Set Public Key** | ⏳ **PENDING** | Chèn public key đã tạo vào `tauri.conf.json`. | `crates/ochi-desktop/tauri.conf.json` |
| `PRE-03` | **GitHub Secrets** | ⏳ **PENDING** | Thêm `TAURI_SIGNING_PRIVATE_KEY` vào secrets của repo GitHub để CI/CD có thể ký release. | GitHub Repo Settings |
| `PRE-04` | **Setup `ochi.sh` domain** | ⏳ **PENDING** | Cấu hình domain để người dùng có thể cài đặt bằng `curl`. Đây là hạng mục `INFRA-01` được nâng lên ưu tiên cao nhất. | `scripts/install.sh` |
| `PRE-05` | **Update `CHANGELOG.md`** | ✅ **Hoàn tất** | Cập nhật file `CHANGELOG.md` để phản ánh tất cả các thay đổi lớn từ khi bắt đầu dự án. | `CHANGELOG.md` |
| `PRE-06` | **Version Bump** | ✅ **Hoàn tất** | Tăng version trong `Cargo.toml` và `tauri.conf.json` lên `0.1.0`. | `Cargo.toml`, `crates/ochi-desktop/tauri.conf.json` |
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

---

## 9. Checklist Nợ Kỹ thuật (Technical Debt)

Các mục trong section này là **nợ kỹ thuật ưu tiên cao** được xác định qua audit ngày 05-03-2026. Mỗi AI builder chỉ nhận **một Task ID** tại một thời điểm. Tuân thủ quy tắc tại **Mục 6** trước khi bắt đầu.

---

### DEBT-01: Đổi tên Triệt để (OpenFang → Ochi)

> **Mục tiêu:** Loại bỏ hoàn toàn mọi tham chiếu đến `OpenFang`/`openfang`/`OPENFANG` trong toàn bộ codebase, đảm bảo tính nhất quán thương hiệu và tránh lỗi cấu hình.
>
> **Phạm vi:** 1,271+ lần xuất hiện trong `.rs`, 26 lần trong `.toml`, 26 lần trong `.md`, và nhiều file `.js`/`.py`.
>
> **Quy tắc phân công:** Mỗi AI builder nhận **một sub-task ID** (ví dụ `DEBT-01-A`). Kiểm tra bảng bên dưới trước khi bắt đầu để tránh xung đột.

#### Phân công Sub-tasks

| Sub-task ID | Phạm vi | Trạng thái | Files liên quan |
| :--- | :--- | :--- | :--- |
| `DEBT-01-A` | **Đổi tên crate `openfang-desktop` → `ochi-desktop`** | ✅ **Hoàn tất** (`6495049`) | `crates/openfang-desktop/` đã được đổi tên thành `crates/ochi-desktop/`. Cập nhật Cargo.toml (tên crate, dependency alias, binary name), tauri.conf.json (productName, identifier, updater endpoint, description), và toàn bộ `src/*.rs` (strings, comments, use declarations). |
| `DEBT-01-B` | **Dọn `openfang` trong `crates/ochi-runtime/`** | ✅ **Hoàn tất** | Toàn bộ `crates/ochi-runtime/src/*.rs` — đã đổi `openfang_types::` → `ochi_types::`, cập nhật Cargo.toml alias. Thêm `urlencoding` dep và sửa import `warn` trong `wit.rs`. |
| `DEBT-01-C` | **Dọn `openfang` trong `crates/ochi-kernel/`** | ✅ **Hoàn tất** | Toàn bộ `crates/ochi-kernel/src/*.rs` — đã đổi `openfang_types::` → `ochi_types::`, `OpenFangKernel` → `OchiKernel`. |
| `DEBT-01-D` | **Dọn `openfang` trong `crates/ochi-api/`** | ✅ **Hoàn tất** | Toàn bộ `crates/ochi-api/src/*.rs` và `crates/ochi-api/static/js/` — đã đổi toàn bộ references. |
| `DEBT-01-E` | **Dọn `openfang` trong `crates/ochi-channels/`** | ✅ **Hoàn tất** | Toàn bộ `crates/ochi-channels/src/*.rs` — đã đổi toàn bộ references. |
| `DEBT-01-F` | **Dọn `openfang` trong `crates/ochi-migrate/` và `crates/ochi-types-legacy/`** | ✅ **Hoàn tất** | `crates/ochi-migrate/src/*.rs`, `crates/ochi-types-legacy/src/*.rs` — đã đổi toàn bộ source references. Giữ `name = "openfang-types"` trong `ochi-types-legacy/Cargo.toml` như backward-compat shim. |
| `DEBT-01-G` | **Dọn `openfang` trong `crates/ochi-memory/`, `ochi-skills/`, `ochi-extensions/`** | ✅ **Hoàn tất** | `crates/ochi-memory/src/*.rs`, `crates/ochi-skills/src/*.rs`, `crates/ochi-extensions/src/*.rs` — đã đổi toàn bộ references. |
| `DEBT-01-H` | **Dọn `openfang` trong `crates/ochi-cli/`, `ochi-wire/`, `ochi-hands/`, `xtask/`** | ✅ **Hoàn tất** | `crates/ochi-cli/src/*.rs`, `crates/ochi-wire/src/*.rs`, `crates/ochi-hands/src/*.rs`, `xtask/src/*.rs` — đã đổi toàn bộ references. |
| `DEBT-01-I` | **Dọn `openfang` trong tất cả `.toml` files** | ✅ **Hoàn tất** | Tất cả `crates/*/Cargo.toml` — đã đổi alias `openfang-types = { package = "ochi-types" }` → `ochi-types = { path = "../ochi-types" }`. |
| `DEBT-01-J` | **Dọn `openfang` trong SDK và packages** | ✅ **Hoàn tất** | `sdk/python/*.py`, `sdk/python/examples/*.py`, `sdk/javascript/index.js`, `packages/whatsapp-gateway/index.js` — Giữ nguyên tên file `openfang_client.py` và `openfang_sdk.py` như alias backward-compat, chỉ cập nhật nội dung bên trong. |
| `DEBT-01-K` | **Dọn `openfang` trong `.env.example` và `deploy/`** | ✅ **Hoàn tất** (`c0b7f69`) | `.env.example` (cập nhật tên biến `OPENFANG_*` → `OCHI_*` với chú thích backward-compat), `deploy/openfang.service` → `deploy/ochi.service` |

| `DEBT-01-L` | **Chạy full test matrix sau mỗi batch rename** | ✅ **Hoàn tất** | `cargo check --workspace --lib` PASS; `cargo test -p ochi-kernel -p ochi-types` → 273 tests PASS. |
| `DEBT-01-M` | **Cập nhật docs publish/migration cho SDK** | ✅ **Hoàn tất** | `docs/sdk-migration.md` — Viết lại hoàn toàn: bảng thay đổi, hướng dẫn migration JS/Python/envvars/service, lịch trình deprecation, checklist migration. |
| `DEBT-01-N` | **Thêm Telemetry/log warning cho đường dẫn legacy** | ✅ **Hoàn tất** | `crates/ochi-kernel/src/config.rs` — `ochi_home()` đã thêm `tracing::warn!` cho `OPENFANG_HOME` env var và `~/.openfang` fallback directory; đồng thời sửa duplicate code. |
| `DEBT-01-O` | **Chốt ngày dừng hỗ trợ alias `openfang`** | ⚪️ **Chưa bắt đầu** | Roadmap & Release Plan |
| `DEBT-01-P` | **Xóa shim/bí danh legacy** | ⚪️ **Chưa bắt đầu** | Toàn bộ codebase |
| `DEBT-01-Q` | **Hoàn thiện release note & migration guide** | ⚪️ **Chưa bắt đầu** | `CHANGELOG.md`, `docs/sdk-migration.md` |

#### Tiêu chí Hoàn thành DEBT-01

Nhiệm vụ DEBT-01 được coi là **HOÀN TẤT** khi:
- `grep -r "openfang\|OpenFang\|OPENFANG" --include="*.rs" . | wc -l` trả về `0`.
- `grep -r "openfang\|OpenFang\|OPENFANG" --include="*.toml" . | wc -l` trả về `0` (ngoại trừ các comment backward-compat có ghi chú rõ ràng).
- `cargo check --workspace` vẫn PASS sau tất cả thay đổi.
- `cargo test -p ochi-kernel -p ochi-types` vẫn PASS.

---

### DEBT-02: Xử lý `unwrap()` và `expect()` An toàn

> **Mục tiêu:** Thay thế các lệnh gọi `.unwrap()` và `.expect()` không an toàn bằng các cấu trúc xử lý lỗi phù hợp, ngăn ngừa `panic` trong production.
>
> **Nguyên tắc thay thế:**
> - Trong **production code** (không phải test): dùng `?`, `if let`, `match`, hoặc `.unwrap_or_else(|e| { tracing::error!(...); default_value })`.
> - Trong **test code** (`#[cfg(test)]`): `.unwrap()` và `.expect()` được chấp nhận.
> - Trong **`main.rs` hoặc `bin` entry points**: `.expect("message rõ ràng")` được chấp nhận nếu là lỗi không thể phục hồi khi khởi động.
> - Ưu tiên xử lý các crate **kernel** và **runtime** trước vì chúng là critical path.
>
> **Quy tắc phân công:** Mỗi AI builder nhận **một sub-task ID**. Kiểm tra bảng trước khi bắt đầu.

#### Phân công Sub-tasks

| Sub-task ID | Crate mục tiêu | Số lượng ước tính | Trạng thái | Ưu tiên |
| :--- | :--- | :--- | :--- | :--- |
| `DEBT-02-A` | **`crates/ochi-kernel/`** | ~209 `unwrap` + 29 `expect` | ✅ **Hoàn tất (production code)** | 🔴 Cao nhất — Sửa `kernel.rs`: dùng constant `DEFAULT_LISTEN_ADDR` + `.expect()` cho infallible parse thay vì `.unwrap_or_else(|_| "...".parse().unwrap())`; sửa `config.rs`: loại bỏ duplicate code trong `ochi_home()`. Các `.unwrap()` còn lại đều nằm trong `#[test]` blocks (chấp nhận được). |
| `DEBT-02-B` | **`crates/ochi-runtime/`** (phần `agent_loop.rs`, `tool_runner.rs`, `compactor.rs`) | ~100 `unwrap` (3 files nặng nhất) | ⏳ **PENDING** | 🔴 Cao nhất |
| `DEBT-02-C` | **`crates/ochi-runtime/`** (phần còn lại) | ~155 `unwrap` + 16 `expect` | ⏳ **PENDING** | 🟠 Cao |
| `DEBT-02-D` | **`crates/ochi-api/`** | ~196 `unwrap` + 16 `expect` | ⏳ **PENDING** | 🟠 Cao |
| `DEBT-02-E` | **`crates/ochi-channels/`** | ~157 `unwrap` + 3 `expect` | ⏳ **PENDING** | 🟡 Trung bình |
| `DEBT-02-F` | **`crates/ochi-memory/`** | ~125 `unwrap` | ⏳ **PENDING** | 🟡 Trung bình |
| `DEBT-02-G` | **`crates/ochi-migrate/`** | ~137 `unwrap` | ⏳ **PENDING** | 🟡 Trung bình |
| `DEBT-02-H` | **`crates/ochi-types-legacy/`** | ~127 `unwrap` | ⏳ **PENDING** | 🟡 Trung bình |
| `DEBT-02-I` | **`crates/ochi-skills/`, `ochi-extensions/`, `ochi-hands/`** | ~84 + 76 + 41 `unwrap` | ⏳ **PENDING** | 🟢 Thấp |
| `DEBT-02-J` | **`crates/ochi-cli/`, `ochi-wire/`, `ochi-desktop/`, `xtask/`** | ~51 + 42 + 7 `expect` | ⏳ **PENDING** | 🟢 Thấp |

#### Quy trình Thực hiện cho mỗi Sub-task

```bash
# 1. Đếm số lượng trước khi bắt đầu (để ghi vào commit message)
grep -rn "\.unwrap()\|\.expect(" crates/<tên-crate>/src/ --include="*.rs" | grep -v "#\[cfg(test)\]" | wc -l

# 2. Thực hiện thay thế từng file, ưu tiên các file có nhiều lần nhất
# Ví dụ thay thế an toàn:
#   TRƯỚC: let val = some_option.unwrap();
#   SAU:   let val = some_option.ok_or(MyError::Missing)?;
#   HOẶC:  let Some(val) = some_option else { return Err(MyError::Missing); };

# 3. Kiểm tra sau mỗi file
cargo check -p <tên-crate>

# 4. Chạy test trước khi commit
cargo test -p <tên-crate>
```

#### Tiêu chí Hoàn thành DEBT-02

Nhiệm vụ DEBT-02 được coi là **HOÀN TẤT** khi:
- `grep -rn "\.unwrap()" crates/ --include="*.rs" | grep -v "#\[cfg(test)\]\|//.*unwrap\|test_"` trả về số lượng giảm ít nhất **80%** so với ban đầu (~1,500 → dưới 300).
- `cargo clippy --workspace -- -W clippy::unwrap_used 2>&1 | grep "unwrap_used" | wc -l` giảm đáng kể.
- `cargo test --workspace` vẫn PASS.
- Không có `panic` nào được ghi nhận trong integration test.

---

> **Lưu ý cho AI Builder:** Sau khi hoàn thành bất kỳ sub-task nào, hãy cập nhật trạng thái trong bảng tương ứng từ `⏳ PENDING` thành `✅ Hoàn tất` và ghi rõ commit hash. Đây là bắt buộc theo **Mục 6.2**.

---

## 10. Cải tiến Quy trình & Nợ Kỹ thuật Tồn đọng

Phần này tổng hợp các nhiệm vụ cải tiến quy trình và xử lý nợ kỹ thuật được rút ra từ các cuộc đánh giá (`repo-audit-5w1h.md`) và báo cáo chuyên sâu. Các nhiệm vụ này cần được ưu tiên để nâng cao chất lượng và độ ổn định của dự án.

| Task ID | Nhiệm vụ | Trạng thái | Chi tiết | Files liên quan |
| :--- | :--- | :--- | :--- | :--- |
| `IMPROVE-01` | **Nâng cấp CI/CD Quality Gates** | ✅ **Hoàn tất** (`5c48ae4`) | Mở rộng workflow `.github/workflows/rust.yml` để chạy `cargo check --workspace`, `cargo test --workspace` và `cargo clippy --workspace -- -D warnings` trên mỗi PR. Tách thành 3 jobs song song: `check`, `test`, `clippy`. | `.github/workflows/rust.yml` |
| `IMPROVE-02` | **Dọn dẹp Cảnh báo (`warnings`)** | ✅ **Hoàn tất** | Đã xóa import `warn` không dùng trong `ochi-runtime/src/manus.rs` và import `GrandAgent` không dùng trong `ochi-api/src/routes.rs`. Build sạch không còn warning trong code của dự án. | `crates/ochi-runtime/`, `crates/ochi-api/` |
| `IMPROVE-03` | **Giảm kích thước Repository** | ✅ **Hoàn tất** (`5c48ae4`) | Xóa các file nhị phân cũ không còn sử dụng (~4.1MB ảnh logo `openfang-*`), gỡ bỏ các schema Tauri tự sinh (`crates/ochi-desktop/gen/`), cập nhật `.gitignore` để chặn commit ảnh lớn trong tương lai, thêm `.gitattributes` cho line endings và binary markers. | `.gitignore`, `.gitattributes`, `public/assets/` |

---

## 11. Roadmap Tương lai

Phần này ghi nhận các định hướng và ý tưởng phát triển trong dài hạn, được thu thập từ các tài liệu định hướng chiến lược.

| Task ID | Nhiệm vụ | Trạng thái | Chi tiết | Nguồn |
| :--- | :--- | :--- | :--- | :--- |
| `FUTURE-01` | **Tích hợp Hyperledger Fabric** | ⚪️ **Chưa bắt đầu** | Tích hợp blockchain Hyperledger Fabric để cung cấp các tính năng audit trail, provenance, và notarization. | `docs/hyperledger-fabric-policy.md` |
| `FUTURE-02` | **Mở rộng hỗ trợ Wit.ai** | ⚪️ **Chưa bắt đầu** | Hỗ trợ các endpoint `POST /speech`, `POST /dictation`, và `POST /synthesize` của Wit.ai để tăng cường khả năng xử lý âm thanh. | `docs/integrations/wit-ai-integration-notes.md` |
