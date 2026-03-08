# Ochi — Sub Plan-Task: Chi tiết Nợ Kỹ thuật

> **File này chứa chi tiết phân công sub-tasks cho các nhiệm vụ nợ kỹ thuật.**
> Được tách ra từ `PLAN-TASK.md` để dễ quản lý và rollback khi cần.
>
> **Tham chiếu chính:** [PLAN-TASK.md](PLAN-TASK.md) — Mục 9: Checklist Nợ Kỹ thuật
>
> **Cập nhật lần cuối:** 08-03-2026

---

## 1. DEBT-01: Đổi tên Triệt để (OpenFang → Ochi)

> **Mục tiêu:** Loại bỏ hoàn toàn mọi tham chiếu đến `OpenFang`/`openfang`/`OPENFANG` trong toàn bộ codebase, đảm bảo tính nhất quán thương hiệu và tránh lỗi cấu hình.
>
> **Phạm vi:** 1,271+ lần xuất hiện trong `.rs`, 26 lần trong `.toml`, 26 lần trong `.md`, và nhiều file `.js`/`.py`.
>
> **Quy tắc phân công:** Mỗi AI builder nhận **một sub-task ID** (ví dụ `DEBT-01-A`). Kiểm tra bảng bên dưới trước khi bắt đầu để tránh xung đột.

### Phân công Sub-tasks

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
| `DEBT-01-N` | **Thêm Telemetry/log warning cho đường dẫn legacy** | ✅ **Hoàn tất/Xóa** | Warning telemetry đã được thêm trước đây. Kể từ DEBT-01-P, toàn bộ code legacy path đã bị xóa khỏi `config.rs` nên warning không còn cần thiết. |
| `DEBT-01-O` | **Chốt ngày dừng hỗ trợ alias `openfang`** | ✅ **Hoàn tất** | Không có giai đoạn deprecated — đã xóa trực tiếp trong v0.1.0 để release sạch. |
| `DEBT-01-P` | **Xóa shim/bí danh legacy** | ✅ **Hoàn tất** | Đã xóa: `OPENFANG_HOME`/`~/.openfang` trong `config.rs`, crate `ochi-types-legacy`, `openfang_client.py`, `openfang_sdk.py`, shims trong `install.sh`/`install.ps1`, tác giả `OpenFang` trong `SKILL.md`. |
| `DEBT-01-Q` | **Hoàn thiện release note & migration guide** | ✅ **Hoàn tất** | `docs/sdk-migration.md` viết lại: chỉ tóm tắt thay đổi và hướng dẫn migrate thư mục. Không còn deprecation schedule vì đã xóa hoàn toàn. |

### Tiêu chí Hoàn thành DEBT-01

Nhiệm vụ DEBT-01 được coi là **HOÀN TẤT** khi:
- `grep -r "openfang\|OpenFang\|OPENFANG" --include="*.rs" . | wc -l` trả về `0`.
- `grep -r "openfang\|OpenFang\|OPENFANG" --include="*.toml" . | wc -l` trả về `0` (ngoại trừ các comment backward-compat có ghi chú rõ ràng).
- `cargo check --workspace` vẫn PASS sau tất cả thay đổi.
- `cargo test -p ochi-kernel -p ochi-types` vẫn PASS.

---

## 2. DEBT-02: Xử lý `unwrap()` và `expect()` An toàn

> **Mục tiêu:** Thay thế các lệnh gọi `.unwrap()` và `.expect()` không an toàn bằng các cấu trúc xử lý lỗi phù hợp, ngăn ngừa `panic` trong production.
>
> **Nguyên tắc thay thế:**
> - Trong **production code** (không phải test): dùng `?`, `if let`, `match`, hoặc `.unwrap_or_else(|e| { tracing::error!(...); default_value })`.
> - Trong **test code** (`#[cfg(test)]`): `.unwrap()` và `.expect()` được chấp nhận.
> - Trong **`main.rs` hoặc `bin` entry points**: `.expect("message rõ ràng")` được chấp nhận nếu là lỗi không thể phục hồi khi khởi động.
> - Ưu tiên xử lý các crate **kernel** và **runtime** trước vì chúng là critical path.
>
> **Quy tắc phân công:** Mỗi AI builder nhận **một sub-task ID**. Kiểm tra bảng trước khi bắt đầu.

### Phân công Sub-tasks

| Sub-task ID | Crate mục tiêu | Số lượng ước tính | Trạng thái | Ưu tiên |
| :--- | :--- | :--- | :--- | :--- |
| `DEBT-02-A` | **`crates/ochi-kernel/`** | ~209 `unwrap` + 29 `expect` | ✅ **Hoàn tất (production code)** | 🔴 Cao nhất — Sửa `kernel.rs`: dùng constant `DEFAULT_LISTEN_ADDR` + `.expect()` cho infallible parse thay vì `.unwrap_or_else(|_| "...".parse().unwrap())`; sửa `config.rs`: loại bỏ duplicate code trong `ochi_home()`. Các `.unwrap()` còn lại đều nằm trong `#[test]` blocks (chấp nhận được). |
| `DEBT-02-B` | **`crates/ochi-runtime/`** (phần `agent_loop.rs`, `tool_runner.rs`, `compactor.rs`) | ~100 `unwrap` (3 files nặng nhất) | ✅ **Hoàn tất** — Tất cả `.unwrap()` trong production code đều là infallible (after guard checks, LazyLock regex, iterator với length check) hoặc nằm trong `#[cfg(test)]` blocks. | 🔴 Cao nhất |
| `DEBT-02-C` | **`crates/ochi-runtime/`** (phần còn lại) | ~155 `unwrap` + 16 `expect` | ✅ **Hoàn tất** — Đã audit toàn bộ: `apply_patch.rs` (after `starts_with` guards), `browser.rs` (OnceLock get after set), `compactor.rs` (infallible after len check), `graceful_shutdown.rs` (mutex với `unwrap_or_else`), `reply_directives.rs` (after peek), `tool_runner.rs` (LazyLock regex). | 🟠 Cao |
| `DEBT-02-D` | **`crates/ochi-api/`** | ~196 `unwrap` + 16 `expect` | ✅ **Hoàn tất** — Đã sửa `routes.rs:9164`: thay `serde_json::to_value(resp).unwrap()` → `Json(resp)` trực tiếp. Các `unwrap()` còn lại đều infallible (literal string header parse, NonZeroU32 với literal, URL parse với literal). | 🟠 Cao |
| `DEBT-02-E` | **`crates/ochi-channels/`** | ~157 `unwrap` + 3 `expect` | ✅ **Hoàn tất** — Audit hoàn tất: 9 production unwrap xác định. Đã sửa 7: `discord.rs` (session_id guarded by is_some(), serde_json::Value infallible), `guilded.rs`/`webex.rs` (Bearer token header infallible), `mattermost.rs`/`slack.rs` (serde_json::Value infallible). 2 còn lại (`webhook.rs`, `dingtalk.rs`) đã có `.expect("HMAC accepts any key size")` sẵn. | 🟡 Trung bình |
| `DEBT-02-F` | **`crates/ochi-memory/`** | ~125 `unwrap` | ✅ **Hoàn tất** — Audit hoàn tất: 0 production unwrap. Toàn bộ `.unwrap()` đều nằm trong `#[cfg(test)]` blocks (chấp nhận được). | 🟡 Trung bình |
| `DEBT-02-G` | **`crates/ochi-migrate/`** | ~137 `unwrap` | ✅ **Hoàn tất** — Audit hoàn tất: 0 production unwrap. Toàn bộ `.unwrap()` đều nằm trong `#[cfg(test)]` blocks (chấp nhận được). | 🟡 Trung bình |
| `DEBT-02-H` | **`crates/ochi-types-legacy/`** | ~127 `unwrap` | ✅ **Hoàn tất** — 0 `.unwrap()` trong toàn crate. Đã clean. | 🟡 Trung bình |
| `DEBT-02-I` | **`crates/ochi-skills/`, `ochi-extensions/`, `ochi-hands/`** | ~84 + 76 + 41 `unwrap` | ✅ **Hoàn tất** — `ochi-skills`: `marketplace.rs` đã có `.expect("Failed to build HTTP client")` sẵn. `ochi-extensions`: sửa `vault.rs:437` `keyring_path.parent().unwrap()` → `.expect("keyring path always has a parent directory")`. `ochi-hands`: 0 production unwrap. | 🟢 Thấp |
| `DEBT-02-J` | **`crates/ochi-cli/`, `ochi-wire/`, `ochi-desktop/`, `xtask/`** | ~51 + 42 + 7 `expect` | ✅ **Hoàn tất** — `ochi-wire`: `peer.rs` default `parse().unwrap()` → `.expect("'127.0.0.1:0' is a valid socket address")`. `ochi-cli`: `print_help().unwrap()` → `.expect()`, `Runtime::new().unwrap()` → `.expect()`, `stdout().flush().unwrap()` → `.expect()`, `fs::write().unwrap()` → `unwrap_or_else` với `eprintln!` + `process::exit(1)`, `create_dir_all().unwrap()` → `unwrap_or_else`. | 🟢 Thấp |

### Quy trình Thực hiện cho mỗi Sub-task

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

### Tiêu chí Hoàn thành DEBT-02

Nhiệm vụ DEBT-02 được coi là **HOÀN TẤT** khi:
- `grep -rn "\.unwrap()" crates/ --include="*.rs" | grep -v "#\[cfg(test)\]\|//.*unwrap\|test_"` trả về số lượng giảm ít nhất **80%** so với ban đầu (~1,500 → dưới 300).
- `cargo clippy --workspace -- -W clippy::unwrap_used 2>&1 | grep "unwrap_used" | wc -l` giảm đáng kể.
- `cargo test --workspace` vẫn PASS.
- Không có `panic` nào được ghi nhận trong integration test.

---

> **Lưu ý cho AI Builder:** Sau khi hoàn thành bất kỳ sub-task nào, hãy cập nhật trạng thái trong bảng tương ứng từ `⏳ PENDING` thành `✅ Hoàn tất` và ghi rõ commit hash. Đây là bắt buộc theo **Mục 6.2** trong [PLAN-TASK.md](PLAN-TASK.md).
