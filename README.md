# Ochi: Hệ Điều Hành AI Tự Chủ — Xây Dựng Tương Lai AI Với Bản Sắc Việt

**Ochi là một hệ điều hành tác tử (Agent OS) mã nguồn mở, được xây dựng trên nền tảng Rust, mang trong mình triết lý Lạc Việt và kiến trúc Ngũ Hành độc đáo. Đây không chỉ là một phần mềm, mà là một nền tảng để kiến tạo các hệ thống AI tự chủ, thông minh, có khả năng thấu hiểu và tương tác sâu sắc với văn hóa và ngôn ngữ Việt Nam.**

(Ảnh bìa: Một hình ảnh nghệ thuật, trừu tượng thể hiện sự kết hợp giữa công nghệ AI hiện đại và các biểu tượng văn hóa Lạc Việt, như trống đồng Đông Sơn, chim Lạc bay trên một bo mạch phát sáng.)

[![GitHub release (latest by date)](https://img.shields.io/github/v/release/lathaihoangmedia-lgtm/Ochi?style=for-the-badge&label=Phi%C3%AAn%20b%E1%BA%A3n)](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest) [![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/lathaihoangmedia-lgtm/Ochi/release.yml?branch=main&style=for-the-badge&label=Build)](https://github.com/lathaihoangmedia-lgtm/Ochi/actions/workflows/release.yml) [![GitHub license](https://img.shields.io/github/license/lathaihoangmedia-lgtm/Ochi?style=for-the-badge&label=B%E1%BA%A3n%20quy%E1%BB%81n)](https://github.com/lathaihoangmedia-lgtm/Ochi/blob/main/README.md#%EF%B8%8F-b%E1%BA%A3n-quy%E1%BB%81n--lu%E1%BA%ADt-ai-vi%E1%BB%87t-nam)

---

## 💡 Tại Sao Chọn Ochi?

Thế giới AI đang phát triển vũ bão, nhưng hầu hết các mô hình và nền tảng đều được xây dựng dựa trên tư duy và dữ liệu phương Tây. Ochi ra đời để giải quyết bài toán này, mang đến một "hệ điều hành cho AI" được thiết kế từ gốc rễ để:

-   **Thấu hiểu sâu sắc tiếng Việt:** Tích hợp các mô hình NLP tiếng Việt hàng đầu (Wit.ai, Zalo AI) và kiến trúc được tối ưu cho ngữ cảnh Việt.
-   **Tư duy theo triết lý phương Đông:** Kiến trúc 9 Đại Tác Tử dựa trên Ngũ Hành và Kinh Dịch, giúp AI ra quyết định một cách cân bằng và toàn diện.
-   **Tự chủ và có thể mở rộng:** Khả năng tự học hỏi, tự tạo ra các "kỹ năng" (Skill) mới và quản lý một hệ sinh thái gồm 108 agents con (36 Thiên Cương, 72 Địa Sát).
-   **An toàn và hiệu suất cao:** Được viết bằng Rust, đảm bảo an toàn bộ nhớ, hiệu suất vượt trội và khả năng chạy trên mọi nền tảng.

(Video: Một video demo ngắn (khoảng 1-2 phút) giới thiệu giao diện Web Dashboard của Ochi, cho thấy cách người dùng nhập một yêu cầu phức tạp bằng tiếng Việt, và hệ thống tự động điều phối cho các Agent khác nhau thực thi.)

## ✨ Tính Năng Nổi Bật

-   **Web Dashboard Toàn Diện:** Giao diện web hiện đại để quản lý agents, theo dõi tác vụ, quản lý bộ nhớ và cấu hình hệ thống.
-   **Kiến Trúc 9 Đại Tác Tử:** Một hệ thống phân cấp độc đáo giúp điều phối và xử lý mọi loại nhiệm vụ, từ logic, sáng tạo, đến tương tác với thế giới thực.
-   **Điều Phối Thông Minh:** Bộ điều phối trung tâm (Thái Cực) tự động phân tích yêu cầu, đánh giá mức độ phức tạp và giao nhiệm vụ cho Agent phù hợp nhất.
-   **Hỗ Trợ Đa Nền Tảng:** Cài đặt và chạy Ochi dễ dàng trên Windows, macOS, và Linux.
-   **Tích Hợp Sẵn Sàng:** Kết nối với các dịch vụ bên ngoài như aaPanel, Lạc Việt DB (vector store), và các nền tảng AI khác.

(Hình ảnh: Một sơ đồ kiến trúc đẹp mắt, trực quan hóa mối quan hệ giữa 9 Đại Tác Tử, Manus, người dùng và các hệ thống bên ngoài.)

## 🚀 Bắt Đầu Sau 60 Giây

Cài đặt Ochi và khởi chạy Web Dashboard chỉ với một vài lệnh đơn giản.

### 1. Cài đặt

**Linux & macOS (Khuyên dùng):**

```bash
curl -sSf https://ochi.sh | sh
```

**Windows (PowerShell):**

```powershell
irm https://ochi.sh/install.ps1 | iex
```

**Các phương pháp khác (Docker, Cargo, Tải trực tiếp):**

<details>
<summary>Nhấn vào đây để xem các cách cài đặt khác</summary>

**Docker:**

```bash
# Tải image đa kiến trúc (amd64 + arm64)
docker pull ghcr.io/lathaihoangmedia-lgtm/ochi:latest

# Chạy container và mở port 4200
docker run -p 4200:4200 -v ~/.ochi:/root/.ochi ghcr.io/lathaihoangmedia-lgtm/ochi:latest
```

**Cargo (dành cho lập trình viên Rust):**

```bash
cargo install ochi
```

**Tải về trực tiếp từ GitHub Releases:**

Truy cập trang [**Releases**](https://github.com/lathaihoangmedia-lgtm/Ochi/releases/latest) để tải file `.zip` (Windows) hoặc `.tar.gz` (macOS/Linux) phù hợp với máy của bạn.

</details>

### 2. Khởi động

Sau khi cài đặt, mở một terminal mới và chạy:

```bash
ochi start
```

Ochi sẽ khởi động server và Web Dashboard sẽ có sẵn tại [**http://localhost:4200**](http://localhost:4200).

## 🗺️ Lộ Trình Phát Triển

Ochi đang trong giai đoạn phát triển tích cực. Đây là những gì chúng tôi đang tập trung:

-   [ ] **Tích hợp NLP Tiếng Việt:** Hoàn thiện kết nối với Zalo AI và các mô hình nội địa khác.
-   [ ] **Kết nối Lạc Việt DB:** Tích hợp vector store để tăng cường khả năng ghi nhớ và học hỏi.
-   [ ] **Triển khai 108 Agents:** Xây dựng và huấn luyện 36 Thiên Cương và 72 Địa Sát đầu tiên.
-   [ ] **Phát hành bản Desktop:** Hoàn thiện ứng dụng desktop cho Windows và macOS.

Xem kế hoạch chi tiết và theo dõi tiến độ tại [**PLAN-TASK.md**](https://github.com/lathaihoangmedia-lgtm/Ochi/blob/main/PLAN-TASK.md).

## 🤝 Tham Gia Đóng Góp

Ochi là một dự án mở và chúng tôi chào đón mọi sự đóng góp từ cộng đồng. Dù bạn là lập trình viên, nhà nghiên cứu AI, hay chỉ đơn giản là người đam mê công nghệ, bạn đều có thể giúp Ochi phát triển.

1.  **Đọc [**PLAN-TASK.md**](https://github.com/lathaihoangmedia-lgtm/Ochi/blob/main/PLAN-TASK.md):** Để hiểu rõ các nhiệm vụ đang cần thực hiện.
2.  **Mở một Issue:** Để báo lỗi, đề xuất tính năng, hoặc đặt câu hỏi.
3.  **Tạo một Pull Request:** Nếu bạn muốn trực tiếp đóng góp mã nguồn hoặc tài liệu.

## ⚖️ Bản Quyền & Luật AI Việt Nam

Bản quyền © 2026 LaBeeTechMedia. Mọi quyền được bảo lưu.

Dự án Ochi được phát triển và hoạt động tuân thủ theo **Luật Trí tuệ nhân tạo của Việt Nam (Luật số 134/2025/QH15)**, có hiệu lực từ ngày 01/03/2026. Chúng tôi cam kết tuân thủ các nguyên tắc cốt lõi của luật:

-   **Lấy con người làm trung tâm:** Mọi hoạt động của Ochi đều đặt quyền và lợi ích hợp pháp của con người lên hàng đầu.
-   **Minh bạch & Giải trình:** Ochi được thiết kế để người dùng luôn nhận biết được khi nào họ đang tương tác với AI. Các quyết định của hệ thống đều có khả năng được giải trình.
-   **An toàn & An ninh:** Chúng tôi áp dụng các biện pháp kỹ thuật và quy trình nghiêm ngặt để đảm bảo an toàn dữ liệu, an ninh hệ thống và bảo vệ người dùng khỏi các rủi ro tiềm ẩn.
-   **Phân loại rủi ro:** Ochi được phân loại là **hệ thống AI có rủi ro cao** theo Điều 9 của Luật, do đó phải tuân thủ các nghĩa vụ quản lý nghiêm ngặt nhất, bao gồm tự đánh giá, đăng ký và kiểm định định kỳ.

Chúng tôi tin rằng việc tuân thủ pháp luật không chỉ là nghĩa vụ, mà còn là nền tảng để xây dựng một hệ sinh thái AI bền vững và đáng tin cậy tại Việt Nam.

---

*Từ khóa: AI Agent, Agent OS, Trí tuệ nhân tạo, Ophiuchus, Lạc Việt, Ngũ Hành, Rust, AI Việt Nam, Xử lý ngôn ngữ tự nhiên, NLP, AI tự chủ, Open Source.*
