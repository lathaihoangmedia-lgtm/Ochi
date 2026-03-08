# Hướng Dẫn Lấy Domain (Cấu Hình Tên Miền Tùy Chỉnh)

Mặc định, Ochi chỉ lắng nghe trên `127.0.0.1:4200` (localhost). Hướng dẫn này giúp bạn cấu hình Ochi để truy cập được từ internet thông qua một tên miền thực (ví dụ: `https://ochi.example.com`).

---

## Tổng Quan

Để truy cập Ochi từ bên ngoài, bạn cần:

1. **Đổi địa chỉ lắng nghe** sang `0.0.0.0:4200` (lắng nghe trên tất cả các network interface)
2. **Đặt API key** để bảo mật endpoint
3. **Khai báo CORS origin** cho tên miền của bạn (để trình duyệt có thể gửi request từ domain đó)
4. **(Khuyến nghị)** Đặt một **Reverse Proxy** (nginx hoặc Caddy) phía trước để xử lý HTTPS và load balancing

---

## Bước 1 — Cấu Hình `config.toml`

Mở file `~/.ochi/config.toml` và sửa / thêm các dòng sau:

```toml
# Lắng nghe trên tất cả các interface (0.0.0.0) thay vì chỉ localhost
api_listen = "0.0.0.0:4200"

# Đặt API key để bảo vệ endpoint (bắt buộc khi public)
api_key = "your-secret-api-key-here"

# Cho phép trình duyệt truy cập từ domain tùy chỉnh
# Thêm cả http:// và https:// nếu cần
cors_allowed_origins = [
    "https://ochi.example.com",
    "http://ochi.example.com",
]
```

> **Lưu ý bảo mật:** Không để `api_key` trống khi public ra internet. Bất kỳ ai có địa chỉ IP của bạn đều có thể gọi API nếu không có key.

---

## Bước 2 — Khởi Động Lại Daemon

```bash
# Dừng daemon cũ (nếu đang chạy)
curl -X POST http://127.0.0.1:4200/api/shutdown  2>/dev/null || true

# Khởi động lại
ochi start
```

---

## Bước 3 — Cấu Hình Reverse Proxy (Khuyến Nghị)

Chạy Ochi phía sau một reverse proxy (nginx, Caddy) để có:
- **HTTPS/TLS tự động**
- **Giấu port 4200** — người dùng chỉ thấy `https://ochi.example.com`
- **Rate limiting & WAF** thêm một lớp bảo vệ

### Sử Dụng Caddy (Đơn Giản Nhất)

Cài Caddy: https://caddyserver.com/docs/install

Tạo file `Caddyfile`:

```
ochi.example.com {
    reverse_proxy localhost:4200
}
```

Chạy Caddy:

```bash
caddy run --config Caddyfile
```

Caddy tự động cấp và gia hạn chứng chỉ TLS qua Let's Encrypt. Không cần cấu hình thêm.

---

### Sử Dụng Nginx

Cài nginx rồi tạo file config, ví dụ `/etc/nginx/sites-available/ochi`:

```nginx
server {
    listen 80;
    server_name ochi.example.com;

    # Chuyển hướng tất cả HTTP → HTTPS
    return 301 https://$host$request_uri;
}

server {
    listen 443 ssl http2;
    server_name ochi.example.com;

    # Chứng chỉ TLS (dùng certbot để cấp tự động)
    ssl_certificate     /etc/letsencrypt/live/ochi.example.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/ochi.example.com/privkey.pem;

    # Proxy tất cả request đến Ochi
    location / {
        proxy_pass         http://127.0.0.1:4200;
        proxy_http_version 1.1;

        # Cần thiết cho WebSocket (streaming chat)
        proxy_set_header Upgrade    $http_upgrade;
        proxy_set_header Connection "upgrade";

        proxy_set_header Host              $host;
        proxy_set_header X-Real-IP         $remote_addr;
        proxy_set_header X-Forwarded-For   $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;

        # Tăng timeout cho các phản hồi LLM dài
        proxy_read_timeout 300s;
        proxy_send_timeout 300s;
    }
}
```

Cấp chứng chỉ TLS với certbot:

```bash
sudo apt install certbot python3-certbot-nginx
sudo certbot --nginx -d ochi.example.com
```

Kích hoạt site và reload nginx:

```bash
sudo ln -s /etc/nginx/sites-available/ochi /etc/nginx/sites-enabled/
sudo nginx -t && sudo systemctl reload nginx
```

---

## Bước 4 — Cập Nhật `cors_allowed_origins` Theo Đúng Domain

Sau khi có HTTPS, cập nhật lại `config.toml` chỉ dùng `https://`:

```toml
cors_allowed_origins = ["https://ochi.example.com"]
```

Rồi khởi động lại daemon.

---

## Bước 5 — Kiểm Tra

```bash
# Kiểm tra health endpoint (không cần auth)
curl https://ochi.example.com/api/health

# Kiểm tra API với key
curl -H "Authorization: Bearer your-secret-api-key-here" \
     https://ochi.example.com/api/agents

# Mở trình duyệt
open https://ochi.example.com
```

---

## Sử Dụng Docker với Domain

Nếu dùng Docker, truyền `api_listen` qua biến môi trường hoặc mount config file:

```bash
docker run -d \
  --name ochi \
  -p 4200:4200 \
  -e OCHI_API_KEY="your-secret-api-key-here" \
  -v ~/.ochi:/root/.ochi \
  ghcr.io/lathaihoangmedia-lgtm/ochi:latest
```

Rồi cấu hình nginx/Caddy trỏ về `localhost:4200` như trên.

---

## Bảng Tóm Tắt Các Trường Config Liên Quan

| Trường                  | Kiểu           | Mặc Định            | Mô Tả                                                          |
|-------------------------|----------------|---------------------|----------------------------------------------------------------|
| `api_listen`            | string         | `"127.0.0.1:4200"`  | Địa chỉ bind của HTTP/WS server. Dùng `"0.0.0.0:4200"` khi public. |
| `api_key`               | string         | `""`                | Bearer token bảo vệ API. Để trống = không cần xác thực (chỉ dùng local). |
| `cors_allowed_origins`  | list of string | `[]`                | Danh sách origin được phép gửi request từ trình duyệt (CORS). Ví dụ: `["https://ochi.example.com"]`. |

---

## Câu Hỏi Thường Gặp

**Q: Tôi có cần reverse proxy không?**  
A: Không bắt buộc, nhưng rất khuyến nghị. Reverse proxy xử lý TLS, giúp bảo mật và ổn định hơn khi public.

**Q: Ochi có hỗ trợ TLS trực tiếp không?**  
A: Chưa. Hiện tại Ochi chỉ phục vụ HTTP thuần. Hãy dùng Caddy hoặc nginx để terminate TLS.

**Q: WebSocket có hoạt động qua reverse proxy không?**  
A: Có, miễn là cấu hình đúng header `Upgrade` và `Connection` như ví dụ nginx ở trên.

**Q: Tôi quên mất API key thì làm sao?**  
A: Mở `~/.ochi/config.toml`, sửa trường `api_key` thành key mới, rồi restart daemon.

---

*Xem thêm: [Cấu Hình Đầy Đủ](configuration.md) | [Bắt Đầu Nhanh](getting-started.md) | [API Reference](api-reference.md)*
