# Chính sách blockchain của Ochi

## Trạng thái hiện tại

- Ochi **chưa triển khai blockchain production**.
- Các thành phần mật mã hiện có trong mã nguồn là phần kỹ thuật nội bộ/legacy để bảo toàn tương thích hệ thống.

## Quyết định kiến trúc

- Ochi **chỉ chấp thuận hướng blockchain Hyperledger Fabric**.
- Không đưa thêm chain public khác vào roadmap chính thức.
- Việc tích hợp Fabric sẽ được triển khai ở pha sau khi có thiết kế chi tiết và tài liệu onboarding phù hợp cho đội không chuyên blockchain.

## Nguyên tắc triển khai giai đoạn tới

1. Tách lớp `blockchain adapter` độc lập với domain logic.
2. Chỉ expose capability ở mức nghiệp vụ (audit trail, provenance, notarization), không buộc đội vận hành thao tác crypto low-level.
3. Viết tài liệu theo hướng "cấu hình trước, kiến thức blockchain sau" để giảm rào cản.

## Việc chưa làm trong commit này

- Chưa gỡ hoàn toàn các thư viện mật mã đang phục vụ chức năng bảo mật hiện hữu.
- Chưa thêm SDK Hyperledger Fabric ở runtime.

