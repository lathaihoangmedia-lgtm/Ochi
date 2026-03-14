# Ochi 108 Architecture Summary

## Core Structure
- **Trung Cung (Thổ)**: Âm–Dương smart router điều phối toàn hệ.
- **Trung Cung Storage**: DuckDB local store cho metadata, registry, routing logs.
- **8 Bát Quái**: các mô-đun định tuyến động (Càn, Khôn, Chấn, Tốn, Khảm, Ly, Cấn, Đoài).
- **5 Ngũ Hành**: 5 agent nền tảng gắn với 5 SQLite DB (kim/moc/thuy/hoa/tho).
- **Thiên Cơ Các**: quản lý 36 Thiên Cương (strategic).
- **Công Hội**: quản lý 72 Địa Sát (execution).

## Automation vs LLM
- **Automation**: Kim, Thủy, Hỏa, Thổ; đa số Bát Quái; Công Hội; phần Âm.
- **LLM**: Mộc; Đoài; 36 Thiên Cương; phần Dương.

## Data Layout
- `data/kim.db` rules/permissions  
- `data/moc.db` knowledge/embeddings  
- `data/thuy.db` message queue/events  
- `data/hoa.db` hot cache/alerts  
- `data/tho.db` system config/metadata  

## Architecture Sources
- `Kiến Trúc Chính Tiếng Việt.txt`
- `Tổng hợp.txt`
- `MVP Development and Integration Resources for Ochi 108.zip`:
  - `README.md`
  - `Ochi_Detailed_Tech_Spec.md`
  - `Ochi 108 - Kiến Trúc Toàn Diện & Lộ Trình Phát Triển.md`
  - `Kiến Trúc Tổng Thể và Lộ Trình Tích Hợp Ochi 108.md`
