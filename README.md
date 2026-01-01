# GeoTruth Narrative Engine

> **Turn raw travel footage into fact-checked, AI-narrated stories.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
[![Docker](https://img.shields.io/badge/Docker-24+-blue.svg)](https://docker.com/)

---

## 📖 Executive Summary

**GeoTruth** is a desktop application designed for travelers, dashcam users, and content creators who have hours of footage but no time to edit it.

Unlike standard AI tools that "guess" locations and hallucinate facts, GeoTruth uses a **"Verify First, Narrate Second"** architecture. It mathematically synchronizes your video with GPS data, validates visible landmarks using geospatial databases, and *only then* allows AI to write a script based on that proven evidence.

**The Result:** An automated travel log, YouTube chapters, and voiceover script that is 100% geographically accurate—processing gigabytes of video locally while keeping your private footage off the cloud.

---

## 🚀 Zero Local Dependencies

GeoTruth requires **no local software installation** besides the app itself:

| Component | Deployment |
|-----------|------------|
| **Desktop App** | Self-contained bundle with all binaries included |
| **Backend Services** | 100% Docker Compose - single command startup |
| **Development** | Docker-based - no local Python/Rust/Node required |

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| 🎥 **Universal Format Support** | GoPro, Insta360, Dashcams, and standard cameras |
| 🔄 **Zero-Shot Time Sync** | Automatic video-GPS alignment via OCR or audio/motion spikes |
| 🔒 **Privacy-First** | All video processing happens locally on your machine |
| 🗺️ **Truth Engine** | Map matching, field-of-view filtering, and visual verification |
| 🤖 **Fact-Checked AI** | Narration grounded in verified geospatial data |
| ✏️ **Human-in-the-Loop** | Review and correct detections before script generation |
| 📊 **Detailed Logging** | Structured JSON logs with correlation IDs for debugging |

---

## 🚀 Quick Start

### Option 1: Desktop Application (End Users)

1. **Download** GeoTruth from [geotruth.io/download](https://geotruth.io/download)
2. **Install** - Double-click to install (all dependencies bundled)
3. **Run** - No additional setup required

### Option 2: Backend Services (Docker)

```bash
# Clone the repository
git clone https://github.com/your-org/geotruth-narrative-engine.git
cd geotruth-narrative-engine/backend

# Start all services (first run downloads images)
docker compose up -d

# Check status
docker compose ps

# View logs
docker compose logs -f
```

**That's it.** No Python, no Node, no Rust installation needed.

### Option 3: Full Development (Docker)

```bash
# Clone and start development environment
git clone https://github.com/your-org/geotruth-narrative-engine.git
cd geotruth-narrative-engine

# Start everything in Docker
docker compose -f docker-compose.dev.yml up -d

# Development server available at http://localhost:5173
# API available at http://localhost:8000
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Architecture Overview](docs/architecture/README.md) | System design and component interactions |
| [Desktop App Guide](docs/desktop/README.md) | Self-contained desktop application |
| [Backend Services](docs/backend/README.md) | Docker-based API and geo services |
| [Logging Guide](docs/logging.md) | Structured logging and debugging |
| [User Guide](docs/user-guide/README.md) | End-user documentation |
| [API Reference](docs/api/README.md) | REST API documentation |
| [Development Guide](docs/development/README.md) | Docker-based development workflow |

---

## 🏗️ Project Structure

```
/geotruth-narrative-engine
├── /backend                    # Docker-based backend services
│   ├── /services
│   │   ├── /api                # FastAPI application (Python 3.12)
│   │   ├── /geo-db             # PostGIS database
│   │   ├── /map-matcher        # Valhalla routing engine
│   │   └── /cache              # Redis caching
│   ├── docker-compose.yml      # Production orchestration
│   └── docker-compose.dev.yml  # Development with hot reload
├── /desktop                    # Tauri Desktop Application
│   ├── /src-tauri              # Rust backend (bundled)
│   ├── /src                    # React frontend (bundled)
│   └── /binaries               # FFmpeg, Whisper (bundled)
├── /docs                       # Documentation
└── docker-compose.dev.yml      # Full-stack development
```

---

## 🎯 Target Use Cases

### Road Trippers
Convert a 10-hour hyperlapse of a cross-country drive into a video with chapters marking every major town, state line, and landmark passed.

### Dashcam Archivists
Automatically catalog "eventful" drives (stops, turns, scenic routes) without scrubbing through hours of empty highway footage.

### Zoo/Park Visitors
Create educational timelines of walking tours where the narration correctly identifies animals and exhibits based on visual and location data.

---

## 🛠️ Technology Stack

| Layer | Technology | Version |
|-------|------------|---------|
| **Desktop App** | Tauri v2 (Rust + React) | Latest |
| **Frontend** | React 19 + Vite 6 | Latest |
| **Backend Runtime** | Python 3.12 | Latest |
| **API Framework** | FastAPI 0.115+ | Latest |
| **Database** | PostgreSQL 17 + PostGIS 3.5 | Latest |
| **Map Matching** | Valhalla 3.5 | Latest |
| **Caching** | Redis 7.4 | Latest |
| **AI Engine** | Google Gemini 2.0 | Latest |
| **Local Processing** | FFmpeg 7, Whisper.cpp | Latest |

> **Update Policy**: All dependencies are pinned to latest stable versions. Automated weekly checks via Dependabot.

---

## 📊 Observability

GeoTruth includes comprehensive structured logging:

```json
{
  "timestamp": "2024-01-15T10:30:00.123Z",
  "level": "INFO",
  "service": "api",
  "correlation_id": "req-abc123",
  "message": "Enrichment completed",
  "context": {
    "lat": 36.1069,
    "lon": -112.1129,
    "pois_found": 3,
    "duration_ms": 45
  }
}
```

See [Logging Guide](docs/logging.md) for details.

---

## 🔐 Security

- **API keys** stored in OS native keychain (desktop) or Docker secrets (backend)
- **JWT-based** authentication
- **Isolated container networks** for service protection
- **No video data** sent to cloud servers

See [Security Documentation](docs/security/README.md) for details.

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

Development is 100% Docker-based - no local toolchain required.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<p align="center">Made with ❤️ for travelers and content creators</p>
