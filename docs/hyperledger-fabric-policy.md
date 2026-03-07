# Chính sách blockchain của Ochi / Ochi Blockchain Policy

> **English summary (for security reviewers):** Ochi does **not** implement any
> cryptocurrency, coin, token, or public blockchain.  The cryptographic
> components in the source code are **standard security infrastructure**
> (HMAC-SHA256 peer authentication, AES-256-GCM secret vault, Ed25519 manifest
> signing, SHA-256 audit chain).  The "Merkle hash chain" audit trail is a
> tamper-evident security log — similar in structure to a blockchain but
> completely unrelated to any cryptocurrency or financial instrument.
> Future blockchain integration will use **Hyperledger Fabric** (an enterprise,
> permissioned, non-cryptocurrency platform), not any public coin/token chain.

---

## Trạng thái hiện tại

- Ochi **chưa triển khai blockchain production**.
- Các thành phần mật mã hiện có trong mã nguồn là **hạ tầng bảo mật tiêu chuẩn**, không liên quan đến tiền mã hóa:
  - HMAC-SHA256 — xác thực peer-to-peer
  - AES-256-GCM — mã hóa vault lưu trữ bí mật
  - Ed25519 — ký số manifest agent
  - SHA-256 hash chain — nhật ký kiểm toán chống giả mạo (`audit.rs`)
- **Không có tiền mã hóa, coin, token, hay ví điện tử nào** trong codebase.

## Quyết định kiến trúc

- Ochi **chỉ chấp thuận hướng blockchain Hyperledger Fabric**.
- Không đưa thêm chain public khác vào roadmap chính thức.
- Việc tích hợp Fabric sẽ được triển khai ở pha sau khi có thiết kế chi tiết và tài liệu onboarding phù hợp cho đội không chuyên blockchain.

## Giải thích "Merkle hash chain" (chuỗi băm Merkle)

"Merkle hash chain" (hay "audit chain") trong `ochi-runtime/src/audit.rs` là **nhật ký bảo mật chống giả mạo**, không phải blockchain tiền mã hóa.

- Mỗi sự kiện bảo mật (spawn agent, truy cập file, kết nối mạng…) được ghi lại với hash SHA-256 liên kết với entry trước đó.
- Kỹ thuật này giống cấu trúc blockchain nhưng **chỉ dùng để phát hiện nếu log bị sửa đổi trái phép**.
- Không có giao dịch tài chính, không có mining, không có token, không kết nối mạng ngoài.

## Nguyên tắc triển khai giai đoạn tới

1. Tách lớp `blockchain adapter` độc lập với domain logic.
2. Chỉ expose capability ở mức nghiệp vụ (audit trail, provenance, notarization), không buộc đội vận hành thao tác crypto low-level.
3. Viết tài liệu theo hướng "cấu hình trước, kiến thức blockchain sau" để giảm rào cản.

## Việc chưa làm trong commit này

- Chưa thêm SDK Hyperledger Fabric ở runtime.

