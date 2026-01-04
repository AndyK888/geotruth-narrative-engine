# Desktop Application

The GeoTruth Desktop Application is a native, monolithic app that serves as the central hub for the Narrative Engine. It combines a high-performance Rust backend with a modern React frontend.

## Key Features

- **Native Performance**: Built with Rust and Tauri for minimal resource usage and maximum speed.
- **Embedded Database**: Uses DuckDB for high-speed, local SQL data persistence without external servers.
- **Geospatial Engine**: Native "GeoRust" implementation using PMTiles for vector map data.
- **AI Integration**:
  - **Narrative**: Google Gemini 2.0 Flash integration for generating location-aware stories.
  - **Processing**: Native integration with FFmpeg and Whisper for media processing.
- **Privacy-First**: All data stays local by default, except for anonymized AI requests (if enabled).

## Architecture

The application is structured as a Monolith:

```
┌───────────────────────────────────────────────┐
│              Tauri Application                │
│                                               │
│  ┌───────────────┐     ┌───────────────────┐  │
│  │ React Frontend│ <-> │   Rust Backend    │  │
│  └───────┬───────┘     └─────────┬─────────┘  │
│          │                       │            │
│          ▼                       ▼            │
│     [User UI]           [DuckDB + DashMap]   │
│                         [GeoEngine + AI]      │
└───────────────────────────────────────────────┘
```

- **Frontend**: React 19, Vite, TypeScript. Communicates with backend via Tauri Commands (`invoke`).
- **Backend**: Rust. Handles database, heavy computation, file I/O, and external API calls.

## Configuration

Configuration is handled via `tauri.conf.json` and local environment variables.
API Keys (like `GEMINI_API_KEY`) are stored securely or read from the environment.

## Data Storage

- **Database**: `~/.data/com.geotruth.app/geotruth.duckdb`
- **Map Tiles**: `~/.data/com.geotruth.app/tiles/*.pmtiles`
- **Logs**: Standard system logs or configured log directory.
