# Backend Services

The GeoTruth backend is a **100% Docker-based** geospatial intelligence layer. No local Python, database, or routing engine installation required.

---

## 🐳 Zero Local Dependencies

Everything runs in isolated Docker containers:

```
┌─────────────────────────────────────────────────────────────────┐
│                     Docker Compose Stack                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌──────────────┐   ┌──────────────┐   ┌──────────────────────┐ │
│  │  api         │   │  geo-db      │   │  map-matcher         │ │
│  │  Python 3.12 │   │  PostgreSQL  │   │  Valhalla 3.5        │ │
│  │  FastAPI     │   │  17 + Post-  │   │                      │ │
│  │              │   │  GIS 3.5     │   │                      │ │
│  │  Port: 8000  │   │  Port: 5432  │   │  Port: 8002          │ │
│  └──────┬───────┘   └──────┬───────┘   └──────────┬───────────┘ │
│         │                  │                      │              │
│         └──────────────────┴──────────────────────┘              │
│                            │                                     │
│                    ┌───────┴───────┐                            │
│                    │     cache     │                            │
│                    │   Redis 7.4   │                            │
│                    │   Port: 6379  │                            │
│                    └───────────────┘                            │
├─────────────────────────────────────────────────────────────────┤
│                    Isolated Networks                             │
│  ┌─────────────────────┐    ┌─────────────────────────────────┐ │
│  │  frontend-network   │    │  backend-network (internal)     │ │
│  │  (api exposed)      │    │  (db, cache, matcher isolated)  │ │
│  └─────────────────────┘    └─────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

---

## 📁 Directory Structure

```
/backend
├── /services
│   ├── /api                     # FastAPI application
│   │   ├── Dockerfile           # Multi-stage build
│   │   ├── requirements.txt     # Pinned to latest
│   │   └── /app
│   │       ├── main.py          # Application entry
│   │       ├── config.py        # Pydantic settings
│   │       ├── logging_config.py # Structured logging
│   │       ├── /api             # Route handlers
│   │       ├── /services        # Business logic
│   │       ├── /models          # Pydantic models
│   │       └── /middleware      # Logging, auth, CORS
│   ├── /geo-db                  # PostGIS container
│   │   ├── Dockerfile
│   │   └── /init-scripts        # Database initialization
│   ├── /map-matcher             # Valhalla container
│   │   ├── Dockerfile
│   │   └── /config              # Routing configuration
│   └── /cache                   # Redis container
│       └── redis.conf           # Custom configuration
├── docker-compose.yml           # Production stack
├── docker-compose.dev.yml       # Development with hot reload
├── docker-compose.test.yml      # Testing stack
├── .env.example                 # Environment template
└── Makefile                     # Convenience commands
```

---

## 🚀 Quick Start

### Prerequisites

- **Docker** 24.0+ with Compose v2
- **8GB+ RAM** (for Valhalla routing)
- **50GB+ disk** (for map data)

### Start Services

```bash
# Clone repository
git clone https://github.com/your-org/geotruth-narrative-engine.git
cd geotruth-narrative-engine/backend

# Copy environment template
cp .env.example .env

# Edit with your API keys (GEMINI_API_KEY required)
nano .env

# Start all services
docker compose up -d

# Check status
docker compose ps

# View all logs
docker compose logs -f

# View specific service logs
docker compose logs -f api
```

### Verify Installation

```bash
# Health check
curl http://localhost:8000/v1/health

# Expected response:
{
  "status": "healthy",
  "version": "1.0.0",
  "services": {
    "database": "connected",
    "redis": "connected",
    "valhalla": "connected"
  }
}
```

---

## ⚙️ Docker Configuration

### docker-compose.yml (Production)

```yaml
version: '3.9'

services:
  # ==========================================================================
  # API Server - FastAPI
  # ==========================================================================
  api:
    build:
      context: ./services/api
      dockerfile: Dockerfile
      args:
        PYTHON_VERSION: "3.12"
    image: geotruth/api:latest
    container_name: geotruth-api
    restart: unless-stopped
    ports:
      - "8000:8000"
    environment:
      - ENVIRONMENT=production
      - LOG_LEVEL=INFO
      - LOG_FORMAT=json
      - POSTGRES_HOST=geo-db
      - POSTGRES_PORT=5432
      - REDIS_URL=redis://cache:6379/0
      - VALHALLA_URL=http://map-matcher:8002
    env_file:
      - .env
    depends_on:
      geo-db:
        condition: service_healthy
      cache:
        condition: service_healthy
      map-matcher:
        condition: service_healthy
    networks:
      - frontend
      - backend
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8000/v1/health"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 40s
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 2G
        reservations:
          cpus: '0.5'
          memory: 512M
    logging:
      driver: "json-file"
      options:
        max-size: "100m"
        max-file: "5"
        labels: "service"

  # ==========================================================================
  # PostGIS Database
  # ==========================================================================
  geo-db:
    image: postgis/postgis:17-3.5-alpine
    container_name: geotruth-db
    restart: unless-stopped
    environment:
      - POSTGRES_USER=${POSTGRES_USER:-geotruth}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD:?Database password required}
      - POSTGRES_DB=${POSTGRES_DB:-geotruth}
      - PGDATA=/var/lib/postgresql/data/pgdata
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./services/geo-db/init-scripts:/docker-entrypoint-initdb.d:ro
    networks:
      - backend
    expose:
      - "5432"
    # NOT exposed to host for security - only internal access
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER:-geotruth} -d ${POSTGRES_DB:-geotruth}"]
      interval: 10s
      timeout: 5s
      retries: 5
      start_period: 30s
    deploy:
      resources:
        limits:
          cpus: '2'
          memory: 4G
    logging:
      driver: "json-file"
      options:
        max-size: "50m"
        max-file: "3"

  # ==========================================================================
  # Redis Cache
  # ==========================================================================
  cache:
    image: redis:7.4-alpine
    container_name: geotruth-cache
    restart: unless-stopped
    command: redis-server /usr/local/etc/redis/redis.conf
    volumes:
      - redis_data:/data
      - ./services/cache/redis.conf:/usr/local/etc/redis/redis.conf:ro
    networks:
      - backend
    expose:
      - "6379"
    # NOT exposed to host for security - only internal access
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 10s
      timeout: 5s
      retries: 5
    deploy:
      resources:
        limits:
          cpus: '1'
          memory: 1G
    logging:
      driver: "json-file"
      options:
        max-size: "20m"
        max-file: "3"

  # ==========================================================================
  # Valhalla Map Matching
  # ==========================================================================
  map-matcher:
    image: ghcr.io/gis-ops/docker-valhalla/valhalla:latest
    container_name: geotruth-valhalla
    restart: unless-stopped
    environment:
      - tile_urls=https://download.geofabrik.de/north-america-latest.osm.pbf
      - serve_tiles=True
      - build_elevation=False
      - build_admins=True
      - build_time_zones=True
    volumes:
      - valhalla_tiles:/custom_files
    networks:
      - backend
    expose:
      - "8002"
    # NOT exposed to host for security - only internal access
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8002/status"]
      interval: 30s
      timeout: 10s
      retries: 3
      start_period: 120s  # Valhalla takes time to build tiles
    deploy:
      resources:
        limits:
          cpus: '4'
          memory: 8G
        reservations:
          cpus: '1'
          memory: 2G
    logging:
      driver: "json-file"
      options:
        max-size: "50m"
        max-file: "3"

networks:
  frontend:
    driver: bridge
  backend:
    driver: bridge
    internal: true  # No external access to backend network

volumes:
  postgres_data:
  redis_data:
  valhalla_tiles:
```

### docker-compose.dev.yml (Development)

```yaml
version: '3.9'

# Development overrides - extends docker-compose.yml
# Usage: docker compose -f docker-compose.yml -f docker-compose.dev.yml up

services:
  api:
    build:
      context: ./services/api
      dockerfile: Dockerfile.dev
    volumes:
      - ./services/api/app:/app/app:cached
    environment:
      - ENVIRONMENT=development
      - LOG_LEVEL=DEBUG
      - LOG_FORMAT=pretty
      - RELOAD=true
    ports:
      - "8000:8000"

  geo-db:
    ports:
      - "5432:5432"  # Expose for development tools

  cache:
    ports:
      - "6379:6379"  # Expose for development tools
```

---

## 🏗️ Dockerfile (API Server)

```dockerfile
# =============================================================================
# Stage 1: Builder
# =============================================================================
FROM python:3.12-slim AS builder

WORKDIR /build

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

# Install Python dependencies
COPY requirements.txt .
RUN pip wheel --no-cache-dir --wheel-dir /build/wheels -r requirements.txt

# =============================================================================
# Stage 2: Runtime
# =============================================================================
FROM python:3.12-slim AS runtime

# Labels for container identification
LABEL org.opencontainers.image.title="GeoTruth API"
LABEL org.opencontainers.image.description="Geospatial intelligence API for GeoTruth"
LABEL org.opencontainers.image.version="1.0.0"

WORKDIR /app

# Install runtime dependencies only
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    libgdal-dev \
    libgeos-dev \
    libproj-dev \
    curl \
    && rm -rf /var/lib/apt/lists/* \
    && apt-get clean

# Install Python packages from wheels
COPY --from=builder /build/wheels /wheels
COPY --from=builder /build/requirements.txt .
RUN pip install --no-cache-dir --no-index /wheels/* \
    && rm -rf /wheels

# Copy application code
COPY ./app ./app

# Create non-root user for security
RUN groupadd -r geotruth && useradd -r -g geotruth geotruth \
    && chown -R geotruth:geotruth /app
USER geotruth

# Environment
ENV PYTHONUNBUFFERED=1 \
    PYTHONDONTWRITEBYTECODE=1 \
    PYTHONPATH=/app

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=40s --retries=3 \
    CMD curl -f http://localhost:8000/v1/health || exit 1

# Run with production server
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000", "--workers", "4"]
```

---

## 📊 Structured Logging

All services use structured JSON logging with correlation IDs for request tracing.

### Logging Configuration

```python
# app/logging_config.py

import logging
import sys
import json
from datetime import datetime, timezone
from typing import Any
import uuid
from contextvars import ContextVar

# Context variable for correlation ID
correlation_id_var: ContextVar[str] = ContextVar('correlation_id', default='')

class StructuredFormatter(logging.Formatter):
    """JSON structured log formatter."""
    
    def format(self, record: logging.LogRecord) -> str:
        log_entry = {
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "level": record.levelname,
            "service": "api",
            "logger": record.name,
            "message": record.getMessage(),
            "correlation_id": correlation_id_var.get() or None,
        }
        
        # Add extra fields
        if hasattr(record, 'context'):
            log_entry["context"] = record.context
        
        # Add exception info if present
        if record.exc_info:
            log_entry["exception"] = self.formatException(record.exc_info)
        
        # Add source location for debugging
        log_entry["source"] = {
            "file": record.pathname,
            "line": record.lineno,
            "function": record.funcName
        }
        
        return json.dumps(log_entry)


class PrettyFormatter(logging.Formatter):
    """Human-readable formatter for development."""
    
    COLORS = {
        'DEBUG': '\033[36m',    # Cyan
        'INFO': '\033[32m',     # Green
        'WARNING': '\033[33m',  # Yellow
        'ERROR': '\033[31m',    # Red
        'CRITICAL': '\033[35m', # Magenta
    }
    RESET = '\033[0m'
    
    def format(self, record: logging.LogRecord) -> str:
        color = self.COLORS.get(record.levelname, self.RESET)
        correlation = correlation_id_var.get()
        corr_str = f"[{correlation[:8]}] " if correlation else ""
        
        return (
            f"{color}{record.levelname:8}{self.RESET} "
            f"{corr_str}"
            f"{record.getMessage()}"
        )


def setup_logging(log_level: str = "INFO", log_format: str = "json"):
    """Configure structured logging for the application."""
    
    # Create formatter based on environment
    if log_format == "json":
        formatter = StructuredFormatter()
    else:
        formatter = PrettyFormatter()
    
    # Configure root logger
    root_logger = logging.getLogger()
    root_logger.setLevel(getattr(logging, log_level.upper()))
    
    # Remove existing handlers
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)
    
    # Add stdout handler
    handler = logging.StreamHandler(sys.stdout)
    handler.setFormatter(formatter)
    root_logger.addHandler(handler)
    
    # Reduce noise from third-party libraries
    logging.getLogger("uvicorn.access").setLevel(logging.WARNING)
    logging.getLogger("httpx").setLevel(logging.WARNING)
    logging.getLogger("httpcore").setLevel(logging.WARNING)
    
    return root_logger
```

### Logging Middleware

```python
# app/middleware/logging.py

import time
import uuid
import logging
from fastapi import Request
from starlette.middleware.base import BaseHTTPMiddleware
from ..logging_config import correlation_id_var

logger = logging.getLogger(__name__)

class LoggingMiddleware(BaseHTTPMiddleware):
    """Middleware for request/response logging with correlation IDs."""
    
    async def dispatch(self, request: Request, call_next):
        # Generate or extract correlation ID
        correlation_id = request.headers.get("X-Correlation-ID", str(uuid.uuid4()))
        correlation_id_var.set(correlation_id)
        
        # Log request
        start_time = time.perf_counter()
        logger.info(
            "Request started",
            extra={
                "context": {
                    "method": request.method,
                    "path": request.url.path,
                    "client_ip": request.client.host if request.client else None,
                    "user_agent": request.headers.get("user-agent"),
                }
            }
        )
        
        try:
            response = await call_next(request)
            duration_ms = (time.perf_counter() - start_time) * 1000
            
            # Log response
            logger.info(
                "Request completed",
                extra={
                    "context": {
                        "method": request.method,
                        "path": request.url.path,
                        "status_code": response.status_code,
                        "duration_ms": round(duration_ms, 2),
                    }
                }
            )
            
            # Add correlation ID to response headers
            response.headers["X-Correlation-ID"] = correlation_id
            return response
            
        except Exception as e:
            duration_ms = (time.perf_counter() - start_time) * 1000
            logger.exception(
                "Request failed",
                extra={
                    "context": {
                        "method": request.method,
                        "path": request.url.path,
                        "duration_ms": round(duration_ms, 2),
                        "error": str(e),
                    }
                }
            )
            raise
```

### Example Log Output

**Production (JSON):**
```json
{"timestamp": "2024-01-15T10:30:00.123456+00:00", "level": "INFO", "service": "api", "logger": "app.middleware.logging", "message": "Request started", "correlation_id": "abc12345-6789-def0-1234-567890abcdef", "context": {"method": "POST", "path": "/v1/enrich", "client_ip": "172.18.0.1", "user_agent": "GeoTruth-Desktop/1.0"}, "source": {"file": "/app/app/middleware/logging.py", "line": 35, "function": "dispatch"}}
{"timestamp": "2024-01-15T10:30:00.168456+00:00", "level": "INFO", "service": "api", "logger": "app.services.enrichment", "message": "Enrichment completed", "correlation_id": "abc12345-6789-def0-1234-567890abcdef", "context": {"lat": 36.1069, "lon": -112.1129, "pois_found": 3, "cache_hit": true}, "source": {"file": "/app/app/services/enrichment.py", "line": 89, "function": "enrich_point"}}
{"timestamp": "2024-01-15T10:30:00.170456+00:00", "level": "INFO", "service": "api", "logger": "app.middleware.logging", "message": "Request completed", "correlation_id": "abc12345-6789-def0-1234-567890abcdef", "context": {"method": "POST", "path": "/v1/enrich", "status_code": 200, "duration_ms": 47.12}, "source": {"file": "/app/app/middleware/logging.py", "line": 52, "function": "dispatch"}}
```

**Development (Pretty):**
```
INFO     [abc12345] Request started - POST /v1/enrich
INFO     [abc12345] Enrichment completed - 3 POIs found
INFO     [abc12345] Request completed - 200 in 47.12ms
```

---

## 📦 Requirements (requirements.txt)

```txt
# =============================================================================
# GeoTruth API Dependencies
# Pinned to latest stable versions as of 2024-01
# =============================================================================

# Web Framework
fastapi>=0.115.0
uvicorn[standard]>=0.32.0
starlette>=0.41.0

# Data Validation
pydantic>=2.10.0
pydantic-settings>=2.6.0

# Database
asyncpg>=0.30.0
sqlalchemy>=2.0.36
geoalchemy2>=0.15.0
alembic>=1.14.0

# Caching
redis>=5.2.0
hiredis>=3.0.0

# HTTP Client
httpx>=0.28.0

# Geospatial
shapely>=2.0.6
pyproj>=3.7.0

# AI
google-generativeai>=0.8.0

# Security
python-jose[cryptography]>=3.3.0
passlib[bcrypt]>=1.7.4

# Observability
structlog>=24.4.0

# Utilities
python-multipart>=0.0.17
python-dotenv>=1.0.1
orjson>=3.10.0

# Development (in requirements-dev.txt)
# pytest>=8.3.0
# pytest-asyncio>=0.24.0
# pytest-cov>=6.0.0
# httpx>=0.28.0
# ruff>=0.8.0
# mypy>=1.13.0
```

---

## 🔒 Network Isolation

Services are isolated using Docker networks:

| Network | Services | External Access |
|---------|----------|-----------------|
| `frontend` | api | Yes (port 8000) |
| `backend` | geo-db, cache, map-matcher | **No** (internal only) |

The `backend` network is marked as `internal: true`, meaning:
- No direct access from host machine
- No internet access from containers
- Only accessible through the `api` service

---

## 🧪 Testing

```bash
# Run tests in Docker
docker compose -f docker-compose.test.yml up --abort-on-container-exit

# Or run specific tests
docker compose exec api pytest tests/ -v

# With coverage
docker compose exec api pytest tests/ --cov=app --cov-report=html
```

---

## 📈 Monitoring

### Log Aggregation

All services output JSON logs to stdout, compatible with:
- Docker logging drivers
- ELK Stack
- Grafana Loki
- CloudWatch

### Viewing Logs

```bash
# All services
docker compose logs -f

# Specific service
docker compose logs -f api

# With timestamps
docker compose logs -f --timestamps

# Last N lines
docker compose logs --tail=100 api

# Filter by correlation ID (requires jq)
docker compose logs api | grep "abc12345"
```

---

## 🚀 Production Deployment

### Using Docker Swarm

```bash
# Initialize swarm
docker swarm init

# Deploy stack
docker stack deploy -c docker-compose.yml geotruth

# Check services
docker service ls
```

### Environment Variables

```bash
# Required
POSTGRES_PASSWORD=<strong-password>
GEMINI_API_KEY=<your-gemini-key>
JWT_SECRET=<256-bit-secret>

# Optional
POSTGRES_USER=geotruth
POSTGRES_DB=geotruth
LOG_LEVEL=INFO
```

---

## 📚 Related Documentation

- [Architecture Overview](../architecture/README.md)
- [Logging Guide](../logging.md)
- [API Reference](../api/README.md)
- [Security Guidelines](../security/README.md)
