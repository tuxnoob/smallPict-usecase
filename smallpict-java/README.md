# SmallPict Java Spring Boot Upload Mini-Service

A robust enterprise-grade image upload mini-service built with **Java 17 & Spring Boot 3 (Spring Data JPA, PostgreSQL, HikariCP)** that integrates with SmallPict image optimization.

## Features
1. **Base API** (`GET /`): Status info and service name.
2. **Health Check** (`GET /health`): PostgreSQL database connectivity health check.
3. **Upload API** (`POST /upload`):
   - Accepts `multipart/form-data` or `application/json` (Base64).
   - Mode `locale` (local storage) & `cdn` (SmallPict CDN delivery).
   - Automatically saves metadata to PostgreSQL `public.images`.

## Quick Start

### 1. Setup Environment
```bash
cp .env.example .env
```

### 2. Build & Run
```bash
mvn spring-boot:run
```

### 3. Package JAR
```bash
mvn clean package -DskipTests
java -jar target/smallpict-java-1.0.0.jar
```
