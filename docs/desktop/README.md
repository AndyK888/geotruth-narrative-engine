# Desktop Application

The GeoTruth desktop application is a **self-contained bundle** with zero local dependencies. It features a robust **Hybrid Architecture** that works both with a powerful Docker backend and fully offline.

---

## 🎯 Zero Local Dependencies

When users download GeoTruth, they get everything they need:

```
┌─────────────────────────────────────────────────────────────────┐
│                   GeoTruth.app (Self-Contained)                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                    Tauri Runtime                            │ │
│  │  ┌──────────────────────┐  ┌────────────────────────────┐  │ │
│  │  │   React Frontend     │  │      Rust Backend          │  │ │
│  │  │   (Bundled Vite)     │  │   (Native Binary)          │  │ │
│  │  └──────────────────────┘  └──────┬─────────────────────┘  │ │
│  └───────────────────────────────────┼────────────────────────┘ │
│                                      │                           │
│  ┌───────────────────────────────────▼────────────────────────┐ │
│  │                   Bundled Binaries & Libraries              │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │ │
│  │  │  FFmpeg  │ │ Whisper  │ │ Llama.cpp│ │ PMTiles      │   │ │
│  │  │   7.x    │ │   cpp    │ │ (Offline)│ │ Reader       │   │ │
│  │  └──────────┘ └──────────┘ └──────────┘ └──────────────┘   │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                   Embedded Database                         │ │
│  │  ┌─────────────────────────────────────────────────────┐   │ │
│  │  │                     DuckDB                           │   │ │
│  │  │              (Compiled into Rust)                    │   │ │
│  │  └─────────────────────────────────────────────────────┘   │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

**Users install the app. That's it. No additional software required.**

---

## 📶 Offline Mode (Hybrid Architecture)

GeoTruth is designed for the "Standalone Traveler."

| Feature | Online Mode (Connected) | Offline Mode (Disconnected) |
|---------|-------------------------|-----------------------------|
| **Verification** | Docker/Cloud API (Valhalla + PostGIS) | Local PMTiles (Vector Map Packs) |
| **Accuracy** | High (Full street network + POI db) | Medium (Downloaded region data) |
| **Narration** | Google Gemini 2.0 (Fast, Creative) | Llama.cpp (Local, Private) |
| **Data Usage** | GPS Traces sent to API | 0 bytes sent |

### Data Manager
The app includes a **Data Manager** to download offline resources:
- **Map Packs**: Download regions (e.g., "California", "France") for offline verification.
- **AI Models**: Download quantized LLMs for offline narration.

---

## 📁 Directory Structure

```
/desktop
├── /src                          # React Frontend
│   ├── /components               # UI components
├── /src-tauri                    # Rust Backend
│   ├── /src
│   │   ├── /services
│   │   │   ├── /ai
│   │   │   │   ├── online.rs     # Gemini Client
│   │   │   │   └── offline.rs    # Llama.cpp Wrapper
│   │   │   ├── /geo
│   │   │   │   ├── online.rs     # API Client
│   │   │   │   └── offline.rs    # PMTiles Reader
├── /binaries                     # Bundled sidecars
├── /offline_data                 # User downloaded maps/models
├── package.json
└── vite.config.ts
```

---

## 🐳 Development (Docker-Based)

No local Rust or Node.js installation required. Everything runs in Docker.

### Start Development Environment

```bash
cd desktop

# Start dev environment (first run downloads dependencies)
docker compose -f docker-compose.dev.yml up

# Access development server at http://localhost:5173
# Changes hot-reload automatically
```

---

## 📦 Bundled Binaries

All processing binaries are bundled with the app and executed as sidecars.

| Binary | Version | Purpose |
|--------|---------|---------|
| **FFmpeg** | 7.1 | Video processing |
| **Whisper.cpp** | 1.7.2 | Audio transcription |
| **Tesseract** | 5.4 | OCR for timestamps |
| **Llama.cpp** | b2xxx | Offline AI Narration |

---

## 📊 Structured Logging

The desktop app uses comprehensive structured logging for debugging. See [Logging Guide](../logging.md).

```json
{"timestamp":"...","level":"INFO","message":"Switched to Offline Mode","correlation_id":"abc-123"}
```

---

## 📚 Related Documentation

- [Architecture Overview](../architecture/README.md)
- [Local Intelligence Layer](../architecture/local-intelligence.md)
- [Logging Guide](../logging.md)
- [User Guide](../user-guide/README.md)
