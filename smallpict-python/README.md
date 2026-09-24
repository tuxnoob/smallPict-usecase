# SmallPict Python Upload Mini-Service

A high-performance Python mini-service built with **FastAPI** and **SQLAlchemy** (PostgreSQL) for handling image uploads with SmallPict optimization (supporting both Locale and CDN modes, Multipart/form-data, and Base64).

## Features
1. **Base API** (`GET /`): Service status info.
2. **Health Check** (`GET /health`): PostgreSQL database connectivity health check.
3. **Upload API** (`POST /upload`):
   - Accepts `multipart/form-data` or `application/json` (Base64).
   - Optimizes image with SmallPict API.
   - Saves record to PostgreSQL `public.images` table.

## Quick Start

### 1. Setup Environment
```bash
cp .env.example .env
```

### 2. Install Dependencies & Run
```bash
pip install -r requirements.txt
python main.py
```

### 3. Docker
```bash
docker build -t smallpict-python .
docker run -p 8000:8000 --env-file .env smallpict-python
```
