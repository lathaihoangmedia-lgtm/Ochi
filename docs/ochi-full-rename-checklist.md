# Checklist rename “Ochi toàn bộ” (an toàn + tương thích ngược)

Mục tiêu: chuyển brand/kỹ thuật sang Ochi theo lộ trình giảm rủi ro, không làm gãy runtime hiện tại.

## Trạng thái thực thi trong commit này

- [x] **Thông báo CLI cho env path chuyển sang Ochi**: hiển thị `~/.ochi/.env` thay vì `~/.openfang/.env` cho các lệnh config key.
- [x] **Env var alias ưu tiên OCHI**: hỗ trợ `OCHI_HOME` trước, sau đó fallback `OPENFANG_HOME`.
- [x] **Home dir mặc định ưu tiên `.ochi`**: nếu `~/.ochi` tồn tại sẽ dùng trước, nếu không có thì dùng `~/.openfang` để tương thích ngược.
- [x] **Config path mặc định đi theo home resolver mới**.

## Phase 1 — Compatibility layer (không phá build)

- [x] Resolver home/config:
  - [x] `OCHI_HOME` (mới)
  - [x] `OPENFANG_HOME` (legacy)
  - [x] `~/.ochi` ưu tiên nếu đã tồn tại
  - [x] fallback `~/.openfang`
- [ ] Bổ sung alias CLI command (`ochi` -> `openfang`) ở mức binary/package.
- [ ] Chuẩn hóa thông báo UI: hiển thị Ochi, nhưng vẫn chấp nhận đường dẫn/biến môi trường legacy.

## Phase 2 — Binary & runtime rename

- [ ] Đổi binary chính từ `openfang` sang `ochi`.
- [ ] Giữ shim executable `openfang` (deprecation window) để gọi sang `ochi`.
- [ ] Cập nhật service/unit/script cài đặt để chạy `ochi`.
- [ ] Cập nhật Dockerfile:
  - [ ] build/copy `ochi`
  - [ ] `ENTRYPOINT ["ochi"]`
  - [ ] chuyển biến môi trường mẫu sang `OCHI_HOME` (vẫn đọc `OPENFANG_HOME`).

## Phase 3 — Crate/workspace rename (rủi ro cao, cần kế hoạch)

- [ ] Đổi tên crate `openfang-*` -> `ochi-*` theo thứ tự từ leaf crate đến top-level.
- [ ] Thêm transitional re-export/compat crate nếu cần cho dependency nội bộ.
- [ ] Cập nhật toàn bộ `Cargo.toml` workspace members, dependencies, docs build.
- [ ] Chạy full test matrix sau mỗi batch rename.

## Phase 4 — SDK & package ecosystem

- [ ] JS SDK: `@openfang/sdk` -> `@ochi/sdk`.
- [ ] Python package: `openfang` -> `ochi` (kèm package alias hoặc metapackage).
- [ ] Cập nhật docs publish/migration cho người dùng SDK.

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
