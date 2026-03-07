# SDK Migration Guide: OpenFang → Ochi

> **Cập nhật lần cuối:** 07-03-2026  
> **Áp dụng cho:** Tất cả người dùng đang sử dụng SDK phiên bản `openfang-*` hoặc `OpenFang` API

Tài liệu này hướng dẫn đầy đủ việc migration từ naming cũ (`OpenFang`) sang naming mới (`Ochi`).
Chiến lược migration theo hướng **không phá vỡ tương thích ngược**: các alias cũ vẫn hoạt động
cho đến khi được thông báo deprecated.

---

## 1. Tổng quan thay đổi

| Thành phần | Tên cũ (OpenFang) | Tên mới (Ochi) | Trạng thái |
| :--- | :--- | :--- | :--- |
| JS package | `@openfang/sdk` | `@ochi/sdk` | ✅ Mới |
| JS class chính | `OpenFang` | `Ochi` | ⚠️ Alias — sẽ deprecated |
| JS error class | `OpenFangError` | `OchiError` | ⚠️ Alias — sẽ deprecated |
| Python package | `openfang` | `ochi` | ✅ Mới |
| Python module | `openfang_client` | `ochi_client` | ⚠️ Alias — sẽ deprecated |
| Python class | `OpenFang` | `Ochi` | ⚠️ Alias — sẽ deprecated |
| Config dir | `~/.openfang/` | `~/.ochi/` | ✅ Đã migrate tự động |
| Env vars | `OPENFANG_*` | `OCHI_*` | ⚠️ Backward-compat supported |
| Systemd service | `openfang.service` | `ochi.service` | ✅ Đã đổi tên |

---

## 2. JavaScript / TypeScript SDK

### 2.1. Cài đặt

```bash
# Package mới (khuyến nghị)
npm install @ochi/sdk

# Package cũ — vẫn hoạt động như alias trong giai đoạn chuyển tiếp
# npm install @openfang/sdk  ← sẽ deprecated trong v0.2.0
```

### 2.2. Migration import

```js
// TRƯỚC (legacy — vẫn hoạt động nhưng sẽ in deprecation warning)
const { OpenFang, OpenFangError } = require("@openfang/sdk");
const client = new OpenFang("http://localhost:4200");

// SAU (khuyến nghị)
const { Ochi, OchiError } = require("@ochi/sdk");
const client = new Ochi("http://localhost:4200");
```

**TypeScript:**
```typescript
// TRƯỚC
import { OpenFang } from "@openfang/sdk";

// SAU
import { Ochi } from "@ochi/sdk";
const client = new Ochi("http://localhost:4200");
```

### 2.3. API không thay đổi

Tất cả các method đều giữ nguyên — chỉ đổi tên class:

```js
const { Ochi } = require("@ochi/sdk");
const client = new Ochi("http://localhost:4200");

// Tất cả API cũ vẫn hoạt động bình thường
const agents = await client.agents.list();
const reply  = await client.agents.message(agents[0].id, "Xin chào!");

// Streaming — không thay đổi
for await (const event of client.agents.stream(agentId, "Kể một câu chuyện")) {
  process.stdout.write(event.delta || "");
}
```

### 2.4. Xử lý lỗi

```js
const { Ochi, OchiError } = require("@ochi/sdk");
try {
  const reply = await client.agents.message(agentId, "Hello");
} catch (err) {
  if (err instanceof OchiError) {
    console.error(`Ochi API error ${err.status}:`, err.message);
  }
}
```

---

## 3. Python SDK

### 3.1. Cài đặt

```bash
# Package mới (khuyến nghị)
pip install ochi

# Package cũ — sẽ deprecated trong v0.2.0
# pip install openfang
```

### 3.2. Migration import

```python
# TRƯỚC (legacy — vẫn hoạt động, module được giữ làm backward-compat shim)
from openfang_client import OpenFang
client = OpenFang("http://localhost:4200")

# SAU (khuyến nghị)
from ochi_client import Ochi
client = Ochi("http://localhost:4200")
```

### 3.3. Các module tương đương

| Module cũ | Module mới | Ghi chú |
| :--- | :--- | :--- |
| `openfang_client` | `ochi_client` | Module chính |
| `openfang_sdk` | `ochi_sdk` | SDK wrapper |
| `OpenFang` class | `Ochi` class | Class chính |
| `OpenFangError` | `OchiError` | Exception |

### 3.4. Ví dụ đầy đủ

```python
from ochi_client import Ochi, OchiError

client = Ochi("http://localhost:4200")

try:
    agents = client.agents.list()
    if agents:
        reply = client.agents.message(agents[0]["id"], "Xin chào Ochi!")
        print(reply["content"])
except OchiError as e:
    print(f"Lỗi API: {e}")
```

---

## 4. Biến môi trường

Các biến môi trường `OPENFANG_*` vẫn được hỗ trợ như backward-compat aliases.
Khuyến nghị migrate sang `OCHI_*`:

```bash
# TRƯỚC
export OPENFANG_API_KEY="sk-..."
export OPENFANG_BASE_URL="http://localhost:4200"

# SAU (khuyến nghị)
export OCHI_API_KEY="sk-..."
export OCHI_BASE_URL="http://localhost:4200"
```

> **Lưu ý:** Cả hai tên biến đều được đọc trong giai đoạn chuyển tiếp. Nếu cả hai được set,
> `OCHI_*` sẽ được ưu tiên.

---

## 5. Thư mục cấu hình local

Thư mục cấu hình đã được đổi từ `~/.openfang/` sang `~/.ochi/`.

### 5.1. Migration tự động (khuyến nghị)

```bash
# Kiểm tra kế hoạch trước (dry-run)
scripts/migrate-home.sh --dry-run

# Chạy migration thật
scripts/migrate-home.sh --yes
```

### 5.2. Rollback

```bash
scripts/migrate-home.sh --rollback <BACKUP_PATH> --target ~/.ochi
```

Script migrate theo nguyên tắc **không ghi đè file đã tồn tại ở `~/.ochi`**
và tự tạo backup snapshot để rollback an toàn.

### 5.3. Migration thủ công

```bash
# Backup trước
cp -r ~/.openfang ~/.openfang.backup

# Copy sang vị trí mới (không xóa cũ)
cp -rn ~/.openfang/. ~/.ochi/

# Kiểm tra
ls ~/.ochi/
```

---

## 6. Systemd Service

```bash
# Dừng service cũ
sudo systemctl stop openfang
sudo systemctl disable openfang

# Cài service mới
sudo cp deploy/ochi.service /etc/systemd/system/
sudo systemctl daemon-reload
sudo systemctl enable ochi
sudo systemctl start ochi

# Kiểm tra
sudo systemctl status ochi
```

---

## 7. Lịch trình Deprecation

| Phiên bản | Thay đổi |
| :--- | :--- |
| `v0.1.0` (hiện tại) | Tất cả alias cũ (`openfang_*`, `OpenFang`) vẫn hoạt động. Không có warning. |
| `v0.2.0` (kế hoạch) | Alias cũ sẽ in `DeprecationWarning` khi import. |
| `v0.3.0` (kế hoạch) | Alias cũ bị xóa. **Breaking change.** |

> Ngày cụ thể sẽ được công bố trong `CHANGELOG.md` và release notes trước ít nhất 60 ngày.

---

## 8. Checklist Migration cho Team

- [ ] Cập nhật `package.json` / `requirements.txt`: đổi `@openfang/sdk` → `@ochi/sdk`, `openfang` → `ochi`
- [ ] Tìm và đổi tất cả import: `OpenFang` → `Ochi`, `OpenFangError` → `OchiError`
- [ ] Đổi biến môi trường trong `.env`, CI/CD secrets: `OPENFANG_*` → `OCHI_*`
- [ ] Chạy `scripts/migrate-home.sh` trên mỗi máy dev và server
- [ ] Cập nhật systemd service file: `openfang.service` → `ochi.service`
- [ ] Chạy test suite sau migration để xác nhận không có regression
- [ ] Cập nhật tài liệu nội bộ và README

---

## 9. Hỗ trợ

Nếu gặp vấn đề trong quá trình migration, vui lòng:

1. Tạo issue trên GitHub: `github.com/lathaihoangmedia-lgtm/Ochi/issues`
2. Xem thêm: `docs/troubleshooting.md`
3. Đọc CHANGELOG: `CHANGELOG.md`
