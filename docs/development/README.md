# Development Guide

GeoTruth development is **100% Docker-based**. No local toolchain installation required.

---

## 🐳 Zero Local Dependencies

You don't need to install:
- ❌ Rust
- ❌ Node.js
- ❌ Python
- ❌ PostgreSQL
- ❌ FFmpeg

Everything runs in Docker containers.

---

## 🚀 Quick Start

### Prerequisites

| Requirement | Version |
|-------------|---------|
| **Docker** | 24.0+ with Compose v2 |
| **RAM** | 16GB recommended |
| **Disk** | 50GB free space |

### Clone and Start

```bash
# Clone repository
git clone https://github.com/your-org/geotruth-narrative-engine.git
cd geotruth-narrative-engine

# Start full development stack
docker compose -f docker-compose.dev.yml up -d

# Check status
docker compose -f docker-compose.dev.yml ps
```

### Access Points

| Service | URL |
|---------|-----|
| **Desktop Dev** | http://localhost:5173 |
| **API Docs** | http://localhost:8000/docs |
| **API Health** | http://localhost:8000/v1/health |

---

## 📁 Project Structure

```
geotruth-narrative-engine/
├── /backend                      # Docker-based backend
│   ├── /services
│   │   ├── /api                  # FastAPI (Python 3.12)
│   │   ├── /geo-db               # PostGIS 17
│   │   ├── /map-matcher          # Valhalla 3.5
│   │   └── /cache                # Redis 7.4
│   ├── docker-compose.yml        # Production
│   └── docker-compose.dev.yml    # Development
├── /desktop                      # Self-contained desktop app
│   ├── /src                      # React frontend
│   ├── /src-tauri                # Rust backend
│   ├── Dockerfile.dev            # Dev container
│   └── docker-compose.dev.yml    # Dev orchestration
├── /docs                         # Documentation
└── docker-compose.dev.yml        # Full-stack development
```

---

## 🔧 Full-Stack Development

### docker-compose.dev.yml (Root)

```yaml
version: '3.9'

services:
  # ==========================================================================
  # Desktop Development
  # ==========================================================================
  desktop:
    build:
      context: ./desktop
      dockerfile: Dockerfile.dev
    volumes:
      - ./desktop/src:/app/src:cached
      - ./desktop/src-tauri/src:/app/src-tauri/src:cached
      - desktop-cargo:/usr/local/cargo/registry
      - desktop-target:/app/src-tauri/target
      - desktop-node:/app/node_modules
    ports:
      - "5173:5173"
    environment:
      - RUST_LOG=debug
      - RUST_BACKTRACE=1
      - API_URL=http://api:8000
    depends_on:
      - api
    networks:
      - frontend

  # ==========================================================================
  # API Server
  # ==========================================================================
  api:
    build:
      context: ./backend/services/api
      dockerfile: Dockerfile.dev
    volumes:
      - ./backend/services/api/app:/app/app:cached
    ports:
      - "8000:8000"
    environment:
      - ENVIRONMENT=development
      - LOG_LEVEL=DEBUG
      - LOG_FORMAT=pretty
      - POSTGRES_HOST=geo-db
      - REDIS_URL=redis://cache:6379/0
      - VALHALLA_URL=http://map-matcher:8002
    env_file:
      - ./backend/.env
    depends_on:
      geo-db:
        condition: service_healthy
      cache:
        condition: service_healthy
    networks:
      - frontend
      - backend

  # ==========================================================================
  # PostGIS Database
  # ==========================================================================
  geo-db:
    image: postgis/postgis:17-3.5-alpine
    volumes:
      - postgres-data:/var/lib/postgresql/data
      - ./backend/services/geo-db/init-scripts:/docker-entrypoint-initdb.d:ro
    environment:
      - POSTGRES_USER=geotruth
      - POSTGRES_PASSWORD=devpassword
      - POSTGRES_DB=geotruth
    ports:
      - "5432:5432"
    networks:
      - backend
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U geotruth"]
      interval: 5s
      timeout: 5s
      retries: 5

  # ==========================================================================
  # Redis Cache
  # ==========================================================================
  cache:
    image: redis:7.4-alpine
    ports:
      - "6379:6379"
    networks:
      - backend
    healthcheck:
      test: ["CMD", "redis-cli", "ping"]
      interval: 5s
      timeout: 5s
      retries: 5

  # ==========================================================================
  # Valhalla Map Matching
  # ==========================================================================
  map-matcher:
    image: ghcr.io/gis-ops/docker-valhalla/valhalla:latest
    volumes:
      - valhalla-tiles:/custom_files
    environment:
      - tile_urls=https://download.geofabrik.de/north-america/us/california-latest.osm.pbf
      - serve_tiles=True
      - build_elevation=False
    ports:
      - "8002:8002"
    networks:
      - backend

networks:
  frontend:
  backend:

volumes:
  postgres-data:
  valhalla-tiles:
  desktop-cargo:
  desktop-target:
  desktop-node:
```

---

## 🛠️ Development Workflows

### Starting Development

```bash
# Start all services
docker compose -f docker-compose.dev.yml up -d

# View logs
docker compose -f docker-compose.dev.yml logs -f

# View specific service
docker compose -f docker-compose.dev.yml logs -f api
```

### Hot Reload

All services support hot reload:

| Service | Trigger |
|---------|---------|
| **Desktop Frontend** | Save `.tsx` file |
| **Desktop Backend** | Save `.rs` file |
| **API** | Save `.py` file |

### Running Commands Inside Containers

```bash
# API shell
docker compose -f docker-compose.dev.yml exec api bash

# Run Python command
docker compose -f docker-compose.dev.yml exec api python -c "print('hello')"

# Run Rust command
docker compose -f docker-compose.dev.yml exec desktop cargo test

# Run npm command
docker compose -f docker-compose.dev.yml exec desktop pnpm test
```

### Database Access

```bash
# Connect to PostgreSQL
docker compose -f docker-compose.dev.yml exec geo-db psql -U geotruth -d geotruth

# Run SQL file
docker compose -f docker-compose.dev.yml exec -T geo-db psql -U geotruth -d geotruth < migration.sql
```

---

## 🧪 Testing

### Backend Tests

```bash
# Run all tests
docker compose -f docker-compose.dev.yml exec api pytest

# With coverage
docker compose -f docker-compose.dev.yml exec api pytest --cov=app --cov-report=html

# Specific test
docker compose -f docker-compose.dev.yml exec api pytest tests/test_enrich.py -v
```

### Desktop Tests

```bash
# Rust tests
docker compose -f docker-compose.dev.yml exec desktop cargo test

# Frontend tests
docker compose -f docker-compose.dev.yml exec desktop pnpm test
```

### E2E Tests

```bash
# Run E2E test suite
docker compose -f docker-compose.test.yml up --abort-on-container-exit
```

---

## 📝 Code Standards

### Formatting

All formatting is enforced via pre-commit hooks (run in Docker):

```bash
# Format all code
docker compose -f docker-compose.dev.yml exec api ruff format .
docker compose -f docker-compose.dev.yml exec api ruff check --fix .
docker compose -f docker-compose.dev.yml exec desktop cargo fmt
docker compose -f docker-compose.dev.yml exec desktop pnpm format
```

### Linting

```bash
# Python
docker compose -f docker-compose.dev.yml exec api ruff check .
docker compose -f docker-compose.dev.yml exec api mypy app

# Rust
docker compose -f docker-compose.dev.yml exec desktop cargo clippy

# TypeScript
docker compose -f docker-compose.dev.yml exec desktop pnpm lint
```

---

## 🔄 Git Workflow

### Branching

```
main                 # Production-ready
├── develop          # Integration
├── feature/xxx      # New features
├── fix/xxx          # Bug fixes
└── docs/xxx         # Documentation
```

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(api): add batch enrichment endpoint
fix(desktop): correct GPS offset calculation
docs(api): update authentication examples
```

### Pre-Commit Hooks

Hooks run in Docker, so no local tools needed:

```yaml
# .pre-commit-config.yaml
repos:
  - repo: local
    hooks:
      - id: format-python
        name: Format Python
        entry: docker compose -f docker-compose.dev.yml exec -T api ruff format .
        language: system
        files: \.py$
```

---

## 📦 Building Releases

### Desktop App

```bash
# Build for current platform
docker compose -f docker-compose.dev.yml exec desktop pnpm tauri build

# Build outputs in desktop/target/release/bundle/
```

### Docker Images

```bash
# Build production images
docker compose -f backend/docker-compose.yml build

# Tag and push
docker tag geotruth/api:latest registry.example.com/geotruth/api:v1.0.0
docker push registry.example.com/geotruth/api:v1.0.0
```

---

## 🐛 Debugging

### View Logs

```bash
# All services
docker compose -f docker-compose.dev.yml logs -f

# Filter by level (requires jq)
docker compose -f docker-compose.dev.yml logs api --no-log-prefix | jq 'select(.level == "ERROR")'
```

### Attach Debugger

**Python (VS Code):**
```json
{
  "name": "Python: Remote Attach",
  "type": "debugpy",
  "request": "attach",
  "connect": {"host": "localhost", "port": 5678}
}
```

**Rust:** Use `println!` or `tracing` (debugger attachment complex in containers)

### Database Inspection

```bash
# List tables
docker compose -f docker-compose.dev.yml exec geo-db psql -U geotruth -d geotruth -c "\dt"

# Query POIs
docker compose -f docker-compose.dev.yml exec geo-db psql -U geotruth -d geotruth -c "SELECT * FROM pois LIMIT 5"
```

---

## 📊 Latest Package Versions

All dependencies are pinned to latest stable versions:

### Backend (Python 3.12)

| Package | Version |
|---------|---------|
| fastapi | 0.115+ |
| pydantic | 2.10+ |
| sqlalchemy | 2.0+ |
| redis | 5.2+ |

### Desktop (Rust 1.83)

| Crate | Version |
|-------|---------|
| tauri | 2.1+ |
| tokio | 1.42+ |
| duckdb | 1.1+ |
| tracing | 0.1+ |

### Frontend (Node 22)

| Package | Version |
|---------|---------|
| react | 19+ |
| vite | 6+ |
| typescript | 5.7+ |

### Update Policy

- **Weekly**: Dependabot PRs reviewed
- **Monthly**: Major version upgrades evaluated
- **Quarterly**: Full dependency audit

---

## 📚 Related Documentation

- [Backend Services](../backend/README.md)
- [Desktop Application](../desktop/README.md)
- [Logging Guide](../logging.md)
- [Architecture Overview](../architecture/README.md)
