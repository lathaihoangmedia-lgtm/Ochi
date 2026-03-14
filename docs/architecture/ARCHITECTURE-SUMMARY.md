# Ochi 108 Architecture Summary

## Core Structure
- **Trung Cung (Thổ)**: Âm–Dương smart router điều phối toàn hệ.
- **Trung Cung Storage**: DuckDB local store cho metadata, registry, routing logs.

## Ground Crates (9 crates theo plan)
1. `crates/ochi-core` — error + utils nền.
2. `crates/ochi-automation` — tools/skills/prompts (automation layer).
3. `crates/ochi-trung-cung` — Âm‑Dương + Bát Quái + DuckDB.
4. `crates/ochi-ngu-hanh` — 5 DB + 5 agent nền.
5. `crates/ochi-thien-co-cac` — quản 36 Thiên Cương (scaffold).
6. `crates/ochi-cong-hoi` — quản 72 Địa Sát (scaffold).
7. `crates/ochi-llm` — LLM integration (Ollama/Candle).
8. `crates/ochi-gateway` — API/gateway routing.
9. `crates/ochi-runtime` — execution/runtime glue.
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
