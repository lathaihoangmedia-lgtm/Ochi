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

## Phase 4 — SDK & package ecosystem

- [x] JS SDK: `@openfang/sdk` -> `@ochi/sdk` (giữ export alias tương thích ngược cho `OpenFang`).
- [x] Python package: `openfang` -> `ochi` (đã đổi package metadata + thêm module alias `ochi_*` và giữ tương thích `openfang_*`).
- [x] Cập nhật docs publish/migration cho người dùng SDK (`docs/sdk-migration.md`).

## Phase 5 — Data/config migration tooling

- [ ] Tool migrate cấu hình và dữ liệu:
  - [ ] `~/.openfang` -> `~/.ochi`
  - [ ] merge an toàn, có backup + rollback.
- [ ] Migration alias cho key/section cấu hình cũ (nếu đổi key).
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
