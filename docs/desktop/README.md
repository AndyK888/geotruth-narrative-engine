# Desktop Application

The GeoTruth desktop application is a **self-contained bundle** with zero local dependencies. All binaries (FFmpeg, Whisper, etc.) are bundled inside the app.

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
│  │  └──────────────────────┘  └────────────────────────────┘  │ │
│  └────────────────────────────────────────────────────────────┘ │
│                                                                  │
│  ┌────────────────────────────────────────────────────────────┐ │
│  │                   Bundled Binaries                          │ │
│  │  ┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────────┐   │ │
│  │  │  FFmpeg  │ │ FFprobe  │ │ Whisper  │ │ Tesseract    │   │ │
│  │  │   7.x    │ │   7.x    │ │  cpp     │ │   5.x        │   │ │
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

## 📁 Directory Structure

```
/desktop
├── /src                          # React Frontend
│   ├── /components               # UI components
│   ├── /hooks                    # React hooks
│   ├── /stores                   # Zustand state
│   ├── /lib                      # Utilities
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── /src-tauri                    # Rust Backend
│   ├── /src
│   │   ├── main.rs               # Entry point
│   │   ├── lib.rs                # Library
│   │   ├── logging.rs            # Structured logging
│   │   ├── /commands             # Tauri commands
│   │   ├── /services             # Business logic
│   │   ├── /models               # Data models
│   │   └── /utils                # Utilities
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── /binaries                     # Bundled sidecars
│   ├── /darwin-aarch64           # macOS ARM
│   ├── /darwin-x86_64            # macOS Intel
│   ├── /windows-x86_64           # Windows
│   └── /linux-x86_64             # Linux
├── /scripts
│   └── download-binaries.sh      # Binary download script
├── package.json
├── vite.config.ts
├── Dockerfile.dev                # Dev environment
└── docker-compose.dev.yml        # Dev orchestration
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

### docker-compose.dev.yml

```yaml
version: '3.9'

services:
  dev:
    build:
      context: .
      dockerfile: Dockerfile.dev
    volumes:
      # Source code for hot reload
      - ./src:/app/src:cached
      - ./src-tauri/src:/app/src-tauri/src:cached
      # Persist build artifacts
      - cargo-cache:/usr/local/cargo/registry
      - target-cache:/app/src-tauri/target
      - node-modules:/app/node_modules
    ports:
      - "5173:5173"  # Vite dev server
    environment:
      - RUST_LOG=debug
      - RUST_BACKTRACE=1
    tty: true
    stdin_open: true

volumes:
  cargo-cache:
  target-cache:
  node-modules:
```

### Dockerfile.dev

```dockerfile
FROM rust:1.83-slim

# Install system dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    # Node.js
    curl \
    # Tauri dependencies
    libwebkit2gtk-4.1-dev \
    libgtk-3-dev \
    librsvg2-dev \
    libayatana-appindicator3-dev \
    # Build tools
    build-essential \
    pkg-config \
    libssl-dev \
    # FFmpeg for testing
    ffmpeg \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 22 LTS
RUN curl -fsSL https://deb.nodesource.com/setup_22.x | bash - \
    && apt-get install -y nodejs \
    && npm install -g pnpm

# Install Tauri CLI
RUN cargo install tauri-cli

WORKDIR /app

# Copy package files
COPY package.json pnpm-lock.yaml ./
RUN pnpm install

# Copy Rust files
COPY src-tauri/Cargo.toml src-tauri/Cargo.lock ./src-tauri/
RUN cd src-tauri && cargo fetch

# Copy source
COPY . .

# Development command
CMD ["pnpm", "tauri", "dev"]
```

---

## 📦 Bundled Binaries

All processing binaries are bundled with the app and executed as sidecars.

### Binary Versions (Latest)

| Binary | Version | Purpose |
|--------|---------|---------|
| **FFmpeg** | 7.1 | Video processing |
| **FFprobe** | 7.1 | Metadata extraction |
| **Whisper.cpp** | 1.7.2 | Audio transcription |
| **Tesseract** | 5.4 | OCR for timestamps |

### Download Script

```bash
#!/bin/bash
# scripts/download-binaries.sh

set -e

BINARIES_DIR="binaries"
FFMPEG_VERSION="7.1"
WHISPER_VERSION="1.7.2"

# Detect platform
case "$(uname -s)-$(uname -m)" in
    Darwin-arm64)  PLATFORM="darwin-aarch64" ;;
    Darwin-x86_64) PLATFORM="darwin-x86_64" ;;
    Linux-x86_64)  PLATFORM="linux-x86_64" ;;
    MINGW*|MSYS*)  PLATFORM="windows-x86_64" ;;
    *) echo "Unsupported platform"; exit 1 ;;
esac

mkdir -p "$BINARIES_DIR/$PLATFORM"

echo "Downloading FFmpeg $FFMPEG_VERSION for $PLATFORM..."
# Platform-specific download URLs
# ... download logic

echo "Downloading Whisper.cpp $WHISPER_VERSION for $PLATFORM..."
# ... download logic

echo "All binaries downloaded to $BINARIES_DIR/$PLATFORM"
```

### Tauri Configuration

```json
// tauri.conf.json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "GeoTruth",
  "version": "1.0.0",
  "identifier": "com.geotruth.desktop",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "pnpm dev",
    "beforeBuildCommand": "pnpm build"
  },
  "app": {
    "windows": [
      {
        "title": "GeoTruth Narrative Engine",
        "width": 1400,
        "height": 900
      }
    ]
  },
  "bundle": {
    "active": true,
    "icon": ["icons/icon.icns", "icons/icon.ico", "icons/icon.png"],
    "externalBin": [
      "binaries/ffmpeg",
      "binaries/ffprobe",
      "binaries/whisper",
      "binaries/tesseract"
    ],
    "resources": ["resources/*"]
  }
}
```

---

## 📊 Structured Logging

The desktop app uses comprehensive structured logging for debugging.

### Rust Logging Configuration

```rust
// src-tauri/src/logging.rs

use std::fs::{self, OpenOptions};
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::Serialize;
use tracing::{Level, Subscriber};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};

#[derive(Serialize)]
struct LogEntry {
    timestamp: String,
    level: String,
    target: String,
    message: String,
    span: Option<String>,
    correlation_id: Option<String>,
    context: serde_json::Value,
}

pub struct LogConfig {
    pub log_dir: PathBuf,
    pub log_level: Level,
    pub json_output: bool,
    pub max_file_size_mb: u64,
    pub max_files: u32,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            log_dir: dirs::data_local_dir()
                .unwrap_or_default()
                .join("GeoTruth")
                .join("logs"),
            log_level: Level::INFO,
            json_output: true,
            max_file_size_mb: 50,
            max_files: 5,
        }
    }
}

pub fn init_logging(config: LogConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Ensure log directory exists
    fs::create_dir_all(&config.log_dir)?;
    
    let log_file = config.log_dir.join("geotruth.log");
    
    // File appender with rotation
    let file_appender = tracing_appender::rolling::Builder::new()
        .rotation(tracing_appender::rolling::Rotation::DAILY)
        .filename_prefix("geotruth")
        .filename_suffix("log")
        .max_log_files(config.max_files as usize)
        .build(&config.log_dir)?;
    
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);
    
    // Build subscriber
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(config.log_level.to_string()));
    
    if config.json_output {
        // JSON format for production
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .json()
                    .with_writer(non_blocking)
                    .with_span_events(FmtSpan::CLOSE)
                    .with_current_span(true)
                    .with_thread_ids(true)
                    .with_file(true)
                    .with_line_number(true)
            )
            .with(
                fmt::layer()
                    .pretty()
                    .with_writer(std::io::stderr)
            )
            .init();
    } else {
        // Pretty format for development
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .pretty()
                    .with_writer(std::io::stderr)
                    .with_span_events(FmtSpan::CLOSE)
            )
            .init();
    }
    
    tracing::info!(
        log_dir = %config.log_dir.display(),
        level = %config.log_level,
        "Logging initialized"
    );
    
    Ok(())
}
```

### Usage in Commands

```rust
// src-tauri/src/commands/ingest.rs

use tracing::{info, warn, error, instrument, Span};
use uuid::Uuid;

#[tauri::command]
#[instrument(
    name = "import_video",
    skip(db),
    fields(
        correlation_id = %Uuid::new_v4(),
        video_path = %video_path,
    )
)]
pub async fn import_video(
    db: State<'_, Database>,
    project_id: String,
    video_path: String,
) -> Result<VideoInfo, String> {
    info!("Starting video import");
    
    // Validate file exists
    let path = std::path::Path::new(&video_path);
    if !path.exists() {
        error!(path = %video_path, "Video file not found");
        return Err("Video file not found".to_string());
    }
    
    // Extract metadata
    info!("Extracting metadata with FFprobe");
    let metadata = match extract_metadata(&video_path).await {
        Ok(meta) => {
            info!(
                duration_secs = meta.duration_secs,
                width = meta.width,
                height = meta.height,
                codec = %meta.codec,
                "Metadata extracted successfully"
            );
            meta
        }
        Err(e) => {
            error!(error = %e, "Failed to extract metadata");
            return Err(format!("Metadata extraction failed: {}", e));
        }
    };
    
    // Store in database
    info!("Storing video reference in database");
    match db.store_video(&project_id, &video_path, &metadata).await {
        Ok(video_info) => {
            info!(video_id = %video_info.id, "Video imported successfully");
            Ok(video_info)
        }
        Err(e) => {
            error!(error = %e, "Database storage failed");
            Err(format!("Failed to store video: {}", e))
        }
    }
}

#[instrument(skip_all, fields(video_path = %path))]
async fn extract_metadata(path: &str) -> Result<VideoMetadata, ProcessError> {
    info!("Spawning FFprobe sidecar");
    
    let start = std::time::Instant::now();
    
    let output = tauri::api::process::Command::new_sidecar("ffprobe")
        .map_err(|e| {
            error!(error = %e, "Failed to create FFprobe sidecar");
            ProcessError::SidecarNotFound
        })?
        .args([
            "-v", "quiet",
            "-print_format", "json",
            "-show_format",
            "-show_streams",
            path,
        ])
        .output()
        .await
        .map_err(|e| {
            error!(error = %e, "FFprobe execution failed");
            ProcessError::ExecutionFailed(e.to_string())
        })?;
    
    let duration = start.elapsed();
    info!(duration_ms = duration.as_millis(), "FFprobe completed");
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        warn!(stderr = %stderr, "FFprobe returned non-zero exit code");
    }
    
    let metadata: FFprobeOutput = serde_json::from_slice(&output.stdout)
        .map_err(|e| {
            error!(error = %e, "Failed to parse FFprobe output");
            ProcessError::ParseError(e.to_string())
        })?;
    
    Ok(metadata.into())
}
```

### Log Output Examples

**File Output (JSON):**
```json
{"timestamp":"2024-01-15T10:30:00.123456Z","level":"INFO","target":"geotruth::commands::ingest","message":"Starting video import","correlation_id":"abc12345-6789-def0-1234-567890abcdef","video_path":"/Users/john/Videos/roadtrip.mp4","span":{"name":"import_video"}}
{"timestamp":"2024-01-15T10:30:00.124456Z","level":"INFO","target":"geotruth::commands::ingest","message":"Spawning FFprobe sidecar","video_path":"/Users/john/Videos/roadtrip.mp4","span":{"name":"extract_metadata","parent":"import_video"}}
{"timestamp":"2024-01-15T10:30:00.456789Z","level":"INFO","target":"geotruth::commands::ingest","message":"FFprobe completed","duration_ms":332,"span":{"name":"extract_metadata"}}
{"timestamp":"2024-01-15T10:30:00.457123Z","level":"INFO","target":"geotruth::commands::ingest","message":"Metadata extracted successfully","duration_secs":3823.5,"width":3840,"height":2160,"codec":"hevc"}
```

**Console Output (Pretty):**
```
  2024-01-15T10:30:00.123Z  INFO import_video{correlation_id=abc12345 video_path=/Users/john/Videos/roadtrip.mp4}: geotruth::commands::ingest: Starting video import
  2024-01-15T10:30:00.124Z  INFO import_video{...}:extract_metadata{video_path=/Users/john/Videos/roadtrip.mp4}: geotruth::commands::ingest: Spawning FFprobe sidecar
  2024-01-15T10:30:00.456Z  INFO import_video{...}:extract_metadata{...}: geotruth::commands::ingest: FFprobe completed duration_ms=332
  2024-01-15T10:30:00.457Z  INFO import_video{...}: geotruth::commands::ingest: Metadata extracted successfully duration_secs=3823.5 width=3840 height=2160 codec="hevc"
```

---

## 🧩 Frontend Logging

```typescript
// src/lib/logger.ts

type LogLevel = 'debug' | 'info' | 'warn' | 'error';

interface LogContext {
  component?: string;
  correlationId?: string;
  [key: string]: unknown;
}

class Logger {
  private component: string;
  
  constructor(component: string) {
    this.component = component;
  }
  
  private log(level: LogLevel, message: string, context: LogContext = {}) {
    const entry = {
      timestamp: new Date().toISOString(),
      level,
      component: this.component,
      message,
      ...context,
    };
    
    // In development, pretty print
    if (import.meta.env.DEV) {
      const color = {
        debug: 'color: gray',
        info: 'color: blue',
        warn: 'color: orange',
        error: 'color: red',
      }[level];
      
      console.log(`%c[${level.toUpperCase()}] ${this.component}:`, color, message, context);
    } else {
      // In production, structured JSON
      console.log(JSON.stringify(entry));
    }
    
    // Also send to Rust backend for unified logging
    if (level === 'error' || level === 'warn') {
      invoke('log_frontend', { entry });
    }
  }
  
  debug(message: string, context?: LogContext) { this.log('debug', message, context); }
  info(message: string, context?: LogContext) { this.log('info', message, context); }
  warn(message: string, context?: LogContext) { this.log('warn', message, context); }
  error(message: string, context?: LogContext) { this.log('error', message, context); }
}

export const createLogger = (component: string) => new Logger(component);

// Usage:
// const logger = createLogger('Timeline');
// logger.info('Event selected', { eventId: 'abc123', timestamp: '00:45:30' });
```

---

## 🔨 Building for Release

### Build Commands

```bash
# Build for current platform (in Docker)
docker compose -f docker-compose.dev.yml run --rm dev pnpm tauri build

# Or use make targets
make build-macos
make build-windows
make build-linux
```

### Build Output

```
target/release/bundle/
├── dmg/
│   └── GeoTruth_1.0.0_aarch64.dmg     # macOS
├── msi/
│   └── GeoTruth_1.0.0_x64_en-US.msi   # Windows
└── appimage/
    └── GeoTruth_1.0.0_amd64.AppImage  # Linux
```

---

## 📋 Cargo Dependencies (Latest)

```toml
# src-tauri/Cargo.toml

[package]
name = "geotruth-desktop"
version = "1.0.0"
edition = "2021"
rust-version = "1.75"

[dependencies]
# Tauri
tauri = { version = "2.1", features = ["devtools"] }
tauri-plugin-store = "2.1"

# Async
tokio = { version = "1.42", features = ["full"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Database
duckdb = { version = "1.1", features = ["bundled"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["json", "env-filter"] }
tracing-appender = "0.2"

# Error handling
thiserror = "2.0"
anyhow = "1.0"

# Utilities
uuid = { version = "1.11", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
dirs = "5.0"

# GPS parsing
gpx = "0.10"
nmea = "0.6"

# Security
keyring = "3.4"

[build-dependencies]
tauri-build = "2.0"
```

---

## 📚 Related Documentation

- [Architecture Overview](../architecture/README.md)
- [Logging Guide](../logging.md)
- [User Guide](../user-guide/README.md)
- [Backend Services](../backend/README.md)
