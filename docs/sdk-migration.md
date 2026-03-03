# SDK Migration Guide: Ochi -> Ochi

Tài liệu này hướng dẫn migration SDK từ naming cũ (`OpenFang`) sang naming mới (`Ochi`) theo hướng **không phá vỡ tương thích ngược**.

## 1) JavaScript/TypeScript SDK

### Package name
- Mới: `@ochi/sdk`
- Cũ: `@ochi/sdk` (legacy naming trong codebase cũ)

### Cài đặt
```bash
npm install @ochi/sdk
```

### Import khuyến nghị
```js
const { Ochi } = require("@ochi/sdk");
const client = new Ochi("http://localhost:3000");
```

### Tương thích ngược
SDK vẫn giữ export cũ để tránh gãy mã hiện có:
- `OpenFang`
- `OpenFangError`

Bạn có thể migrate dần sang:
- `Ochi`
- `OchiError`

---

## 2) Python SDK

### Package name
- Mới: `ochi`
- Cũ: `ochi` (legacy)

### Cài đặt
```bash
pip install ochi
```

### Import khuyến nghị
```python
from ochi_client import Ochi

client = Ochi("http://localhost:3000")
```

### Tương thích ngược
Vẫn hỗ trợ module cũ:
- `openfang_client`
- `openfang_sdk`

Và bổ sung module mới:
- `ochi_client`
- `ochi_sdk`

Alias tương thích:
- `Ochi` (alias từ implementation cũ)
- `OchiError` (alias từ `OpenFangError`)

---

## 3) Checklist migration cho team

1. Đổi lệnh cài đặt trong docs/scripts sang package mới (`@ochi/sdk`, `ochi`).
2. Đổi import mới (`Ochi`) trong ví dụ public-facing.
3. Giữ alias cũ tối thiểu 2-3 phiên bản ổn định trước khi xem xét deprecate.
4. Thông báo migration trong release notes khi cắt bỏ alias legacy.


## 4) Local home directory migration

Khi chuyển runtime từ đường dẫn cũ sang mới, chạy script migration an toàn:

```bash
# kiểm tra kế hoạch trước
scripts/migrate-home.sh --dry-run

# chạy migrate thật
scripts/migrate-home.sh --yes
```

Rollback nếu cần:

```bash
scripts/migrate-home.sh --rollback <BACKUP_PATH> --target ~/.ochi
```

Script này merge theo nguyên tắc **không ghi đè file đã tồn tại ở `~/.ochi`**, và tự tạo backup snapshot để rollback.
