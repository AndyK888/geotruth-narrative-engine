# Backend Services

The GeoTruth backend is a cloud-based geospatial intelligence layer that provides map matching, POI discovery, and AI narration services. This document covers setup, configuration, and deployment.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Docker Compose Stack                      │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│   ┌──────────────┐     ┌──────────────┐     ┌────────────┐  │
│   │  FastAPI     │────▶│    Redis     │     │  PostGIS   │  │
│   │  (api:8000)  │     │  (cache:6379)│     │  (db:5432) │  │
│   └──────┬───────┘     └──────────────┘     └─────┬──────┘  │
│          │                                         │         │
│          │         ┌───────────────────┐          │         │
│          └────────▶│     Valhalla      │◀─────────┘         │
│                    │ (routing:8002)    │                    │
│                    └───────────────────┘                    │
│                                                              │
└─────────────────────────────────────────────────────────────┘
```

---

## 📁 Directory Structure

```
/backend
├── /app
│   ├── __init__.py
│   ├── main.py              # FastAPI application entry
│   ├── config.py            # Settings management
│   ├── /api
│   │   ├── __init__.py
│   │   ├── routes.py        # API route definitions
│   │   ├── deps.py          # Dependencies (auth, db)
│   │   └── /v1
│   │       ├── enrich.py    # Enrichment endpoints
│   │       ├── narrate.py   # AI narration endpoints
│   │       └── health.py    # Health checks
│   ├── /services
│   │   ├── map_matcher.py   # Valhalla integration
│   │   ├── poi_service.py   # PostGIS queries
│   │   ├── cache_service.py # Redis caching
│   │   └── ai_service.py    # Gemini integration
│   ├── /models
│   │   ├── requests.py      # Pydantic request models
│   │   ├── responses.py     # Pydantic response models
│   │   └── database.py      # SQLAlchemy models
│   └── /utils
│       ├── geo.py           # Geospatial utilities
│       └── fov.py           # Field-of-view calculations
├── /docker
│   ├── Dockerfile.api       # API server image
│   ├── Dockerfile.valhalla  # Routing engine image
│   └── postgis-init.sql     # Database initialization
├── /scripts
│   ├── init-db.sh           # Database setup script
│   ├── import-osm.sh        # OpenStreetMap data import
│   └── seed-pois.py         # POI seeding script
├── docker-compose.yml       # Service orchestration
├── docker-compose.prod.yml  # Production overrides
├── requirements.txt         # Python dependencies
└── .env.example             # Environment template
```

---

## 🚀 Quick Start

### Prerequisites

- Docker 24.x+
- Docker Compose 2.x+
- 8GB+ RAM (for Valhalla routing)
- 50GB+ disk space (for map data)

### Development Setup

```bash
# Clone and navigate
cd backend

# Copy environment file
cp .env.example .env

# Edit with your API keys
vim .env

# Start services
docker-compose up -d

# Check status
docker-compose ps

# View logs
docker-compose logs -f api
```

### Initialize Database

```bash
# Run database migrations
docker-compose exec api alembic upgrade head

# Seed initial POI data
docker-compose exec api python scripts/seed-pois.py
```

---

## ⚙️ Configuration

### Environment Variables

```bash
# .env file

# Database
POSTGRES_HOST=geo-db
POSTGRES_PORT=5432
POSTGRES_USER=geotruth
POSTGRES_PASSWORD=your_secure_password
POSTGRES_DB=geotruth

# Redis
REDIS_URL=redis://cache:6379/0

# Valhalla
VALHALLA_URL=http://map-matcher:8002

# AI Services
GEMINI_API_KEY=your_gemini_api_key

# Security
JWT_SECRET=your_jwt_secret_key
JWT_ALGORITHM=HS256
JWT_EXPIRE_MINUTES=1440

# Logging
LOG_LEVEL=INFO
```

### Python Settings (pydantic-settings)

```python
# app/config.py

from pydantic_settings import BaseSettings
from functools import lru_cache

class Settings(BaseSettings):
    # Database
    postgres_host: str
    postgres_port: int = 5432
    postgres_user: str
    postgres_password: str
    postgres_db: str
    
    # Redis
    redis_url: str
    
    # Valhalla
    valhalla_url: str
    
    # AI
    gemini_api_key: str
    
    # Security
    jwt_secret: str
    jwt_algorithm: str = "HS256"
    jwt_expire_minutes: int = 1440
    
    # Logging
    log_level: str = "INFO"
    
    @property
    def database_url(self) -> str:
        return f"postgresql://{self.postgres_user}:{self.postgres_password}@{self.postgres_host}:{self.postgres_port}/{self.postgres_db}"
    
    class Config:
        env_file = ".env"
        case_sensitive = False

@lru_cache()
def get_settings() -> Settings:
    return Settings()
```

---

## 🐳 Docker Configuration

### docker-compose.yml

```yaml
version: '3.8'

services:
  api:
    build:
      context: .
      dockerfile: docker/Dockerfile.api
    ports:
      - "8000:8000"
    environment:
      - POSTGRES_HOST=geo-db
      - REDIS_URL=redis://cache:6379/0
      - VALHALLA_URL=http://map-matcher:8002
    env_file:
      - .env
    depends_on:
      - geo-db
      - cache
      - map-matcher
    volumes:
      - ./app:/app/app:ro
    restart: unless-stopped

  geo-db:
    image: postgis/postgis:15-3.3
    environment:
      - POSTGRES_USER=${POSTGRES_USER}
      - POSTGRES_PASSWORD=${POSTGRES_PASSWORD}
      - POSTGRES_DB=${POSTGRES_DB}
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./docker/postgis-init.sql:/docker-entrypoint-initdb.d/init.sql
    ports:
      - "5432:5432"
    restart: unless-stopped

  cache:
    image: redis:7-alpine
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data
    restart: unless-stopped

  map-matcher:
    build:
      context: .
      dockerfile: docker/Dockerfile.valhalla
    ports:
      - "8002:8002"
    volumes:
      - valhalla_tiles:/data/valhalla
    restart: unless-stopped

volumes:
  postgres_data:
  redis_data:
  valhalla_tiles:
```

### Dockerfile.api

```dockerfile
# Stage 1: Builder
FROM python:3.11-slim as builder

WORKDIR /app
COPY requirements.txt .
RUN pip wheel --no-cache-dir --no-deps --wheel-dir /app/wheels -r requirements.txt

# Stage 2: Runner
FROM python:3.11-slim

WORKDIR /app

# Install system dependencies for Geo (GDAL/PROJ)
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        libgdal-dev \
        libgeos-dev \
        libproj-dev && \
    rm -rf /var/lib/apt/lists/*

# Install Python packages
COPY --from=builder /app/wheels /wheels
COPY --from=builder /app/requirements.txt .
RUN pip install --no-cache /wheels/*

# Copy application
COPY ./app ./app

# Create non-root user
RUN useradd -m -u 1000 geotruth && \
    chown -R geotruth:geotruth /app
USER geotruth

# Production server
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "8000", "--workers", "4"]
```

---

## 📊 Database Schema

### Core Tables

```sql
-- PostGIS Extension
CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS postgis_topology;

-- POIs (Points of Interest)
CREATE TABLE pois (
    id SERIAL PRIMARY KEY,
    osm_id BIGINT UNIQUE,
    name TEXT NOT NULL,
    name_local TEXT,
    category TEXT NOT NULL,
    subcategory TEXT,
    geom GEOMETRY(Point, 4326) NOT NULL,
    tags JSONB DEFAULT '{}',
    facts JSONB DEFAULT '{}',
    source TEXT DEFAULT 'osm',
    confidence FLOAT DEFAULT 0.8,
    verified BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

-- Spatial index
CREATE INDEX idx_pois_geom ON pois USING GIST (geom);
CREATE INDEX idx_pois_category ON pois (category);
CREATE INDEX idx_pois_name ON pois USING GIN (to_tsvector('english', name));

-- Regions (Countries, States, Counties)
CREATE TABLE regions (
    id SERIAL PRIMARY KEY,
    osm_id BIGINT UNIQUE,
    name TEXT NOT NULL,
    admin_level INT NOT NULL,
    parent_id INT REFERENCES regions(id),
    geom GEOMETRY(MultiPolygon, 4326) NOT NULL,
    timezone TEXT,
    country_code CHAR(2)
);

CREATE INDEX idx_regions_geom ON regions USING GIST (geom);

-- Road Segments (for offline matching)
CREATE TABLE road_segments (
    id SERIAL PRIMARY KEY,
    osm_way_id BIGINT,
    name TEXT,
    road_class TEXT,
    oneway BOOLEAN DEFAULT FALSE,
    geom GEOMETRY(LineString, 4326) NOT NULL,
    tags JSONB DEFAULT '{}'
);

CREATE INDEX idx_roads_geom ON road_segments USING GIST (geom);
```

### Useful Queries

```sql
-- Find POIs within radius
SELECT 
    id, name, category,
    ST_Distance(geom::geography, ST_MakePoint($1, $2)::geography) as distance_m
FROM pois
WHERE ST_DWithin(
    geom::geography,
    ST_MakePoint($1, $2)::geography,
    $3  -- radius in meters
)
ORDER BY distance_m
LIMIT 50;

-- Reverse geocode (find region)
SELECT 
    name, admin_level, timezone
FROM regions
WHERE ST_Contains(geom, ST_MakePoint($1, $2))
ORDER BY admin_level DESC;

-- Find nearest road
SELECT 
    name, road_class,
    ST_Distance(geom::geography, ST_MakePoint($1, $2)::geography) as distance_m
FROM road_segments
ORDER BY geom <-> ST_MakePoint($1, $2)
LIMIT 1;
```

---

## 🔌 API Endpoints

See [API Reference](../api/README.md) for complete documentation.

### Quick Reference

| Endpoint | Method | Description |
|----------|--------|-------------|
| `/v1/health` | GET | Health check |
| `/v1/enrich` | POST | Enrich single GPS point |
| `/v1/enrich_batch` | POST | Enrich multiple points |
| `/v1/map_match` | POST | Match GPS to roads |
| `/v1/narrate` | POST | Generate AI narration |
| `/v1/pois` | GET | Query POIs |

---

## 🗺️ Valhalla Setup

### Building Tiles

```bash
# Download OSM extract (example: North America)
wget https://download.geofabrik.de/north-america-latest.osm.pbf

# Build Valhalla tiles (takes 2-6 hours)
docker run --rm -v $(pwd)/valhalla_tiles:/data/valhalla \
    ghcr.io/gis-ops/docker-valhalla/valhalla:latest \
    valhalla_build_tiles -c /data/valhalla/valhalla.json \
    /data/valhalla/north-america-latest.osm.pbf
```

### Memory Optimization

For limited RAM environments:

```yaml
# docker-compose.override.yml
services:
  map-matcher:
    deploy:
      resources:
        limits:
          memory: 4G
    environment:
      - tile_extract: "/data/valhalla/tiles.tar"
      - concurrency: 2
```

---

## 🔒 Security

### Authentication

All API endpoints (except health) require JWT authentication:

```python
# Example authenticated request
import httpx

async def call_api():
    token = "your_jwt_token"
    headers = {"Authorization": f"Bearer {token}"}
    
    async with httpx.AsyncClient() as client:
        response = await client.post(
            "http://localhost:8000/v1/enrich",
            headers=headers,
            json={"lat": 36.1069, "lon": -112.1129}
        )
        return response.json()
```

### Rate Limiting

```python
# Implemented via slowapi
from slowapi import Limiter
from slowapi.util import get_remote_address

limiter = Limiter(key_func=get_remote_address)

@app.get("/v1/enrich")
@limiter.limit("100/minute")
async def enrich(request: Request):
    ...
```

---

## 📈 Monitoring

### Health Check

```bash
curl http://localhost:8000/v1/health
```

Response:
```json
{
  "status": "healthy",
  "services": {
    "database": "connected",
    "redis": "connected",
    "valhalla": "connected"
  },
  "version": "1.0.0"
}
```

### Logging

Logs are structured JSON for easy parsing:

```json
{
  "timestamp": "2024-01-15T10:30:00Z",
  "level": "INFO",
  "message": "Enrichment completed",
  "request_id": "abc-123",
  "duration_ms": 45,
  "pois_found": 3
}
```

---

## 🚀 Production Deployment

### Using docker-compose.prod.yml

```yaml
# docker-compose.prod.yml
version: '3.8'

services:
  api:
    image: your-registry/geotruth-api:latest
    deploy:
      replicas: 3
      resources:
        limits:
          cpus: '2'
          memory: 2G
    environment:
      - LOG_LEVEL=WARNING
    # Remove volume mounts for immutable deployment
    volumes: []
```

```bash
# Deploy to production
docker-compose -f docker-compose.yml -f docker-compose.prod.yml up -d
```

### Nginx Reverse Proxy

```nginx
upstream geotruth_api {
    server api:8000;
}

server {
    listen 80;
    server_name api.geotruth.example.com;

    location / {
        proxy_pass http://geotruth_api;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    }
}
```

---

## 📚 Related Documentation

- [Architecture Overview](../architecture/README.md)
- [Truth Engine](../architecture/truth-engine.md)
- [API Reference](../api/README.md)
- [Security Guidelines](../security/README.md)
