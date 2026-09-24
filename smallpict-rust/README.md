# SmallPict Rust Upload Mini-Service

A blazing-fast and memory-safe image upload mini-service built with **Rust (Axum + SQLx + Reqwest)** that connects to PostgreSQL and optimizes images using SmallPict.

## Features
1. **Base API** (`GET /`): Status info and service name.
2. **Health Check** (`GET /health`): PostgreSQL connectivity health check.
3. **Upload API** (`POST /api/upload` / `POST /upload`):
   - Supports `multipart/form-data` and `application/json` (Base64).
   - Mode `locale` (local storage) & `cdn` (SmallPict CDN delivery).
   - Saves record to PostgreSQL `public.images`.

## Quick Start

### 1. Setup Environment
```bash
cp .env.example .env
```

### 2. Run
```bash
cargo run
```

### 3. Build Release
```bash
cargo build --release
```
