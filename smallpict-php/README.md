# SmallPict PHP Upload Mini-Service

A lightweight and fast image upload mini-service built with **PHP 8.1+ (PDO PostgreSQL & Guzzle)** that integrates with SmallPict image optimization.

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
composer install
```

### 3. Run Development Server
```bash
php -S localhost:8005 -t public
```
