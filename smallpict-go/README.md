# SmallPict Go Upload Mini-Service

A high-performance image upload mini-service built with **Go (Gin & GORM PostgreSQL)** that integrates with SmallPict image optimization.

## Features
1. **Base API** (`GET /`): Status info and service name.
2. **Health Check** (`GET /health`): PostgreSQL database connectivity health check.
3. **Upload API** (`POST /upload`):
   - Accepts `multipart/form-data` or `application/json` (Base64).
   - Mode `locale` (local storage `./uploads`) & `cdn` (SmallPict CDN delivery).
   - Automatically saves metadata to PostgreSQL `public.images`.

## Quick Start

### 1. Setup Environment
```bash
cp .env.example .env
```

### 2. Download Dependencies & Run
```bash
go mod tidy
go run main.go
```

### 3. Build Binary
```bash
go build -o smallpict-go main.go
./smallpict-go
```
