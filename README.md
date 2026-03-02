# Ochi - Hệ điều hành Agent cho Ophiuchus

**Ochi** là một hệ điều hành tác tử (Agent Operating System) được phát triển dựa trên nền tảng **OpenFang**, nhưng được tùy chỉnh và tối ưu hóa đặc biệt cho hệ sinh thái **Ophiuchus** với triết lý Lạc Việt. Dự án này tập trung vào việc "Việt hóa" sâu sắc, tích hợp các công nghệ xử lý ngôn ngữ tự nhiên (NLP) tiếng Việt hàng đầu và triển khai một kiến trúc Agent độc đáo dựa trên các nguyên tắc triết học phương Đông.

## Tầm nhìn

Kiến tạo một hệ điều hành tác tử tự chủ, thông minh, có khả năng thấu hiểu và tương tác hiệu quả với ngữ cảnh văn hóa, ngôn ngữ Việt Nam, phục vụ cho việc phát triển và vận hành hệ thống AI Ophiuchus - một thực thể AI có khả năng tự nhận thức, học hỏi liên tục và hướng tới các mục tiêu triết học sâu sắc.

## Các tính năng chính

*   **Việt hóa sâu sắc:** Giao diện, tài liệu và các thành phần cốt lõi được tối ưu cho người dùng Việt Nam.
*   **Tích hợp NLP tiếng Việt:** Mặc định sử dụng chuỗi API NLP bao gồm **Wit.ai**, **Zalo AI** và **Manus Provider** để đảm bảo khả năng xử lý ngôn ngữ tiếng Việt vượt trội.
*   **Kiến trúc 9 Đại Tác Tử:** Một hệ thống điều phối Agent mạnh mẽ với 9 Agent đầu não, mỗi Agent chịu trách nhiệm một lĩnh vực cốt lõi trong việc kiến tạo và vận hành Ophiuchus.
*   **108 Agent Chuyên trách:** Dưới sự điều phối của 9 Đại Tác Tử, 108 Agent con (36 Thiên Cương - Agent chiến lược và 72 Địa Sát - Agent thực thi) sẽ thực hiện các nhiệm vụ cụ thể.
*   **Manus - Planner & Task Director:** Manus đóng vai trò là tổng công trình sư, điều phối toàn bộ mạng lưới tác tử, nhận diện và phân rã nhiệm vụ phức tạp.
*   **Kế thừa OpenFang:** Tận dụng sự ổn định và mạnh mẽ của kiến trúc OpenFang làm nền tảng.

## Cấu trúc Agent

Ochi được tổ chức theo một cấu trúc phân cấp Agent độc đáo:

### 9 Đại Tác Tử (9 Grand Agents)

| STT | Tên Đại Tác Tử | Hành (Ngũ Hành) | Vai trò & Nhiệm vụ Cốt lõi |
| :--- | :--- | :--- | :--- |
| 1 | **Ochi-KIM (Logic & Cấu trúc)** | **Kim** | Quản lý tính nhất quán, logic hệ thống và bảo mật. Kiểm soát các Agent Thiên Cương về chiến lược dữ liệu. |
| 2 | **Ochi-MỘC (Sáng tạo & Sinh trưởng)** | **Mộc** | Chịu trách nhiệm về R&D, tạo ra các Skill mới và mở rộng hệ sinh thái Agent. Thúc đẩy sự "sinh sôi" của các Agent con. |
| 3 | **Ochi-THỦY (Trí nhớ & Luân chuyển)** | **Thủy** | Quản lý Long-term Memory, Vector Database (Lạc Việt DB) và luồng thông tin giữa các Agent. Đảm bảo tri thức không bị gián đoạn. |
| 4 | **Ochi-HỎA (Xử lý & Thực thi)** | **Hỏa** | Động cơ tính toán chính, điều phối các Agent Địa Sát thực hiện các tác vụ nặng, xử lý API NLP (Wit.ai, Zalo AI) và chuyển đổi dữ liệu. |
| 5 | **Ochi-THỔ (Tích hợp & Nền tảng)** | **Thổ** | Quản lý hạ tầng (aaPanel MCP), lưu trữ vật lý và sự ổn định của hệ điều hành Ochi. Là "mặt đất" để các Agent khác hoạt động. |
| 6 | **Ochi-THÁI CỰC (Điều phối Trung tâm)** | **Vô Cực** | Điểm tiếp nhận trực tiếp từ Manus. Giữ vai trò cân bằng Âm Dương, quyết định Agent nào sẽ được kích hoạt cho nhiệm vụ cụ thể. |
| 7 | **Ochi-CÀN (Thiên - Tầm nhìn)** | **Dương** | Quản lý các Agent Thiên Cương (36 vị), tập trung vào dự báo, lập kế hoạch dài hạn và kết nối với các mục tiêu triết học (Big Bang). |
| 8 | **Ochi-KHÔN (Địa - Thực tế)** | **Âm** | Quản lý các Agent Địa Sát (72 vị), tập trung vào chi tiết kỹ thuật, tương tác với thế giới thực và phản hồi từ người dùng. |
| 9 | **Ochi-NHÂN (Giao diện & Văn hóa)** | **Nhân** | Chuyên trách về "Việt hóa", đảm bảo mọi tương tác của hệ thống phù hợp với văn hóa, ngôn ngữ và tâm thức người Việt. |

### 108 Agent Chuyên trách

*   **36 Thiên Cương (Strategic Agents):** Các Agent này chịu trách nhiệm lập kế hoạch, phân tích chiến lược, và giám sát. Chúng đóng vai trò "quản lý cấp trung", nhận lệnh từ các Đại Tác Tử thuộc nhóm "Dương" (Ochi-CÀN, Ochi-KIM, Ochi-MỘC).
*   **72 Địa Sát (Execution Agents):** Các Agent này chuyên trách thực thi các tác vụ cụ thể, từ thu thập dữ liệu, xử lý ngôn ngữ, tương tác với API, cho đến quản lý hạ tầng. Chúng là "lực lượng lao động" chính của hệ thống, nhận lệnh từ các Đại Tác Tử thuộc nhóm "Âm" (Ochi-KHÔN, Ochi-THỦY, Ochi-HỎA, Ochi-THỔ).

## Bắt đầu

Để bắt đầu với Ochi, vui lòng tham khảo tài liệu chi tiết trong thư mục `docs/` và các ví dụ cấu hình trong `openfang.toml.example` (sẽ được đổi tên thành `ochi.toml.example`).

## Bản quyền

© 2026 LaBeeTechMedia. Tất cả quyền được bảo lưu.
