# SmallPict Ruby Sinatra Upload Mini-Service

An elegant and lightweight image upload mini-service built with **Ruby (Sinatra + Sequel + PostgreSQL)** with SmallPict optimization.

## Features
1. **Base API** (`GET /`): Status info and service name.
2. **Health Check** (`GET /health`): PostgreSQL database connectivity health check.
3. **Upload API** (`POST /upload`):
   - Supports `multipart/form-data` and `application/json` (Base64).
   - Mode `locale` (local file storage) & `cdn` (SmallPict CDN delivery).
   - Saves record to PostgreSQL `public.images`.

## Quick Start

### 1. Setup Environment
```bash
cp .env.example .env
```

### 2. Install Dependencies & Run
```bash
bundle install
ruby app.rb
# or via Puma / rackup:
bundle exec puma -p 8003
```
