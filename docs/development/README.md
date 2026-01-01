# Development Guide

This guide covers setting up your development environment, coding standards, and contribution workflow for GeoTruth.

---

## 🛠️ Development Environment

### Prerequisites

| Tool | Version | Purpose |
|------|---------|---------|
| **Node.js** | 18.x+ | Frontend tooling |
| **Rust** | 1.70+ | Desktop backend |
| **Python** | 3.11+ | Backend services |
| **Docker** | 24.x+ | Container runtime |
| **Docker Compose** | 2.x+ | Service orchestration |

### System Dependencies

#### macOS

```bash
# Install Xcode command line tools
xcode-select --install

# Install Homebrew packages
brew install node rust python@3.11 docker ffmpeg

# Install Tauri dependencies
brew install cairo pango
```

#### Ubuntu/Debian

```bash
# System packages
sudo apt update
sudo apt install -y \
    build-essential \
    curl \
    wget \
    libssl-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    librsvg2-dev \
    ffmpeg

# Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt install -y nodejs

# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

#### Windows

```powershell
# Install Chocolatey first, then:
choco install nodejs-lts rust python docker-desktop ffmpeg

# Visual Studio Build Tools (required for some Rust crates)
choco install visualstudio2022buildtools
```

---

## 📂 Repository Setup

### Clone and Initialize

```bash
# Clone the repository
git clone https://github.com/your-org/geotruth-narrative-engine.git
cd geotruth-narrative-engine

# Install global tools
npm install -g pnpm
cargo install tauri-cli

# Set up all workspaces
./scripts/setup-dev.sh
```

### Project Structure

```
geotruth-narrative-engine/
├── backend/               # Python backend services
│   ├── app/               # FastAPI application
│   ├── tests/             # Backend tests
│   └── docker-compose.yml
├── desktop/               # Tauri desktop app
│   ├── src/               # React frontend
│   ├── src-tauri/         # Rust backend
│   └── binaries/          # Sidecar binaries
├── docs/                  # Documentation
├── scripts/               # Development scripts
├── .github/               # GitHub workflows
└── package.json           # Root package for scripts
```

---

## 🖥️ Desktop Development

### Running the App

```bash
cd desktop

# Install dependencies
pnpm install

# Start development (hot reload enabled)
pnpm tauri dev
```

### Frontend Only (React)

```bash
# Start Vite dev server without Tauri
pnpm dev

# Access at http://localhost:5173
```

### Backend Only (Rust)

```bash
cd desktop/src-tauri

# Build and run
cargo run

# Run tests
cargo test

# Check for issues
cargo clippy
```

### Building for Release

```bash
cd desktop

# Build optimized bundle
pnpm tauri build

# Outputs:
# - macOS: target/release/bundle/dmg/GeoTruth_x.y.z_aarch64.dmg
# - Windows: target/release/bundle/msi/GeoTruth_x.y.z_x64_en-US.msi
# - Linux: target/release/bundle/appimage/GeoTruth_x.y.z_amd64.AppImage
```

---

## ☁️ Backend Development

### Local Setup

```bash
cd backend

# Create virtual environment
python -m venv venv
source venv/bin/activate  # or `venv\Scripts\activate` on Windows

# Install dependencies
pip install -r requirements.txt -r requirements-dev.txt

# Copy environment template
cp .env.example .env
# Edit .env with your API keys
```

### Running Services

```bash
# Start all services (recommended)
docker-compose up -d

# Start API server only (for debugging)
uvicorn app.main:app --reload --port 8000

# View logs
docker-compose logs -f api
```

### Database Management

```bash
# Run migrations
docker-compose exec api alembic upgrade head

# Create new migration
docker-compose exec api alembic revision -m "description"

# Seed POI data
docker-compose exec api python scripts/seed-pois.py
```

### API Testing

```bash
# Run tests
pytest tests/ -v

# With coverage
pytest tests/ --cov=app --cov-report=html

# Run specific test
pytest tests/test_enrich.py::test_single_point -v
```

---

## 📝 Coding Standards

### TypeScript/React

```typescript
// Use functional components with TypeScript
interface EventCardProps {
  event: Event;
  onSelect: (id: string) => void;
}

export function EventCard({ event, onSelect }: EventCardProps) {
  return (
    <div 
      className="event-card"
      onClick={() => onSelect(event.id)}
    >
      <h3>{event.name}</h3>
      <span>{event.timestamp}</span>
    </div>
  );
}
```

**Guidelines:**
- Use TypeScript strict mode
- Prefer functional components with hooks
- Use TanStack Query for data fetching
- Keep components small and focused

### Rust

```rust
/// Process a video file and extract metadata.
/// 
/// # Arguments
/// * `path` - Path to the video file
/// 
/// # Returns
/// * `Result<VideoInfo, ProcessError>` - Video metadata or error
/// 
/// # Example
/// ```
/// let info = process_video("/path/to/video.mp4").await?;
/// println!("Duration: {} seconds", info.duration_secs);
/// ```
pub async fn process_video(path: &str) -> Result<VideoInfo, ProcessError> {
    let sidecar = SidecarRunner::new();
    sidecar.get_video_info(path).await
}
```

**Guidelines:**
- Use `Result<T, E>` for error handling
- Document public functions with `///` comments
- Run `cargo fmt` before committing
- Run `cargo clippy` and fix warnings

### Python

```python
from typing import Optional
from pydantic import BaseModel

class EnrichRequest(BaseModel):
    """Request model for GPS enrichment."""
    
    lat: float
    lon: float
    timestamp: Optional[str] = None
    heading_deg: Optional[float] = None
    
    class Config:
        json_schema_extra = {
            "example": {
                "lat": 36.1069,
                "lon": -112.1129,
                "timestamp": "2024-01-15T10:30:00Z"
            }
        }


async def enrich_point(request: EnrichRequest) -> EnrichResponse:
    """
    Enrich a GPS point with geospatial context.
    
    Args:
        request: The enrichment request containing coordinates.
        
    Returns:
        EnrichResponse with location context and nearby POIs.
        
    Raises:
        ValueError: If coordinates are out of bounds.
    """
    # Implementation
    pass
```

**Guidelines:**
- Use type hints everywhere
- Use Pydantic for request/response models
- Format with `black` and `isort`
- Check with `ruff` linter

---

## 🧪 Testing

### Frontend Tests

```bash
cd desktop

# Run all tests
pnpm test

# Watch mode
pnpm test:watch

# Coverage report
pnpm test:coverage
```

### Rust Tests

```bash
cd desktop/src-tauri

# All tests
cargo test

# Specific test
cargo test process_video

# With output
cargo test -- --nocapture
```

### Backend Tests

```bash
cd backend

# All tests
pytest

# Specific file
pytest tests/test_enrich.py

# Specific test
pytest tests/test_enrich.py::test_batch_enrichment

# With coverage
pytest --cov=app --cov-report=term-missing
```

### End-to-End Tests

```bash
# Start all services
docker-compose up -d

# Run E2E tests
pnpm test:e2e
```

---

## 🔄 Git Workflow

### Branching

```
main                 # Production-ready code
├── develop          # Integration branch
├── feature/xxx      # New features
├── fix/xxx          # Bug fixes
└── docs/xxx         # Documentation updates
```

### Commit Messages

Use [Conventional Commits](https://www.conventionalcommits.org/):

```
feat(desktop): add timeline zoom controls
fix(backend): correct POI distance calculation
docs(api): update authentication examples
refactor(rust): simplify sidecar error handling
test(enrich): add batch processing tests
chore: update dependencies
```

### Pull Request Process

1. **Create branch** from `develop`
   ```bash
   git checkout develop
   git pull origin develop
   git checkout -b feature/my-feature
   ```

2. **Make changes** with tests

3. **Run checks**
   ```bash
   pnpm lint
   pnpm test
   cargo clippy
   cargo test
   pytest
   ```

4. **Push and create PR**
   ```bash
   git push origin feature/my-feature
   ```

5. **Fill PR template** with:
   - Description of changes
   - Screenshots (if UI)
   - Test coverage
   - Breaking changes (if any)

---

## 🏗️ Architecture Decisions

### ADR Template

Create new ADRs in `docs/adr/`:

```markdown
# ADR-001: Use DuckDB for Local Storage

## Status
Accepted

## Context
We need a local database for storing project data and GPS points.

## Decision
Use DuckDB instead of SQLite because:
- Better performance for analytical queries
- Native support for JSON types
- Excellent Rust bindings

## Consequences
- Learning curve for team members unfamiliar with DuckDB
- Need to bundle DuckDB library with the app
```

---

## 🔧 Debugging

### Desktop App

**React DevTools:**
- Open with `Cmd/Ctrl+Shift+I`
- React tab for component inspection
- Network tab for API calls

**Rust Debugging:**
```bash
# Enable debug logging
RUST_LOG=debug pnpm tauri dev

# Use VS Code with CodeLLDB extension for breakpoints
```

### Backend

**FastAPI Debug:**
```bash
# Run with auto-reload and debug
uvicorn app.main:app --reload --log-level debug

# API docs at http://localhost:8000/docs
```

**Database Queries:**
```bash
# Connect to PostGIS
docker-compose exec geo-db psql -U geotruth -d geotruth

# View slow queries
SELECT query, calls, mean_time 
FROM pg_stat_statements 
ORDER BY mean_time DESC 
LIMIT 10;
```

---

## 📚 Resources

### Documentation

- [Tauri v2 Guides](https://v2.tauri.app/guides/)
- [FastAPI Tutorial](https://fastapi.tiangolo.com/tutorial/)
- [PostGIS Documentation](https://postgis.net/docs/)
- [DuckDB Rust API](https://duckdb.org/docs/api/rust)

### Internal Docs

- [Architecture Overview](../architecture/README.md)
- [API Reference](../api/README.md)
- [Security Guidelines](../security/README.md)

---

## 🤝 Getting Help

- **Discord**: [#dev-help channel](https://discord.gg/geotruth)
- **GitHub Issues**: For bugs and features
- **Team Meetings**: Every Tuesday 10am PT
