# Docker Deployment Guide

Complete guide to deploying php-rs-toon with Docker.

> **[← Back to Building](BUILDING.md)** | **[Index](INDEX.md)**

## Overview

Three Docker images are provided:
1. **Production** (`Dockerfile.prod`) - Pre-built extension for production use
2. **Development** (`Dockerfile`) - Build from source for development
3. **Benchmark** (`Dockerfile.benchmark`) - Performance testing

## Production Deployment

### Quick Start

```bash
# Build production image
docker build -f Dockerfile.prod -t php-rs-toon:prod .

# Run container
docker run -d --name php-app php-rs-toon:prod php-fpm
```

### Dockerfile.prod Explained

```dockerfile
FROM php:8.2-cli

# Download pre-built extension from GitHub release
ARG TOON_VERSION=1.0.0
RUN apt-get update && apt-get install -y unzip && \
    curl -L https://github.com/mesak/php-rs-toon/releases/download/v${TOON_VERSION}/php-rs-toon-linux-x86_64.zip \
    -o /tmp/ext.zip && \
    unzip /tmp/ext.zip -d /tmp && \
    mv /tmp/libphp_rs_toon.so $(php-config --extension-dir)/

# Enable extension
RUN echo "extension=libphp_rs_toon.so" > /usr/local/etc/php/conf.d/php-rs-toon.ini
```

### Customization

Create your own `Dockerfile`:
```dockerfile
FROM php:8.2-fpm

# Install extension
ARG TOON_VERSION=1.0.0
RUN apt-get update && apt-get install -y unzip && \
    curl -L https://github.com/mesak/php-rs-toon/releases/download/v${TOON_VERSION}/php-rs-toon-linux-x86_64.zip \
    -o /tmp/ext.zip && \
    unzip /tmp/ext.zip -d /tmp && \
    mv /tmp/libphp_rs_toon.so $(php-config --extension-dir)/ && \
    echo "extension=libphp_rs_toon.so" > /usr/local/etc/php/conf.d/php-rs-toon.ini

# Install Composer
COPY --from=composer:latest /usr/bin/composer /usr/bin/composer

# Copy application
WORKDIR /var/www/html
COPY . .

# Install dependencies
RUN composer install --no-dev --optimize-autoloader

EXPOSE 9000
CMD ["php-fpm"]
```

## Docker Compose

### Example Setup

`docker-compose.yml`:
```yaml
version: '3.8'

services:
  app:
    build:
      context: .
      dockerfile: Dockerfile.prod
    volumes:
      - ./:/var/www/html
    environment:
      PHP_MEMORY_LIMIT: 256M
    networks:
      - app-network

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./:/var/www/html
      - ./nginx.conf:/etc/nginx/conf.d/default.conf
    depends_on:
      - app
    networks:
      - app-network

networks:
  app-network:
    driver: bridge
```

### Running

```bash
# Start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f app

# Stop services
docker-compose down
```

## Development Workflow

### Dev Container

```dockerfile
FROM php:8.2-cli

# Install build dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Set working directory
WORKDIR /workspace

# Copy source
COPY . .

# Build extension
RUN cargo build --release && \
    cp target/release/libphp_rs_toon.so $(php-config --extension-dir)/ && \
    echo "extension=libphp_rs_toon.so" > /usr/local/etc/php/conf.d/php-rs-toon.ini

CMD ["bash"]
```

### VS Code Dev Container

`.devcontainer/devcontainer.json`:
```json
{
  "name": "PHP TOON Extension Development",
  "dockerFile": "../Dockerfile",
  "extensions": [
    "rust-lang.rust-analyzer",
    "bmewburn.vscode-intelephense-client"
  ],
  "settings": {
    "rust-analyzer.checkOnSave.command": "clippy"
  },
  "postCreateCommand": "cargo build --release"
}
```

## Multi-Stage Build

For minimal production images:

```dockerfile
# Build stage
FROM rust:1.70 as builder

WORKDIR /build
COPY Cargo.* ./
COPY src ./src

RUN cargo build --release

# Runtime stage
FROM php:8.2-fpm

# Copy extension from builder
COPY --from=builder /build/target/release/libphp_rs_toon.so \
    /usr/local/lib/php/extensions/no-debug-non-zts-20220829/

# Enable extension
RUN echo "extension=libphp_rs_toon.so" > /usr/local/etc/php/conf.d/php-rs-toon.ini

WORKDIR /var/www/html
CMD ["php-fpm"]
```

Benefits:
- Smaller final image (~200MB vs ~2GB)
- No build tools in production
- Better security

## Kubernetes Deployment

### Deployment YAML

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: php-app
spec:
  replicas: 3
  selector:
    matchLabels:
      app: php-app
  template:
    metadata:
      labels:
        app: php-app
    spec:
      containers:
      - name: php-fpm
        image: your-registry/php-rs-toon:latest
        ports:
        - containerPort: 9000
        resources:
          requests:
            memory: "128Mi"
            cpu: "100m"
          limits:
            memory: "256Mi"
            cpu: "500m"
```

### Service YAML

```yaml
apiVersion: v1
kind: Service
metadata:
  name: php-app
spec:
  selector:
    app: php-app
  ports:
  - protocol: TCP
    port: 9000
    targetPort: 9000
```

## Health Checks

### Dockerfile Health Check

```dockerfile
HEALTHCHECK --interval=30s --timeout=3s \
  CMD php -r "if (!function_exists('toon_encode')) exit(1);"
```

### Docker Compose Health Check

```yaml
services:
  app:
    build: .
    healthcheck:
      test: ["CMD", "php", "-r", "if (!function_exists('toon_encode')) exit(1);"]
      interval: 30s
      timeout: 3s
      retries: 3
```

## Best Practices

### 1. Use Multi-Stage Builds

Reduces image size and improves security.

### 2. Pin Versions

```dockerfile
FROM php:8.2.10-fpm
```

### 3. Minimize Layers

```dockerfile
RUN apt-get update && apt-get install -y \
    package1 \
    package2 \
    && rm -rf /var/lib/apt/lists/*
```

### 4. Use .dockerignore

```.dockerignore
target/
.git/
*.md
benchmark/
examples/
```

### 5. Security Scanning

```bash
docker scan php-rs-toon:prod
```

## Troubleshooting

### Extension Not Loading

Check logs:
```bash
docker logs <container-id>
```

Verify extension file:
```bash
docker exec <container-id> ls -la $(php-config --extension-dir)/
```

### Permission Issues

Ensure correct ownership:
```dockerfile
RUN chown -R www-data:www-data /var/www/html
```

## See Also

- **[INSTALLATION.md](INSTALLATION.md)** - Installation guide
- **[BUILDING.md](BUILDING.md)** - Building from source
- **[QUICKSTART.md](../QUICKSTART.md)** - Quick start guide

---

**Navigation**: [← Building](BUILDING.md) | [Index](INDEX.md) | [FAQ →](FAQ.md)
