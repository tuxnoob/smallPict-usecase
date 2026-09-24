# SmallPict Multi-Language Image Upload Mini-Services (Monorepo)

A collection of lightweight, production-ready image upload mini-services implemented across **7 different programming languages and frameworks**. Each service integrates seamlessly with the **[SmallPict Image Optimization API](https://smallpict.app)** to provide next-gen image transcoding (AVIF, WebP), smart compression, and storage management (local file storage or SmallPict Edge CDN).

---

## 🚀 Supported Languages & Frameworks

| Directory | Language / Framework | Database ORM / Driver | Default Port |
| :--- | :--- | :--- | :--- |
| **[`smallpict-go`](./smallpict-go)** | Go (Gin) | GORM PostgreSQL | `8002` |
| **[`smallpict-java`](./smallpict-java)** | Java 17 (Spring Boot 3) | Spring Data JPA / HikariCP | `8004` |
| **[`smallpict-js`](./smallpict-js)** | Node.js / TypeScript (Express) | `pg` Pool | `8000` |
| **[`smallpict-php`](./smallpict-php)** | PHP 8.1+ (Native MVC) | PDO PostgreSQL & Guzzle | `8005` |
| **[`smallpict-python`](./smallpict-python)** | Python 3.9+ (FastAPI) | SQLAlchemy | `5005` |
| **[`smallpict-ruby`](./smallpict-ruby)** | Ruby 3.0+ (Sinatra) | Sequel ORM | `5005` |
| **[`smallpict-rust`](./smallpict-rust)** | Rust (Axum 0.7) | SQLx (Async PostgreSQL) | `5005` |

---

## 🌟 Common Architecture & Features

All services implement a unified API contract and consistent business logic:

1. **Dual Optimization Modes**:
   - `locale`: Optimizes the image through SmallPict, downloads the compressed result, and stores it in the local `./uploads` directory.
   - `cdn`: Uses the SmallPict Edge CDN URL directly for fast global delivery without using local disk storage.
2. **Dual Payload Ingestion**:
   - **`multipart/form-data`**: Upload binary image files directly via form-data (`image` or `file` field).
   - **`application/json`**: Upload Base64-encoded images (`image` or `base64` field).
3. **Database Logging**:
   - Automatically saves metadata (UUID, filename, URL, compressed size, original size, MIME type, created timestamp) into PostgreSQL (`public.images`).
4. **Health Check**:
   - `GET /health` endpoint for monitoring database connectivity and service readiness.

---

## 🗄️ Database Schema

All services share the same PostgreSQL table schema:

```sql
CREATE TABLE IF NOT EXISTS public.images (
    id VARCHAR(50) PRIMARY KEY,
    filename VARCHAR(255),
    url TEXT,
    size BIGINT,
    size_origin BIGINT,
    mime_type VARCHAR(100),
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

---

## ⚙️ Environment Variables

Each subproject includes a `.env.example` file. Copy it to `.env` in the target directory:

```env
# Application Settings
APP_NAME="SmallPict Service"
APP_PORT=5005
APP_HOST=0.0.0.0

# PostgreSQL Database Configuration
DB_HOST=localhost
DB_PORT=5432
DB_NAME=smallpict
DB_USER=postgres
DB_PASS=postgres
DB_SSLMODE=disable

# SmallPict Configuration
SMALLPICT_MODE=locale # locale | cdn
SMALLPICT_BASE_URL=https://api.smallpict.app
SMALLPICT_API_KEY=your_api_key_here
SMALLPICT_SECRET_KEY=your_secret_key_here
SMALLPICT_FORMAT=auto # auto | webp | avif | jpeg | png
SMALLPICT_QUALITY=80 # 1 - 100

# Local Storage Directory (used when SMALLPICT_MODE=locale)
UPLOAD_PATH=./uploads
```

---

## 📡 Unified API Specification

### 1. Base Service Info
- **Method:** `GET /`
- **Response:**
```json
{
  "status": "ok",
  "service": "SmallPict Python Service",
  "version": "1.0.0"
}
```

---

### 2. Database Health Check
- **Method:** `GET /health`
- **Response (`200 OK`):**
```json
{
  "status": "ok",
  "database": "connected",
  "timestamp": "2026-09-24T08:00:00Z"
}
```

---

### 3. Upload Image API
- **Method:** `POST /upload`

#### A. Multipart Form-Data (File Upload)
```bash
curl --location 'http://127.0.0.1:5005/upload' \
  --form 'image=@"/path/to/image.jpg"'
```

#### B. JSON Payload (Base64)
```bash
curl --location 'http://127.0.0.1:5005/upload' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "sample.jpg",
    "image": "data:image/jpeg;base64,/9j/4AAQSkZJRg..."
  }'
```

#### Success Response (`200 OK`):
```json
{
  "status": "success",
  "message": "Image uploaded successfully",
  "data": {
    "id": "88ebc5ca-ebde-4985-94b0-044fe7bb096a",
    "filename": "sample_1790236606.webp",
    "url": "/uploads/sample_1790236606.webp",
    "size": 43846,
    "size_origin": 111678,
    "mime_type": "image/webp",
    "created_at": "2026-09-24T14:56:49Z"
  }
}
```

---

## 🛠️ Quick Start by Language

### 1. Go (`smallpict-go`)
```bash
cd smallpict-go
cp .env.example .env
go mod tidy
go run main.go
```

### 2. Java Spring Boot (`smallpict-java`)
```bash
cd smallpict-java
cp .env.example .env
mvn spring-boot:run
```

### 3. Node.js / TypeScript (`smallpict-js`)
```bash
cd smallpict-js
cp .env.example .env
npm install
npm run dev
```

### 4. PHP (`smallpict-php`)
```bash
cd smallpict-php
cp .env.example .env
composer install
php -S localhost:8005 -t public
```

### 5. Python FastAPI (`smallpict-python`)
```bash
cd smallpict-python
cp .env.example .env
python -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python main.py
```

### 6. Ruby Sinatra (`smallpict-ruby`)
```bash
cd smallpict-ruby
cp .env.example .env
bundle install
ruby app.rb
```

### 7. Rust Axum (`smallpict-rust`)
```bash
cd smallpict-rust
cp .env.example .env
cargo run
```

---

## 📄 License
MIT License. Open-source and free to use.
