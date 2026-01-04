# GeoTruth Narrative Engine

> **Turn raw travel footage into fact-checked, AI-narrated stories.**

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Tauri](https://img.shields.io/badge/Tauri-v2-blue.svg)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.77+-orange.svg)](https://www.rust-lang.org/)
[![DuckDB](https://img.shields.io/badge/DuckDB-1.0+-yellow.svg)](https://duckdb.org/)

---

## 📖 Executive Summary

**GeoTruth** is a native desktop application designed for travelers, dashcam users, and content creators who have hours of footage but no time to edit it.

Unlike standard AI tools that "guess" locations and hallucinate facts, GeoTruth uses a **"Verify First, Narrate Second"** architecture. It mathematically synchronizes your video with GPS data, validates visible landmarks using embedded geospatial databases, and *only then* allows AI to write a script based on that proven evidence.

**The Result:** An automated travel log, locally processed to respect your privacy, merging high-performance Rust processing with cloud-based AI reasoning.

---

## 🚀 Native Monolith

GeoTruth is a single binary application:

| Component | Deployment |
|-----------|------------|
| **Desktop App** | Self-contained Tauri bundle (Rust + React) |
| **Database** | Embedded DuckDB (No external server required) |
| **Logic** | Native Rust (No Python/Node backend required) |

---

## ✨ Key Features

| Feature | Description |
|---------|-------------|
| 🎥 **Universal Format Support** | GoPro, Insta360, Dashcams, and standard cameras |
| 🔄 **Zero-Shot Time Sync** | Automatic video-GPS alignment via OCR or audio/motion spikes |
| 🔒 **Privacy-First** | All video processing happens locally on your machine |
| 🗺️ **Truth Engine** | Local map matching and visual verification via DuckDB/GeoRust |
| 🤖 **Fact-Checked AI** | Narration grounded in verified geospatial data (Gemini) |
| ✏️ **Human-in-the-Loop** | Review and correct detections before script generation |
| ⚡ **Hardware Accelerated** | Native FFmpeg/GPU utilization for transcoding and inference |

---

## 🚀 Quick Start

### Option 1: Desktop Application (End Users)

1. **Download** GeoTruth from [geotruth.io/download](https://geotruth.io/download)
2. **Install** - Double-click to install.
3. **Run** - No additional setup required.

### Option 2: Development (Local)

Prerequisites: Rust, Node.js, pnpm.

```bash
# Clone the repository
git clone https://github.com/your-org/geotruth-narrative-engine.git
cd geotruth-narrative-engine

# Install dependencies
pnpm install

# Run Development Mode
pnpm tauri dev
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [Architecture Overview](docs/architecture/README.md) | System design and monolith structure |
| [Desktop App Guide](docs/desktop/README.md) | Application usage and features |
| [Development Guide](docs/development/README.md) | Setting up the local dev environment |
| [User Guide](docs/user-guide/README.md) | End-user documentation |

---

## 🏗️ Project Structure

```
/geotruth-narrative-engine
├── /desktop                    # Monorepo Root
│   ├── /src-tauri              # Rust Backbone (Logic, DB, API)
│   ├── /src                    # React Frontend (UI)
│   └── /binaries               # Bundled utilities (FFmpeg sidecars)
├── /docs                       # Documentation
└── /locales                    # I18n translations
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
| **Core** | Tauri v2 |
| **Frontend** | React 19 + Vite 6 |
| **Backend Logic** | Rust (Tokio, Reqwest) |
| **Database** | DuckDB (Embedded, Spatial) |
| **State Management** | DashMap (In-Memory) |
| **Geospatial** | GeoRust (geo, geozero), PMTiles |
| **AI Engine** | Google Gemini 2.0 (Cloud) |
| **Media Processing** | Native FFmpeg |

---

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

---

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<p align="center">Made with ❤️ for travelers and content creators</p>
