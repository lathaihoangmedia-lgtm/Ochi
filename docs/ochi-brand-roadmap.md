# Ochi Brand nhất quán — Kế hoạch triển khai kỹ thuật

Tài liệu này chốt lại quyết định **dùng Ochi làm brand nhất quán** cho toàn bộ hoạt động phát triển trong repository này, đồng thời tách rõ các phần legacy chưa đổi tên kỹ thuật.

## 1) Nguyên tắc phạm vi

- Chỉ làm việc trong repository `Ochi`.
- Giữ nguyên cấu trúc mã tham chiếu từ các repo gốc; ưu tiên **cải tạo, nâng cấp, Việt hóa**.
- Không thực hiện rename kỹ thuật ồ ạt nếu chưa có kế hoạch migration/compatibility đầy đủ.

## 2) Repo tham chiếu chính thức

- Ophiuchus agent reference: `https://github.com/lathaihoangmedia-lgtm/Ophiuchus--agent`

## 3) Mục tiêu kỹ thuật ngắn hạn (Sprint hiện tại)

1. Chuẩn hóa ngôn ngữ tài liệu theo thương hiệu Ochi.
2. Giữ tương thích với cấu trúc workspace/crate hiện có để tránh phá vỡ build.
3. Cố định ví dụ cấu hình theo file `ochi.toml.example`.
4. Duy trì kiến trúc điều phối 9 Đại Tác Tử như một lớp orchestration ở mức tài liệu + cấu hình, sau đó mới tiến tới wiring vào runtime.

## 4) Mục tiêu kỹ thuật trung hạn

- Thiết kế lớp điều phối Ochi-THÁI CỰC làm entrypoint cho luồng nhiệm vụ từ Manus Planner.
- Chuẩn hóa taxonomy cho 108 agent chức năng:
  - 36 Thiên Cương: chiến lược / lập kế hoạch / điều phối.
  - 72 Địa Sát: thực thi / tích hợp công cụ / vận hành.
- Ánh xạ capability theo nhóm để tái sử dụng cơ chế policy, RBAC, audit log từ nền hiện tại.

## 5) Tiêu chí “brand nhất quán”

- Tài liệu public-facing ưu tiên tên **Ochi**.
- Tránh để tên legacy xuất hiện trong tài liệu public-facing.
- Với mã nguồn lõi, thực hiện đổi tên theo lộ trình migration, không đổi ồ ạt trong một lần để giảm rủi ro kỹ thuật.

## 6) Định hướng tích hợp NLP nội địa

Chuỗi provider mặc định theo định hướng Ochi:

`Wit.ai -> Zalo AI -> Manus Provider -> OpenAI`

Mục tiêu:
- Ưu tiên xử lý tiếng Việt trước.
- Có fallback quốc tế khi provider nội địa không khả dụng.
- Giữ khả năng mở rộng theo profile từng tác vụ.
