# SDK Migration: OpenFang → Ochi (Đã hoàn tất)

> **Trạng thái:** ✅ **Hỗ trợ backward-compatibility đã được xóa hoàn toàn kể từ v0.1.0.**  
> Tất cả các alias và shim cũ (`openfang_*`, `OpenFang`, `OPENFANG_*`) đã bị xóa.  
> Sử dụng trực tiếp các API và tên mới (`Ochi`, `ochi_*`, `OCHI_*`).

---

## Tóm tắt thay đổi

| Thành phần | Tên cũ (đã xóa) | Tên mới (hiện tại) |
| :--- | :--- | :--- |
| JS package | `@openfang/sdk` | `@ochi/sdk` |
| JS class | `OpenFang` | `Ochi` |
| JS error | `OpenFangError` | `OchiError` |
| Python module | `openfang_client` | `ochi_client` |
| Python module | `openfang_sdk` | `ochi_sdk` |
| Python class | `OpenFang` | `Ochi` |
| Config dir | `~/.openfang/` | `~/.ochi/` |
| Env var | `OPENFANG_HOME` | `OCHI_HOME` |
| Env var | `OPENFANG_INSTALL_DIR` | `OCHI_INSTALL_DIR` |
| Env var | `OPENFANG_VERSION` | `OCHI_VERSION` |
| Systemd service | `openfang.service` | `ochi.service` |
| Install binary shim | `openfang` | (removed) |

---

## Cập nhật code

### JavaScript / TypeScript

```js
// SAU (duy nhất được hỗ trợ)
const { Ochi, OchiError } = require("@ochi/sdk");
const client = new Ochi("http://localhost:4200");
```

### Python

```python
# SAU (duy nhất được hỗ trợ)
from ochi_client import Ochi, OchiError
client = Ochi("http://localhost:4200")
```

### Biến môi trường

```bash
# SAU (duy nhất được hỗ trợ)
export OCHI_HOME="/custom/path"
export OCHI_INSTALL_DIR="$HOME/.ochi/bin"
```

---

## Migration thư mục cấu hình

Nếu bạn vẫn còn thư mục `~/.openfang/` cũ, hãy chạy script migration để chuyển sang `~/.ochi/`:

```bash
# Kiểm tra kế hoạch (dry-run)
scripts/migrate-home.sh --dry-run

# Chạy thật
scripts/migrate-home.sh --yes
```

---

## Hỗ trợ

Nếu gặp vấn đề, hãy tạo issue tại: `github.com/lathaihoangmedia-lgtm/Ochi/issues`
