# SmallPict Node.js / JavaScript Upload Mini-Service

A modern and high-speed image upload mini-service built with **Node.js, Express & TypeScript (PostgreSQL pg pool)** that integrates with SmallPict image optimization.

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

### 2. Install Dependencies
```bash
npm install
```

### 3. Run Development Server
```bash
npm run dev
```

### 4. Build & Run Production Bundle
```bash
npm run build
npm start
```
