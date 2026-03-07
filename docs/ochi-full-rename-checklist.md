> **Lưu ý:** Nội dung trong tài liệu này đã được tổng hợp và chuyển vào file `PLAN-TASK.md` chính. Vui lòng tham khảo `PLAN-TASK.md` để có thông tin cập nhật và đầy đủ nhất.

# Checklist rename “Ochi toàn bộ” (an toàn + tương thích ngược) (đã lỗi thời)

Mục tiêu: chuyển brand/kỹ thuật sang Ochi theo lộ trình giảm rủi ro, không làm gãy runtime hiện tại.

## Trạng thái rà soát hiện tại

- [x] **Thông báo CLI cho config key đã chuyển sang Ochi**: các thông báo `config set-key/delete-key` đang hiển thị `~/.ochi/.env`.
- [x] **Resolver home/config ở core đã ưu tiên OCHI**: `OCHI_HOME` -> `OCHI_HOME` -> `~/.ochi` -> `~/.ochi`.
- [x] **Config path mặc định ở kernel đã đi theo home resolver mới** (`ochi_home().join("config.toml")`).
- [x] **CLI helper còn hardcode `.ochi` ở nhiều chỗ** (`ochi_home()` trong CLI đã dùng core resolver và chuẩn hóa hint/log path sang `~/.ochi`).

## Phase 1 — Compatibility layer (không phá build)

- [x] Resolver home/config ở tầng core:
  - [x] `OCHI_HOME` (mới)
  - [x] `OCHI_HOME` (legacy)
  - [x] `~/.ochi` ưu tiên nếu đã tồn tại
  - [x] fallback `~/.ochi`
- [x] Bổ sung alias CLI command (`ochi` song song với `ochi`) ở mức binary/package.
- [x] Chuẩn hóa thông báo UI/CLI toàn cục: hiển thị Ochi nhất quán, nhưng vẫn chấp nhận đường dẫn/biến môi trường legacy.
- [x] Dọn các hardcode đường dẫn `.ochi` còn sót trong CLI/TUI/help text.

## Phase 2 — Binary & runtime rename

- [x] Đổi binary **chính** từ `ochi` sang `ochi` (cutover runtime artifacts: Docker ENTRYPOINT và systemd ExecStart chuyển sang `ochi`).
- [x] Giữ shim executable `ochi` (deprecation window) để gọi sang `ochi` sau khi cutover.
- [x] Cập nhật service/unit/script cài đặt để chạy `ochi` (systemd + install script ưu tiên `ochi`, vẫn hỗ trợ `ochi`).
- [x] Cập nhật Dockerfile:
  - [x] build/copy `ochi`
  - [x] `ENTRYPOINT ["ochi"]`
  - [x] chuyển biến môi trường mẫu sang `OCHI_HOME` (vẫn đọc `OCHI_HOME`).

## Phase 3 — Crate/workspace rename (rủi ro cao, cần kế hoạch)

- [x] Đổi tên crate `openfang-*` -> `ochi-*` theo thứ tự từ leaf crate đến top-level. ✅ DONE 2026-03-03
- [x] Thêm transitional re-export/compat crate (ochi-types compat layer). ✅ DONE
- [x] Cập nhật toàn bộ `Cargo.toml` workspace members, dependencies, docs build. ✅ DONE
- [ ] Chạy full test matrix sau mỗi batch rename.

### Bảng triển khai “ready-to-execute” (leaf -> top-level)

| Batch | Phạm vi crate (rename trong batch) | Lý do chọn thứ tự | Definition of Done |
|---|---|---|---|
| B0 (chuẩn bị) | Không rename crate; chuẩn hóa baseline CI + lockfile | Giảm nhiễu trước khi rename diện rộng | Baseline `cargo check`/`cargo test` pass; snapshot warning hiện tại; thống nhất nhánh triển khai |
| B1 (leaf models) | `ochi-types` -> `ochi-types`, `openfang-wire` -> `ochi-wire`, `openfang-memory` -> `ochi-memory` | Đây là nhóm ít phụ thuộc vào crate khác, phù hợp mở đầu | Build pass cho 3 crate + toàn workspace; docs/reference cập nhật tên crate mới |
| B2 (leaf extensions) | `ochi-channels` -> `ochi-channels`, `ochi-skills` -> `ochi-skills`, `ochi-extensions` -> `ochi-extensions` | Chủ yếu tiêu thụ types/runtime API, dễ cô lập theo module | Unit test nhóm extension pass; command/examples dùng crate name mới |
| B3 (core runtime) | `ochi-runtime` -> `ochi-runtime`, `ochi-migrate` -> `ochi-migrate` | Đã có leaf ổn định, bắt đầu rename phần core nhưng chưa chạm entrypoint lớn | Regression test runtime + migration pass; không mất tương thích dữ liệu cũ |
| B4 (service/kernel) | `ochi-kernel` -> `ochi-kernel`, `ochi-api` -> `ochi-api` | Đây là tầng orchestrator/API, rename sau khi dependency graph đã sạch | API integration tests pass; route/docs nội bộ cập nhật nhất quán |
| B5 (entrypoint/apps) | `ochi-cli` -> `ochi-cli`, `ochi-desktop` -> `ochi-desktop`, `ochi-hands` -> `ochi-hands` | Tầng người dùng cuối; rename cuối để tránh ripple effect sớm | Smoke test binary `ochi` pass; shim tương thích `ochi` vẫn hoạt động trong deprecation window |
| B6 (workspace cutover) | Cập nhật `Cargo.toml` workspace members/dependencies toàn cục; dọn alias tạm | Chốt cutover khi tất cả batch trước đã xanh | Không còn dependency path nội bộ `ochi-*` (trừ alias chủ đích); full matrix pass |

### Trạng thái thực thi thực tế (theo bảng)

- [x] **B0 đã khởi động**: chạy baseline và ghi nhận rào cản môi trường cho workspace đầy đủ.
- [x] **B1 đã bắt đầu (bước tương thích)**: thêm crate chuyển tiếp `ochi-types` re-export từ `ochi-types`.
- [x] B1 đã đổi thư mục/package gốc `openfang-types` sang `ochi-types` và chuyển `openfang-types` thành compat re-export crate.
- [x] **B2 đã bắt đầu (leaf extensions)**: migrate dependency `ochi-channels` và `ochi-skills` sang `ochi-types` qua alias.

**Kết quả baseline B0 (batch log ngắn):**
- `cargo metadata --no-deps` ✅
- `cargo check --workspace` ⚠️ dừng ở `glib-sys` do thiếu system lib `glib-2.0` trong môi trường hiện tại.
- `cargo check -p ochi-kernel` ⚠️ chưa có kết quả kết thúc ổn định trong phiên chạy (build kéo dài, cần rerun trong CI có timeout/log đầy đủ).

> Ghi chú triển khai: để tiếp tục đúng nhịp bảng B1, nên tách lane CI không bao gồm desktop GUI (hoặc provision `glib-2.0` đầy đủ) nhằm giữ vòng phản hồi nhanh cho rename crate thuần Rust.

**Tiến độ B1 (incremental cutover):**
- Đã cutover crate gốc sang `crates/ochi-types` (package `ochi-types`) và giữ `crates/ochi-types-legacy` làm compat package `openfang-types` re-export từ `ochi_types::*`.
- Đã giữ cả hai crate trong `workspace.members`/`workspace.default-members` để đảm bảo migration không phá build.
- Kiểm tra nhanh: `cargo check -p ochi-types` và `cargo check -p openfang-types` pass.
- Đã migrate dependency của `openfang-wire` từ `ochi-types` sang `ochi-types` (bước 1 trong leaf-models).
- Kiểm tra nhanh: `cargo check -p ochi-wire` pass sau migration dependency.
- Đã migrate dependency của `openfang-memory` sang compat crate `ochi-types` (giữ import path `openfang_types` qua dependency alias).
- Kiểm tra nhanh: `cargo check -p ochi-memory` pass sau migration dependency.
- Đã migrate dependency của `ochi-channels` sang compat crate `ochi-types` (dependency alias).
- Kiểm tra nhanh: `cargo check -p ochi-channels` pass sau migration dependency.
- Đã migrate dependency của `ochi-skills` sang compat crate `ochi-types` (dependency alias).
- Kiểm tra nhanh: `cargo check -p ochi-skills` pass sau migration dependency.

### Checklist kỹ thuật cho mỗi batch

- [ ] Tạo branch batch riêng (ví dụ: `rename/phase3-b1-leaf-models`).
- [ ] Rename package name trong `Cargo.toml` + thư mục crate + cập nhật đường dẫn `path = "../..."`.
- [ ] Cập nhật toàn bộ `use`/import/reference crate name bằng thay thế có kiểm soát (không thay mù toàn repo).
- [ ] Nếu batch có rủi ro public API: thêm compat layer tạm (`pub use`) hoặc crate alias để giảm gãy build liên batch.
- [ ] Chạy test matrix của batch (mục bên dưới) và lưu log ngắn trong PR.
- [ ] Sau khi merge batch: rebase batch kế tiếp + chạy lại smoke test toàn workspace.

### Test matrix bắt buộc theo batch

1. **Build nhanh toàn workspace**
   - `cargo check --workspace`
2. **Unit/integration cơ bản toàn workspace**
   - `cargo test --workspace --all-targets`
3. **Kiểm tra package graph**
   - `cargo metadata --no-deps`
4. **Smoke test CLI sau batch có liên quan entrypoint**
   - `cargo run -p ochi-cli -- --help`
   - `cargo run -p ochi-cli -- doctor` (hoặc lệnh health-check tương đương đang có)
5. **Tương thích ngược trong deprecation window**
   - `cargo run -p ochi-cli --bin ochi -- --help` (nếu shim còn tồn tại)

### Tiêu chí dừng khẩn (rollback trigger)

- Tăng số lỗi compile liên batch mà không xác định được root cause trong 1 phiên làm việc.
- Mất tương thích migration config/home resolver (`OCHI_HOME`/`OCHI_HOME`).
- Smoke test entrypoint thất bại trên crate đã rename ở batch hiện tại.
- Phát sinh thay đổi public API không có migration note đi kèm.

## Phase 4 — SDK & package ecosystem

- [x] JS SDK: `@ochi/sdk` -> `@ochi/sdk` (giữ export alias tương thích ngược cho `OpenFang`).
- [x] Python package: `ochi` -> `ochi` (đã đổi package metadata + thêm module alias `ochi_*` và giữ tương thích `openfang_*`).
- [x] Cập nhật docs publish/migration cho người dùng SDK (`docs/sdk-migration.md`, examples Python dùng import Ochi).
- [ ] Cập nhật docs publish/migration cho người dùng SDK.

## Phase 5 — Data/config migration tooling

- [x] Tool migrate cấu hình và dữ liệu:
  - [x] `~/.ochi` -> `~/.ochi` (script: `scripts/migrate-home.sh`)
  - [x] merge an toàn, có backup + rollback.
- [x] Migration alias cho key/section cấu hình cũ (nếu đổi key) — giữ `OCHI_HOME` + `OCHI_HOME` fallback ở core resolver.
- [ ] Telemetry/log warning có hạn thời gian cho đường dẫn legacy.

## Phase 6 — Cutover & deprecation

- [ ] Chốt ngày dừng hỗ trợ alias `ochi`.
- [ ] Xóa shim/bí danh legacy sau 2–3 phiên bản ổn định.
- [ ] Tài liệu release note + migration guide chính thức.

## Lệnh kiểm tra tối thiểu mỗi phase

- `cargo check`
- `cargo test -p ochi-kernel`
- `cargo test -p ochi-types`
- smoke test CLI: init/load config từ `OCHI_HOME` và fallback `.ochi`

---
## Phase 6 — Wiring 9 Grand Agents Logic (DEFERRED)
> **Note:** Tạm hoãn đến thời điểm thích hợp sau khi rename hoàn tất.
- [ ] Wiring logic điều phối 9 Đại Tác Tử vào `ochi-kernel/src/orchestration.rs`
- [ ] Hoàn thiện cấu hình NLP tiếng Việt mặc định
- [ ] Kiểm tra build toàn workspace sau khi wiring
- [ ] Tích hợp với Lạc Việt DB
