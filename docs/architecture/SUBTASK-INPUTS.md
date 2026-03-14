# Sub-Task Inputs (Unified Reference)

Mục tiêu: gom kiến thức nền thành 1 file để kích hoạt Sub-Task (schema DuckDB, routing rules, task mẫu).

---

## 1) Kiến trúc lõi Ochi (tóm tắt thống nhất)
- 5 Ngũ Hành → 5 DB riêng (kim/moc/thuy/hoa/tho).
- Trung Cung (Thổ) nắm Âm–Dương: smart router điều hành.
- 8 Bát Quái → 8 tuyến định tuyến động.
- 108 agent → 36 Thiên Cương (strategic) + 72 Địa Sát (execution).
- Thiên Cơ Các quản 36 Thiên Cương; Công Hội quản 72 Địa Sát.
- LLM local: **Ollama dùng ngay** (primary), Candle để giai đoạn deploy ổn định.
- Runtime: `ochi-runtime` gọi Trung Cung để chạy automation tasks theo route.
- Gateway: `ochi-gateway` nhận request và gọi runtime.
- Ollama auto-tune: `OllamaAutoTuner` chọn options theo hardware + model size.

---

## 2) Quy ước 5 lớp dữ liệu (theo LabeeTECH)
1. **Cửu Cung = Schema (tĩnh)**
2. **Bát Quái = Index (phân loại)**
3. **Ngũ Hành = Dynamics (tương sinh/khắc)**
4. **5W+1H = Parsing Rules**
5. **Tam đoạn luận = Constraint Rules**

---

## 3) Hà Đồ – Lạc Thư (tham chiếu cấu trúc)
Nguồn tổng hợp: bài viết về công thức Tinh Hà Đồ thành Lạc Thư (nghiencuulichsu.com, 2019).
Link: https://nghiencuulichsu.com/2019/02/13/cong-thuc-tinh-ha-do-thanh-lac-thu/

Ý nghĩa tích hợp gợi ý:
- Hà Đồ → quy luật sinh/khắc (dynamics).
- Lạc Thư → bố cục 3x3 (schema).
- Dùng để ánh xạ Cửu Cung vào DB và logic routing.

---

## 4) Đề xuất schema DuckDB (Trung Cung – Thổ)
Mục tiêu: lưu metadata điều phối, registry, cache index, routing log.

### 4.1 Bảng nền tảng (core)
- `router_logs`: lịch sử định tuyến (id, intent, bat_quai, notes, time).
- `agent_registry`: danh mục agent + group + trạng thái.
- `task_queue`: hàng đợi task cấp Trung Cung.

### 4.2 Bảng kiến trúc (schema/index/rules)
- `cuu_cung_nodes`: 9 cung + thuộc tính + mapping DB.
- `bat_quai_db_map`: map quẻ → DB/agent group.
- `ngu_hanh_rules`: bảng rule sinh/khắc (from, to, action).
- `rules_5w1h`: template rules cho parse.
- `logic_constraints`: constraint rules (tam đoạn luận).
Ghi chú: bản triển khai hiện đã tạo `cuu_cung_nodes`, `bat_quai_db_map`, `cuu_cung_schema`, `ngu_hanh_rules`.
Ghi chú: đã tạo thêm `rules_5w1h`, `logic_constraints`, `task_flow_logs` và 9 bảng `cung_1..cung_9` trong DuckDB Trung Cung.

---

## 5) Quy tắc routing gốc (heuristics đề xuất)
1. **Parse 5W+1H** từ request → tạo `intent_tags`.
2. **Bát Quái Index**: map `intent_tags` → quẻ chính.
3. **Cửu Cung Schema**: map quẻ → cung → DB/agent target.
4. **Ngũ Hành Dynamics**: áp dụng sinh/khắc (gợi ý mở rộng / filter).
5. **Tam Đoạn Luận**: validate logic trước khi trả kết quả.
6. **Âm–Dương phối hợp**: Âm check cache, Dương quyết định final route.

---

## 6) Task mẫu để test routing
1. "Phân tích báo cáo tài chính tháng này và gửi email cho team."
2. "Tóm tắt 10 tin mới nhất về AI và lưu vào kho tri thức."
3. "Theo dõi realtime cảnh báo hệ thống, nếu quá tải thì báo cáo."
4. "Upload file lớn và stream kết quả phân tích."
5. "Người dùng hỏi chatbot phản hồi nhanh kèm feedback."

---

## 7) Sub-Task cần user kích hoạt (điền thông tin)
- [ ] Chốt schema DuckDB: bảng nào bắt buộc, bảng nào optional.
- [ ] Quy tắc mapping Bát Quái → DB/Agent cụ thể.
- [ ] Quy tắc sinh/khắc cần ưu tiên thực thi.
- [ ] Constraint rules quan trọng (tam đoạn luận nào áp dụng).
- [ ] Task mẫu bổ sung theo domain thực tế.

---

## 8) Thứ tự triển khai 1 → 2 → 3 (theo yêu cầu)
1. **Gán Cửu Cung Lạc Việt vào Bát Quái + DB**
   - Lạc Thư 3x3: 4‑9‑2 / 3‑5‑7 / 8‑1‑6.
   - Trung Cung luôn là số 5.
   - Mapping hiện tại (đã seed vào DuckDB):
     - 1 Bắc = Khảm → Thủy → `thuy.db`
     - 2 Đông Bắc = Khôn → Thổ → `tho.db`
     - 3 Đông = Chấn → Mộc → `moc.db`
     - 4 Đông Nam = Tốn → Mộc → `moc.db`
     - 5 Trung Cung = Thổ → `tho.db`
     - 6 Tây Bắc = Càn → Kim → `kim.db`
     - 7 Tây = Đoài → Kim → `kim.db`
     - 8 Tây Nam = Cấn → Thổ → `tho.db`
     - 9 Nam = Ly → Hỏa → `hoa.db`
2. **Ánh xạ Bát Quái → routing rules**
   - Ưu tiên rules_5w1h trong DuckDB, fallback về tags/intent (chat/stream/cache/webhook/session/search…).
   - Lưu route log vào `router_logs` của DuckDB.
3. **Chuẩn hóa task mẫu & kiểm thử**
   - Dùng 5 task mẫu + bổ sung task thật từ dự án.
   - Test đã thêm: mapping Bát Quái → DB + chọn quẻ theo tag/intent.
   - Test đã thêm: checkpoint lookup + route có checkpoint/DB.
   - Test đã thêm: execute task có log pre/post checkpoint.

---

## 9) Phân bổ số theo Ngũ Hành (checkpoint logic)
- **Thổ (2,5,8)**: 3 checkpoint chính → nền tảng điều hòa, Trung Cung điều hành.
  - 2 = Thổ phụ (ổn định/nuôi dưỡng)
  - 5 = Trung Cung (điều phối)
  - 8 = Thổ phụ (vững chắc/neo)
- **Kim (6,7)**: 2 checkpoint xác thực & quyết định
  - 6 = kỷ luật, policy/auth/role
  - 7 = quyết đoán, rule engine/validation
- **Mộc (3,4)**: 2 checkpoint sinh trưởng & mở rộng
  - 3 = sáng tạo/tri thức nền
  - 4 = mở rộng/điều chỉnh
- **Thủy (1)**: khởi nguồn, dòng dữ liệu ban đầu
- **Hỏa (9)**: năng lượng bùng nổ, tốc độ phản hồi

Nguyên tắc: Âm–Dương (Trung Cung) luôn kiểm tra checkpoint tương ứng trước khi route.
