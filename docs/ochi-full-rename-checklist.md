# Checklist rename “Ochi toàn bộ” (an toàn + tương thích ngược)

Mục tiêu: chuyển brand/kỹ thuật sang Ochi theo lộ trình giảm rủi ro, không làm gãy runtime hiện tại.

## Trạng thái rà soát hiện tại

- [x] **Thông báo CLI cho config key đã chuyển sang Ochi**: các thông báo `config set-key/delete-key` đang hiển thị `~/.ochi/.env`.
- [x] **Resolver home/config ở core đã ưu tiên OCHI**: `OCHI_HOME` -> `OPENFANG_HOME` -> `~/.ochi` -> `~/.openfang`.
- [x] **Config path mặc định ở kernel đã đi theo home resolver mới** (`openfang_home().join("config.toml")`).
- [x] **CLI helper còn hardcode `.openfang` ở nhiều chỗ** (`openfang_home()` trong CLI đã dùng core resolver và chuẩn hóa hint/log path sang `~/.ochi`).

## Phase 1 — Compatibility layer (không phá build)

- [x] Resolver home/config ở tầng core:
  - [x] `OCHI_HOME` (mới)
  - [x] `OPENFANG_HOME` (legacy)
  - [x] `~/.ochi` ưu tiên nếu đã tồn tại
  - [x] fallback `~/.openfang`
- [x] Bổ sung alias CLI command (`ochi` song song với `openfang`) ở mức binary/package.
- [x] Chuẩn hóa thông báo UI/CLI toàn cục: hiển thị Ochi nhất quán, nhưng vẫn chấp nhận đường dẫn/biến môi trường legacy.
- [x] Dọn các hardcode đường dẫn `.openfang` còn sót trong CLI/TUI/help text.

## Phase 2 — Binary & runtime rename

- [x] Đổi binary **chính** từ `openfang` sang `ochi` (cutover runtime artifacts: Docker ENTRYPOINT và systemd ExecStart chuyển sang `ochi`).
- [x] Giữ shim executable `openfang` (deprecation window) để gọi sang `ochi` sau khi cutover.
- [x] Cập nhật service/unit/script cài đặt để chạy `ochi` (systemd + install script ưu tiên `ochi`, vẫn hỗ trợ `openfang`).
- [x] Cập nhật Dockerfile:
  - [x] build/copy `ochi`
  - [x] `ENTRYPOINT ["ochi"]`
  - [x] chuyển biến môi trường mẫu sang `OCHI_HOME` (vẫn đọc `OPENFANG_HOME`).

## Phase 3 — Crate/workspace rename (rủi ro cao, cần kế hoạch)

- [ ] Đổi tên crate `openfang-*` -> `ochi-*` theo thứ tự từ leaf crate đến top-level.
- [ ] Thêm transitional re-export/compat crate nếu cần cho dependency nội bộ.
- [ ] Cập nhật toàn bộ `Cargo.toml` workspace members, dependencies, docs build.
- [ ] Chạy full test matrix sau mỗi batch rename.

### Bảng triển khai “ready-to-execute” (leaf -> top-level)

| Batch | Phạm vi crate (rename trong batch) | Lý do chọn thứ tự | Definition of Done |
|---|---|---|---|
| B0 (chuẩn bị) | Không rename crate; chuẩn hóa baseline CI + lockfile | Giảm nhiễu trước khi rename diện rộng | Baseline `cargo check`/`cargo test` pass; snapshot warning hiện tại; thống nhất nhánh triển khai |
| B1 (leaf models) | `openfang-types` -> `ochi-types`, `openfang-wire` -> `ochi-wire`, `openfang-memory` -> `ochi-memory` | Đây là nhóm ít phụ thuộc vào crate khác, phù hợp mở đầu | Build pass cho 3 crate + toàn workspace; docs/reference cập nhật tên crate mới |
| B2 (leaf extensions) | `openfang-channels` -> `ochi-channels`, `openfang-skills` -> `ochi-skills`, `openfang-extensions` -> `ochi-extensions` | Chủ yếu tiêu thụ types/runtime API, dễ cô lập theo module | Unit test nhóm extension pass; command/examples dùng crate name mới |
| B3 (core runtime) | `openfang-runtime` -> `ochi-runtime`, `openfang-migrate` -> `ochi-migrate` | Đã có leaf ổn định, bắt đầu rename phần core nhưng chưa chạm entrypoint lớn | Regression test runtime + migration pass; không mất tương thích dữ liệu cũ |
| B4 (service/kernel) | `openfang-kernel` -> `ochi-kernel`, `openfang-api` -> `ochi-api` | Đây là tầng orchestrator/API, rename sau khi dependency graph đã sạch | API integration tests pass; route/docs nội bộ cập nhật nhất quán |
| B5 (entrypoint/apps) | `openfang-cli` -> `ochi-cli`, `openfang-desktop` -> `ochi-desktop`, `openfang-hands` -> `ochi-hands` | Tầng người dùng cuối; rename cuối để tránh ripple effect sớm | Smoke test binary `ochi` pass; shim tương thích `openfang` vẫn hoạt động trong deprecation window |
| B6 (workspace cutover) | Cập nhật `Cargo.toml` workspace members/dependencies toàn cục; dọn alias tạm | Chốt cutover khi tất cả batch trước đã xanh | Không còn dependency path nội bộ `openfang-*` (trừ alias chủ đích); full matrix pass |

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
   - `cargo run -p ochi-cli --bin openfang -- --help` (nếu shim còn tồn tại)

### Tiêu chí dừng khẩn (rollback trigger)

- Tăng số lỗi compile liên batch mà không xác định được root cause trong 1 phiên làm việc.
- Mất tương thích migration config/home resolver (`OCHI_HOME`/`OPENFANG_HOME`).
- Smoke test entrypoint thất bại trên crate đã rename ở batch hiện tại.
- Phát sinh thay đổi public API không có migration note đi kèm.

## Phase 4 — SDK & package ecosystem

- [x] JS SDK: `@openfang/sdk` -> `@ochi/sdk` (giữ export alias tương thích ngược cho `OpenFang`).
- [x] Python package: `openfang` -> `ochi` (đã đổi package metadata + thêm module alias `ochi_*` và giữ tương thích `openfang_*`).
- [x] Cập nhật docs publish/migration cho người dùng SDK (`docs/sdk-migration.md`, examples Python dùng import Ochi).
- [ ] Cập nhật docs publish/migration cho người dùng SDK.

## Phase 5 — Data/config migration tooling

- [x] Tool migrate cấu hình và dữ liệu:
  - [x] `~/.openfang` -> `~/.ochi` (script: `scripts/migrate-home.sh`)
  - [x] merge an toàn, có backup + rollback.
- [x] Migration alias cho key/section cấu hình cũ (nếu đổi key) — giữ `OCHI_HOME` + `OPENFANG_HOME` fallback ở core resolver.
- [ ] Telemetry/log warning có hạn thời gian cho đường dẫn legacy.

## Phase 6 — Cutover & deprecation

- [ ] Chốt ngày dừng hỗ trợ alias `openfang`.
- [ ] Xóa shim/bí danh legacy sau 2–3 phiên bản ổn định.
- [ ] Tài liệu release note + migration guide chính thức.

## Lệnh kiểm tra tối thiểu mỗi phase

- `cargo check`
- `cargo test -p openfang-kernel`
- `cargo test -p openfang-types`
- smoke test CLI: init/load config từ `OCHI_HOME` và fallback `.openfang`
