# GeoTruth Narrative Engine

> **Turn raw travel footage into fact-checked, AI-narrated stories.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
[![React](https://img.shields.io/badge/React-18-61dafb.svg)](https://reactjs.org/)

---

## 📖 Executive Summary

**GeoTruth** is a desktop application designed for travelers, dashcam users, and content creators who have hours of footage but no time to edit it.

Unlike standard AI tools that "guess" locations and hallucinate facts, GeoTruth uses a **"Verify First, Narrate Second"** architecture. It mathematically synchronizes your video with GPS data, validates visible landmarks using geospatial databases, and *only then* allows AI to write a script based on that proven evidence.

**The Result:** An automated travel log, YouTube chapters, and voiceover script that is 100% geographically accurate—processing gigabytes of video locally while keeping your private footage off the cloud.

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

---

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18.x or higher
- **Rust** 1.70 or higher
- **Docker** and **Docker Compose** (for backend services)
- **FFmpeg** installed locally

### Installation

```bash
# Clone the repository
git clone https://github.com/your-org/geotruth-narrative-engine.git
cd geotruth-narrative-engine

# Install desktop dependencies
cd desktop
npm install

# Start development
npm run tauri dev
```

### Backend Setup

```bash
# Start the geo-stack
cd backend
docker-compose up -d

# Initialize the database
./scripts/init-db.sh
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Architecture Overview](docs/architecture/README.md) | System design and component interactions |
| [Desktop App Guide](docs/desktop/README.md) | Tauri application development |
| [Backend Services](docs/backend/README.md) | API and geospatial services |
| [User Guide](docs/user-guide/README.md) | End-user documentation |
| [API Reference](docs/api/README.md) | REST API documentation |
| [Development Guide](docs/development/README.md) | Contributing and local development |

---

## 🏗️ Project Structure

```
/geotruth-narrative-engine
├── /backend                # Cloud Geo-Intelligence Layer
│   ├── /app                # FastAPI application
│   ├── /docker             # Dockerfiles for services
│   ├── /scripts            # Database migration/init scripts
│   └── docker-compose.yml  # Service orchestration
├── /desktop                # Tauri Desktop Application
│   ├── /src-tauri          # Rust backend (core logic)
│   ├── /src                # React frontend
│   └── /binaries           # Sidecars (ffmpeg, whisper)
├── /docs                   # Comprehensive documentation
└── /shared                 # Shared types and schemas
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

| Layer | Technology |
|-------|------------|
| **Desktop App** | Tauri v2 (Rust + React) |
| **Local Processing** | FFmpeg, Whisper.cpp |
| **Local Database** | DuckDB |
| **Cloud Database** | PostGIS |
| **Map Matching** | Valhalla / OSRM |
| **Caching** | Redis |
| **API Server** | FastAPI (Python) |
| **AI Engine** | Google Gemini |

---

## 🔐 Security

- **API keys** stored in OS native keychain
- **JWT-based** authentication
- **Environment-based** configuration management
- **No video data** sent to cloud servers

See [Security Documentation](docs/security/README.md) for details.

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- [Tauri](https://tauri.app/) - Desktop application framework
- [OpenStreetMap](https://www.openstreetmap.org/) - Geospatial data
- [Valhalla](https://valhalla.github.io/valhalla/) - Routing engine
- [Whisper](https://github.com/openai/whisper) - Speech recognition

---

<p align="center">Made with ❤️ for travelers and content creators</p>
