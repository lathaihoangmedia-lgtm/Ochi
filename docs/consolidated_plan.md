# Tổng hợp các Kế hoạch và Nhiệm vụ còn tồn đọng

Đây là tài liệu tổng hợp các kế hoạch, nhiệm vụ, và các mục cần thực hiện đã được phát hiện rải rác trong các file tài liệu khác nhau của dự án Ochi. Mục tiêu là để hợp nhất chúng vào file `PLAN-TASK.md` chính, tạo ra một cấu trúc quản lý công việc duy nhất và nhất quán.

## 1. Phân tích các nguồn kế hoạch

- **PLAN-TASK.md**: Đóng vai trò là kế hoạch tổng thể, đã bao gồm các luồng công việc chính (Hạ tầng, Tích hợp chức năng, Phát triển Agent) và các khoản nợ kỹ thuật lớn (DEBT-01: Đổi tên, DEBT-02: Xử lý `unwrap`).
- **docs/launch-roadmap.md**: Ghi lại tiến độ theo sprint, hầu hết đã hoàn thành. Mục còn lại là `2.4 Install script domain`, tương ứng với `INFRA-01` và `PRE-04` trong `PLAN-TASK.md`.
- **docs/production-checklist.md**: Chứa các mục kỹ thuật cụ thể để phát hành, nhiều mục đang ở trạng thái `BLOCKING`. Các mục này tương ứng với các task `PRE-*` trong `PLAN-TASK.md`.
- **docs/repo-audit-5w1h.md**: Đưa ra các khuyến nghị ưu tiên (P0, P1) và kế hoạch 30-60-90 ngày. Đây là một kế hoạch độc lập cần được tích hợp.
- **docs/ochi-full-rename-checklist.md**: Chi tiết hóa `DEBT-01`, nhưng có nhiều mục chưa được đánh dấu hoàn thành. Cần được đối chiếu và cập nhật vào `PLAN-TASK.md`.
- **docs/hyperledger-fabric-policy.md** & **docs/integrations/*.md**: Chứa các định hướng và mục tiêu tương lai, cần được đưa vào roadmap dài hạn.

## 2. Danh sách các nhiệm vụ lẻ cần tích hợp

Dưới đây là danh sách các nhiệm vụ cụ thể được rút ra từ các tài liệu trên, chưa được thể hiện rõ ràng hoặc cần được cập nhật trong `PLAN-TASK.md`.

### Từ `repo-audit-5w1h.md` (Ưu tiên cao)

- **[AUDIT-P0-01]** Chốt và kiểm chứng install endpoint (`ochi.sh`). (Trùng với `INFRA-01`)
- **[AUDIT-P0-02]** Rà soát release secrets + signing cho desktop updater. (Trùng với `PRE-01`, `PRE-02`, `PRE-03`)
- **[AUDIT-P1-01]** Dọn warning dễ sửa ở `ochi-cli`.
- **[AUDIT-P1-02]** Thiết lập quality gate tối thiểu cho PR/release (`cargo test --workspace` + lint).

### Từ `ochi-full-rename-checklist.md` (Chi tiết hóa DEBT-01)

- **[DEBT-01-L]** Chạy full test matrix sau mỗi batch rename.
- **[DEBT-01-M]** Cập nhật docs publish/migration cho người dùng SDK.
- **[DEBT-01-N]** Thêm Telemetry/log warning có hạn thời gian cho đường dẫn legacy.
- **[DEBT-01-O]** Chốt ngày dừng hỗ trợ alias `openfang`.
- **[DEBT-01-P]** Xóa shim/bí danh legacy sau 2–3 phiên bản ổn định.
- **[DEBT-01-Q]** Hoàn thiện tài liệu release note + migration guide chính thức.

### Từ các tài liệu khác (Định hướng tương lai)

- **[FUTURE-01]** Tích hợp Hyperledger Fabric (từ `hyperledger-fabric-policy.md`).
- **[FUTURE-02]** Hỗ trợ `POST /speech`, `POST /dictation`, `POST /synthesize` cho Wit.ai (từ `wit-ai-integration-notes.md`).
- **[CI-CD-01]** Mở rộng workflow `rust.yml` để chạy `cargo test --workspace` và `cargo clippy --workspace` (từ báo cáo đánh giá và `repo-audit`).

## 3. Kế hoạch hợp nhất

1.  **Cập nhật `PLAN-TASK.md`:**
    *   Tạo một mục mới `10. Nợ Kỹ thuật & Cải tiến Quy trình` để tích hợp các task `AUDIT-*` và `CI-CD-*`.
    *   Bổ sung các task `DEBT-01-L` đến `DEBT-01-Q` vào cuối mục `DEBT-01` để hoàn thiện checklist đổi tên.
    *   Tạo một mục mới `11. Roadmap Dài hạn` để ghi nhận các mục `FUTURE-*`.
2.  **Đánh dấu các file tài liệu cũ là `deprecated`**: Sau khi hợp nhất, thêm một ghi chú ở đầu các file `docs/repo-audit-5w1h.md`, `docs/ochi-brand-roadmap.md`, `docs/ochi-full-rename-checklist.md` để chỉ dẫn người đọc tham khảo `PLAN-TASK.md` là nguồn thông tin chính thức và duy nhất.

Sau khi hoàn thành các bước này, `PLAN-TASK.md` sẽ trở thành trung tâm điều phối cho toàn bộ công việc của dự án.
