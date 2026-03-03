# Changelog

All notable changes to Ochi will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-03-03

### Added

- **Phase 6: Orchestration Core (Thái Cực)**: Triển khai `ochi-kernel/src/orchestration.rs` với logic điều phối 9 Đại Tác Tử (KIM, MỘC, THỦY, HỎA, THỔ, THÁI CỰC, CÀN, KHÔN, NHÂN). Bao gồm multi-signal routing, confidence scoring, và phân cực Âm/Dương. (436d82d)
- **Hệ sinh thái Agents**: Khởi tạo cấu trúc 36 Thiên Cương và 72 Địa Sát, sẵn sàng để triển khai các sub-agents chuyên biệt. (95c716e)
- **API Endpoints mới**: Thêm `/api/orchestrate` để điều phối nhiệm vụ và `/api/orchestrate/agents` để liệt kê 9 Đại Tác Tử. (436d82d)
- **Giao thức AI Builder**: Bổ sung `PLAN-TASK.md` với các quy tắc bắt buộc cho AI builder về tôn trọng kế hoạch, tự động cập nhật tiến độ, và phối hợp đa AI. (bd47b07)
- **Chiến lược Pre-release**: Cập nhật `PLAN-TASK.md` với chiến lược phát hành 2 giai đoạn (Web-first, sau đó là Desktop). (221c2e3)

### Changed

- **Phase 3: Brand Rename (OpenFang → Ochi)**: Hoàn thành đổi tên thương hiệu toàn diện trên 13 crates Rust, bao gồm `ochi-kernel`, `ochi-api`, `ochi-cli`, và các thư viện cốt lõi. (59e30eb)
- **Toàn bộ hệ sinh thái**: Cập nhật đổi tên trên tất cả các layer không phải Rust, bao gồm Docker, scripts cài đặt, Web UI (localStorage, JS classes), SDK (Python, JS), và toàn bộ tài liệu. (3b5738d)
- **Cấu hình & Môi trường**: Chuyển đường dẫn mặc định từ `~/.openfang` sang `~/.ochi` và biến môi trường `OPENFANG_HOME` sang `OCHI_HOME`. (b61448a, d1a9032)
- **CI/CD**: Cập nhật workflow `.github/workflows/rust.yml` để sử dụng tên package `ochi-*` mới. (9ae95f7)

### Fixed

- **CLI**: Sửa lỗi cảnh báo `duplicate-target` bằng cách tách `ochi` và `ochi-legacy` thành hai binary riêng biệt. (bcf1e99)
- **Build Dependencies**: Sửa lỗi build do thiếu các dependency của môi trường đồ họa (GUI) và các vấn đề tương thích của `ochi-types`. (1d05652, và các commit trong quá trình debug)

## [0.0.1] - 2026-02-24

### Added

#### Core Platform
- 15-crate Rust workspace: types, memory, runtime, kernel, api, channels, wire, cli, migrate, skills, hands, extensions, desktop, xtask
- Agent lifecycle management: spawn, list, kill, clone, mode switching (Full/Assist/Observe)
- SQLite-backed memory substrate with structured KV, semantic recall, vector embeddings
- 41 built-in tools (filesystem, web, shell, browser, scheduling, collaboration, image analysis, inter-agent, TTS, media)
- WASM sandbox with dual metering (fuel + epoch interruption with watchdog thread)
- Workflow engine with pipelines, fan-out parallelism, conditional steps, loops, and variable expansion
- Visual workflow builder with drag-and-drop node graph, 7 node types, and TOML export
- Trigger system with event pattern matching, content filters, and fire limits
- Event bus with publish/subscribe and correlation IDs
- 7 Hands packages for autonomous agent actions

#### LLM Support
- 3 native LLM drivers: Anthropic, Google Gemini, OpenAI-compatible
- 27 providers: Anthropic, Gemini, OpenAI, Groq, OpenRouter, DeepSeek, Together, Mistral, Fireworks, Cohere, Perplexity, xAI, AI21, Cerebras, SambaNova, Hugging Face, Replicate, Ollama, vLLM, LM Studio, and more
- Model catalog with 130+ built-in models, 23 aliases, tier classification
- Intelligent model routing with task complexity scoring
- Fallback driver for automatic failover between providers
- Cost estimation and metering engine with per-model pricing
- Streaming support (SSE) across all drivers

#### Token Management & Context
- Token-aware session compaction (chars/4 heuristic, triggers at 70% context capacity)
- In-loop emergency trimming at 70%/90% thresholds with summary injection
- Tool profile filtering (cuts default 41 tools to 4-10 for chat agents, saving 15-20K tokens)
- Context budget allocation for system prompt, tools, history, and response
- MAX_TOOL_RESULT_CHARS reduced from 50K to 15K to prevent tool result bloat
- Default token quota raised from 100K to 1M per hour

#### Security
- Capability-based access control with privilege escalation prevention
- Path traversal protection in all file tools
- SSRF protection blocking private IPs and cloud metadata endpoints
- Ed25519 signed agent manifests
- Merkle hash chain audit trail with tamper detection
- Information flow taint tracking
- HMAC-SHA256 mutual authentication for peer wire protocol
- API key authentication with Bearer token
- GCRA rate limiter with cost-aware token buckets
- Security headers middleware (CSP, X-Frame-Options, HSTS)
- Secret zeroization on all API key fields
- Subprocess environment isolation
- Health endpoint redaction (public minimal, auth full)
- Loop guard with SHA256-based detection and circuit breaker thresholds
- Session repair (validates and fixes orphaned tool results, empty messages)

#### Channels
- 40 channel adapters: Telegram, Discord, Slack, WhatsApp, Signal, Matrix, Email, Teams, Mattermost, Google Chat, Webex, Feishu/Lark, LINE, Viber, Facebook Messenger, Mastodon, Bluesky, Reddit, LinkedIn, Twitch, IRC, XMPP, and 18 more
- Unified bridge with agent routing, command handling, message splitting
- Per-channel user filtering and RBAC enforcement
- Graceful shutdown, exponential backoff, secret zeroization on all adapters

#### API
- 100+ REST/WS/SSE API endpoints (axum 0.8)
- WebSocket real-time streaming with per-agent connections
- OpenAI-compatible `/v1/chat/completions` API (streaming SSE + non-streaming)
- OpenAI-compatible `/v1/models` endpoint
- WebChat embedded UI with Alpine.js
- Google A2A protocol support (agent card, task send/get/cancel)
- Prometheus text-format `/api/metrics` endpoint for monitoring
- Multi-session management: list, create, switch, label sessions per agent
- Usage analytics: summary, by-model, daily breakdown
- Config hot-reload via polling (30-second interval, no restart required)

#### Web UI
- Chat message search with Ctrl+F, real-time filtering, text highlighting
- Voice input with hold-to-record mic button (WebM/Opus codec)
- TTS audio playback inline in tool cards
- Browser screenshot rendering in chat (inline images)
- Canvas rendering with iframe sandbox and CSP support
- Session switcher dropdown in chat header
- 6-step first-run setup wizard with provider API key help (12 providers)
- Skill marketplace with 4 tabs (Installed, ClawHub, MCP Servers, Quick Start)
- Copy-to-clipboard on messages, message timestamps
- Visual workflow builder with drag-and-drop canvas

#### Client SDKs
- JavaScript SDK (`@ochi/sdk`): full REST API client with streaming, TypeScript declarations
- Python client SDK (`ochi_client`): zero-dependency stdlib client with SSE streaming
- Python agent SDK (`ochi_sdk`): decorator-based framework for writing Python agents
- Usage examples for both languages (basic + streaming)

#### CLI
- 14+ subcommands: init, start, agent, workflow, trigger, migrate, skill, channel, config, chat, status, doctor, dashboard, mcp
- Daemon auto-detection via PID file
- Shell completion generation (bash, zsh, fish, PowerShell)
- MCP server mode for IDE integration

#### Skills Ecosystem
- 60 bundled skills across 14 categories
- Skill registry with TOML manifests
- 4 runtimes: Python, Node.js, WASM, PromptOnly
- FangHub marketplace with search/install
- ClawHub client for OpenClaw skill compatibility
- SKILL.md parser with auto-conversion
- SHA256 checksum verification
- Prompt injection scanning on skill content

#### Desktop App
- Tauri 2.0 native desktop app
- System tray with status and quick actions
- Single-instance enforcement
- Hide-to-tray on close
- Updated CSP for media, frame, and blob sources

#### Session Management
- LLM-based session compaction with token-aware triggers
- Multi-session per agent with named labels
- Session switching via API and UI
- Cross-channel canonical sessions
- Extended chat commands: `/new`, `/compact`, `/model`, `/stop`, `/usage`, `/think`

#### Image Support
- `ContentBlock::Image` with base64 inline data
- Media type validation (png, jpeg, gif, webp only)
- 5MB size limit enforcement
- Mapped to all 3 native LLM drivers

#### Usage Tracking
- Per-response cost estimation with model-aware pricing
- Usage footer in WebSocket responses and WebChat UI
- Usage events persisted to SQLite
- Quota enforcement with hourly windows

#### Interoperability
- OpenClaw migration engine (YAML/JSON5 to TOML)
- MCP client (JSON-RPC 2.0 over stdio/SSE, tool namespacing)
- MCP server (exposes Ochi tools via MCP protocol)
- A2A protocol client and server
- Tool name compatibility mappings (21 OpenClaw tool names)

#### Infrastructure
- Multi-stage Dockerfile (debian:bookworm-slim runtime)
- docker-compose.yml with volume persistence
- GitHub Actions CI (check, test, clippy, format)
- GitHub Actions release (multi-platform, GHCR push, SHA256 checksums)
- Cross-platform install script (curl/irm one-liner)
- systemd service file for Linux deployment

#### Multi-User
- RBAC with Owner/Admin/User/Viewer roles
- Channel identity resolution
- Per-user authorization checks
- Device pairing and approval system

#### Production Readiness
- 1731+ tests across 15 crates, 0 failures
- Cross-platform support (Linux, macOS, Windows)
- Graceful shutdown with signal handling (SIGINT/SIGTERM on Unix, Ctrl+C on Windows)
- Daemon PID file with stale process detection
- Release profile with LTO, single codegen unit, symbol stripping
- Prometheus metrics for monitoring
- Config hot-reload without restart

[0.1.0]: https://github.com/lathaihoangmedia-lgtm/Ochi/releases/tag/v0.1.0
