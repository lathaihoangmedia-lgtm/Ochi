# Web UI Plan

> Kế hoạch làm Web UI cho Ochi Core - Build sau khi xong tech foundation

---

## 🎯 Mục Tiêu

- **Simple Demo UI** - Test inference nhanh
- **Config Panel** - Chỉnh model params
- **Hardware Toggle** - Bật/tắt GPU (nếu khả dụng)
- **Chat Interface** - Test model real-time

---

## 📋 Features Checklist

### Core Features

- [ ] **Model Selector**
  - [ ] Dropdown chọn model đã load
  - [ ] Hiển thị model info (params, VRAM usage)
  - [ ] Nút load/unload model

- [ ] **Inference Panel**
  - [ ] Text input cho prompt
  - [ ] Text output cho response
  - [ ] Streaming text (token by token)
  - [ ] Copy button
  - [ ] Clear button

- [ ] **Settings Panel**
  - [ ] Temperature slider (0.0 - 1.0)
  - [ ] Max tokens slider (1 - 2048)
  - [ ] Context size selector (1024/2048/4096/8192)
  - [ ] Top-P slider
  - [ ] Repeat penalty slider

- [ ] **Hardware Toggle**
  - [ ] GPU On/Off toggle ⭐
  - [ ] CPU threads selector
  - [ ] VRAM usage indicator
  - [ ] RAM usage indicator

- [ ] **Performance Monitor**
  - [ ] Tokens/sec counter
  - [ ] Inference time display
  - [ ] GPU utilization % (nếu có GPU)
  - [ ] Memory usage graph

---

## 🎨 UI Layout

```
┌─────────────────────────────────────────────────────────────┐
│  OCHI CORE - AI Demo                        [Hardware] [⚙️] │
├─────────────────────────────────────────────────────────────┤
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Model: [Qwen3.5-0.8B ▼]  Status: ● Loaded          │   │
│  │ VRAM: 1.2GB / 4GB  ████████░░░░░░░░  30%          │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────┐  ┌──────────────────────┐ │
│  │ ⚙️ Settings                 │  │ 📊 Performance       │ │
│  │ Temperature: [0.7] ───○─── │  │ Tokens/sec: 85       │ │
│  │ Max Tokens: [512] ──○───── │  │ Time: 1.2s           │ │
│  │ Context: [4096 ▼]          │  │ GPU: ████████░░ 80%  │ │
│  │                             │  │                      │ │
│  │ ☑ GPU Enabled              │  │                      │ │
│  └─────────────────────────────┘  └──────────────────────┘ │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Prompt:                                             │   │
│  │ ┌─────────────────────────────────────────────────┐ │   │
│  │ │ Hello, how are you?                             │ │   │
│  │ │                                                 │ │   │
│  │ └─────────────────────────────────────────────────┘ │   │
│  │                                      [Generate] [🗑] │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌─────────────────────────────────────────────────────┐   │
│  │ Response:                                           │   │
│  │ ┌─────────────────────────────────────────────────┐ │   │
│  │ │ I'm doing great! How can I help you today?      │ │   │
│  │ │                                                 │ │   │
│  │ │                                                 │ │   │
│  │ └─────────────────────────────────────────────────┘ │   │
│  │                                      [Copy] [🗑] [⬇] │   │
│  └─────────────────────────────────────────────────────┘   │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 🛠️ Tech Stack

### Frontend Options

**Option 1: React + Vite (Recommended)**
```
- React 18
- Vite (fast build)
- TailwindCSS (styling)
- shadcn/ui (components)
- Zustand (state management)
```

**Option 2: Vanilla JS + HTMX (Simple)**
```
- Vanilla JS
- HTMX (dynamic updates)
- Pico.css (minimal CSS)
- No build step
```

**Option 3: Tauri App (Desktop)**
```
- Tauri (Rust backend)
- React/Svelte (frontend)
- Native app
- No web server needed
```

### Backend Options

**Option 1: Axum (Rust Web Framework)**
```rust
- Axum web server
- Tower middleware
- WebSocket streaming
- CORS support
```

**Option 2: Actix-web**
```rust
- Actix-web server
- Fast performance
- WebSocket support
```

**Option 3: HTTP + FFI (Go)**
```go
- Go HTTP server
- Call Rust FFI
- Simple setup
```

---

## 📁 Project Structure (Planned)

```
ochi/
├── crates/
│   └── ochi-core/          # Rust core (done ✅)
├── apps/
│   └── ochi-webui/         # Web UI app (TODO)
│       ├── src/
│       │   ├── main.rs     # Axum server
│       │   ├── routes.rs   # API endpoints
│       │   └── ws.rs       # WebSocket handler
│       ├── frontend/
│       │   ├── src/
│       │   │   ├── App.tsx
│       │   │   ├── components/
│       │   │   │   ├── ModelSelector.tsx
│       │   │   │   ├── SettingsPanel.tsx
│       │   │   │   ├── InferencePanel.tsx
│       │   │   │   └── HardwareToggle.tsx
│       │   │   └── hooks/
│       │   │       └── useInference.ts
│       │   ├── index.html
│       │   └── package.json
│       └── Cargo.toml
├── docs/
│   └── WEBUI-PLAN.md       # This file
└── README.md
```

---

## 🔌 API Endpoints (Planned)

### REST API

```
GET  /api/health           # Health check
GET  /api/models           # List available models
POST /api/models/load      # Load a model
POST /api/models/unload    # Unload current model
POST /api/generate         # Generate text (non-streaming)
GET  /api/generate/stream  # Generate text (streaming)
GET  /api/hardware         # Get hardware info
POST /api/settings         # Update settings
```

### WebSocket

```
WS /ws/inference           # Real-time inference stream
```

---

## 🚀 Implementation Phases

### Phase 1: MVP (1-2 days)
- [ ] Simple HTML page
- [ ] Text input/output
- [ ] Call Rust FFI directly
- [ ] No styling

### Phase 2: Basic UI (2-3 days)
- [ ] React setup
- [ ] Component structure
- [ ] API integration
- [ ] Basic styling

### Phase 3: Full Features (3-5 days)
- [ ] All settings controls
- [ ] Hardware toggle
- [ ] Performance monitor
- [ ] Streaming support

### Phase 4: Polish (2-3 days)
- [ ] Nice UI/UX
- [ ] Error handling
- [ ] Loading states
- [ ] Responsive design

---

## 📝 Notes

### Important Considerations

1. **GPU Toggle** ⭐
   - Need to reload model when toggling
   - Show warning before reload
   - Save preference to config

2. **Model Management**
   - Support multiple models
   - Quick switch between models
   - Show model card info

3. **Performance**
   - Don't block UI during inference
   - Show progress indicator
   - Cancel button for long generations

4. **Error Handling**
   - OOM errors (VRAM/RAM)
   - Model load failures
   - GPU errors (CUDA/driver)
   - User-friendly messages

---

## 🎨 Design References

- [Ollama Web UI](https://github.com/ollama/ollama)
- [LocalAI](https://github.com/mudler/LocalAI)
- [text-generation-webui](https://github.com/oobabooga/text-generation-webui)
- [LM Studio](https://lmstudio.ai/)

---

## ✅ When to Start

**Start Web UI when:**
- [ ] Rust core is stable ✅
- [ ] FFI bindings work ✅
- [ ] Hardware detection works ✅
- [ ] Basic inference tested ✅
- [ ] Have time for frontend 😄

**Priority:** Low (do after core features complete)

---

**Last Updated:** 2026-03-13
**Status:** Planning Phase 📋
