# Ochi - Hệ điều hành Agent cho Ophiuchus

**Ochi** là một hệ điều hành tác tử (Agent Operating System) được phát triển cho hệ sinh thái **Ophiuchus** với triết lý Lạc Việt. Dự án này tập trung vào việc "Việt hóa" sâu sắc, tích hợp các công nghệ xử lý ngôn ngữ tự nhiên (NLP) tiếng Việt hàng đầu và triển khai một kiến trúc Agent độc đáo dựa trên các nguyên tắc triết học phương Đông.

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/lathaihoangmedia-lgtm/Ochi?style=for-the-badge)](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/lathaihoangmedia-lgtm/Ochi/rust.yml?branch=main&style=for-the-badge)](https://github.com/lathaihoangmedia-lgtm/Ochi/actions/workflows/rust.yml)
[![GitHub license](https://img.shields.io/github/license/lathaihoangmedia-lgtm/Ochi?style=for-the-badge)](https://github.com/lathaihoangmedia-lgtm/Ochi/blob/main/LICENSE-APACHE)

---

## 🚀 Cài đặt & Sử dụng

Ochi được thiết kế để dễ dàng cài đặt và sử dụng trên nhiều nền tảng.

### Cài đặt qua Script (Linux & macOS)

Cách nhanh nhất để cài đặt Ochi là sử dụng script cài đặt. Mở terminal và chạy lệnh sau:

```bash
curl -sSf https://ochi.sh | sh
```

Script này sẽ tự động phát hiện hệ điều hành và kiến trúc máy của bạn, tải về phiên bản phù hợp và cài đặt vào hệ thống.

### Cài đặt qua Cargo (Dành cho Lập trình viên)

Nếu bạn đã cài đặt Rust, bạn có thể cài đặt Ochi trực tiếp từ `crates.io`:

```bash
cargo install ochi
```

### Tải về trực tiếp từ GitHub Releases

Bạn có thể tải về các phiên bản đã được build sẵn cho từng hệ điều hành từ trang [GitHub Releases](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest).

| Hệ điều hành | Kiến trúc | Định dạng | Tải về |
| :--- | :--- | :--- | :--- |
| **Windows** | x86_64 | `.zip` (CLI) | [Link](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest/download/ochi-x86_64-pc-windows-msvc.zip) |
| | ARM64 | `.zip` (CLI) | [Link](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest/download/ochi-aarch64-pc-windows-msvc.zip) |
| **macOS** | Intel (x86_64) | `.tar.gz` (CLI) | [Link](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest/download/ochi-x86_64-apple-darwin.tar.gz) |
| | Apple Silicon (ARM64) | `.tar.gz` (CLI) | [Link](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest/download/ochi-aarch64-apple-darwin.tar.gz) |
| **Linux** | x86_64 | `.tar.gz` (CLI) | [Link](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest/download/ochi-x86_64-unknown-linux-gnu.tar.gz) |
| | ARM64 | `.tar.gz` (CLI) | [Link](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest/download/ochi-aarch64-unknown-linux-gnu.tar.gz) |

Sau khi tải về, giải nén file và bạn sẽ có file thực thi `ochi` (hoặc `ochi.exe` trên Windows).

### Sử dụng

Sau khi cài đặt, khởi động Ochi server:

```bash
ochi start
```

Sau đó, mở trình duyệt và truy cập [http://localhost:4200](http://localhost:4200) để bắt đầu sử dụng Web Dashboard.

---

## Tầm nhìn

Kiến tạo một hệ điều hành tác tử tự chủ, thông minh, có khả năng thấu hiểu và tương tác hiệu quả với ngữ cảnh văn hóa, ngôn ngữ Việt Nam, phục vụ cho việc phát triển và vận hành hệ thống AI Ophiuchus - một thực thể AI có khả năng tự nhận thức, học hỏi liên tục và hướng tới các mục tiêu triết học sâu sắc.

## Kiến trúc 9 Đại Tác Tử

Ochi được tổ chức theo một cấu trúc phân cấp Agent độc đáo:

| STT | Tên Đại Tác Tử | Hành (Ngũ Hành) | Vai trò & Nhiệm vụ Cốt lõi |
| :--- | :--- | :--- | :--- |
| 1 | **Ochi-KIM (Logic & Cấu trúc)** | **Kim** | Quản lý tính nhất quán, logic hệ thống và bảo mật. |
| 2 | **Ochi-MỘC (Sáng tạo & Sinh trưởng)** | **Mộc** | Chịu trách nhiệm về R&D, tạo ra các Skill mới. |
| 3 | **Ochi-THỦY (Trí nhớ & Luân chuyển)** | **Thủy** | Quản lý Long-term Memory, Vector Database (Lạc Việt DB). |
| 4 | **Ochi-HỎA (Xử lý & Thực thi)** | **Hỏa** | Động cơ tính toán chính, xử lý API NLP (Wit.ai, Zalo AI). |
| 5 | **Ochi-THỔ (Tích hợp & Nền tảng)** | **Thổ** | Quản lý hạ tầng (aaPanel MCP), sự ổn định của hệ điều hành. |
| 6 | **Ochi-THÁI CỰC (Điều phối Trung tâm)** | **Vô Cực** | Điểm tiếp nhận trực tiếp từ Manus, cân bằng Âm Dương. |
| 7 | **Ochi-CÀN (Thiên - Tầm nhìn)** | **Dương** | Quản lý các Agent Thiên Cương (36 vị), lập kế hoạch dài hạn. |
| 8 | **Ochi-KHÔN (Địa - Thực tế)** | **Âm** | Quản lý các Agent Địa Sát (72 vị), tương tác với thế giới thực. |
| 9 | **Ochi-NHÂN (Giao diện & Văn hóa)** | **Nhân** | Chuyên trách về "Việt hóa", đảm bảo phù hợp văn hóa, ngôn ngữ. |

## Bắt đầu

Để bắt đầu với Ochi, vui lòng tham khảo tài liệu chi tiết trong thư mục `docs/` và kế hoạch phát triển tại `PLAN-TASK.md`.

## Contribution

Repository đã để **public** để thuận tiện cộng tác qua GitHub (issues/PR/discussions). Trước khi đóng góp, đọc `CONTRIBUTING.md` và `PLAN-TASK.md` để nắm quy trình.

## Bản quyền

© 2026 LaBeeTechMedia. Tất cả quyền được bảo lưu.
