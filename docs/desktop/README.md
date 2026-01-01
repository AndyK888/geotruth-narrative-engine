# Desktop Application

The GeoTruth desktop application is built with **Tauri v2**, combining a Rust backend for performance-critical operations with a React frontend for the user interface. This document covers development, architecture, and deployment.

---

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Tauri Desktop App                         │
├─────────────────────────────────────────────────────────────┤
│  ┌───────────────────────────────────────────────────────┐  │
│  │                   React Frontend                       │  │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐  │  │
│  │  │  Timeline   │ │   Map View  │ │  Export Panel   │  │  │
│  │  │  Component  │ │  Component  │ │   Component     │  │  │
│  │  └─────────────┘ └─────────────┘ └─────────────────┘  │  │
│  │                         │                              │  │
│  │              TanStack Query + Tauri IPC               │  │
│  └─────────────────────────┬─────────────────────────────┘  │
│                            │                                 │
│  ┌─────────────────────────▼─────────────────────────────┐  │
│  │                    Rust Backend                        │  │
│  │  ┌─────────────┐ ┌─────────────┐ ┌─────────────────┐  │  │
│  │  │   Project   │ │    Sync     │ │    Sidecar      │  │  │
│  │  │   Manager   │ │   Engine    │ │  Orchestrator   │  │  │
│  │  └─────────────┘ └─────────────┘ └─────────────────┘  │  │
│  │                         │                              │  │
│  │                      DuckDB                            │  │
│  └─────────────────────────┬─────────────────────────────┘  │
│                            │                                 │
│  ┌─────────────────────────▼─────────────────────────────┐  │
│  │                   Sidecar Binaries                     │  │
│  │  ┌────────┐  ┌────────┐  ┌─────────┐  ┌───────────┐   │  │
│  │  │ FFmpeg │  │FFprobe │  │ Whisper │  │ Tesseract │   │  │
│  │  └────────┘  └────────┘  └─────────┘  └───────────┘   │  │
│  └───────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

---

## 📁 Directory Structure

```
/desktop
├── /src                        # React Frontend
│   ├── /components
│   │   ├── /timeline           # Truth Timeline components
│   │   ├── /map                # Map visualization
│   │   ├── /editor             # Event editor
│   │   └── /export             # Export options
│   ├── /hooks
│   │   ├── useProject.ts       # Project state management
│   │   ├── useEvents.ts        # Event queries
│   │   └── useTauri.ts         # Tauri IPC wrapper
│   ├── /stores
│   │   └── appStore.ts         # Zustand global state
│   ├── /lib
│   │   ├── tauri.ts            # Tauri command bindings
│   │   └── utils.ts            # Utility functions
│   ├── /pages
│   │   ├── Home.tsx
│   │   ├── Project.tsx
│   │   └── Settings.tsx
│   ├── App.tsx
│   ├── main.tsx
│   └── index.css
├── /src-tauri                  # Rust Backend
│   ├── /src
│   │   ├── main.rs             # Application entry
│   │   ├── lib.rs              # Library exports
│   │   ├── /commands           # Tauri commands
│   │   │   ├── mod.rs
│   │   │   ├── project.rs      # Project management
│   │   │   ├── ingest.rs       # Media ingestion
│   │   │   ├── sync.rs         # Time synchronization
│   │   │   └── export.rs       # Export functions
│   │   ├── /services
│   │   │   ├── mod.rs
│   │   │   ├── database.rs     # DuckDB operations
│   │   │   ├── gps.rs          # GPS parsing
│   │   │   ├── sidecar.rs      # FFmpeg/Whisper runner
│   │   │   └── api.rs          # Cloud API client
│   │   ├── /models
│   │   │   ├── mod.rs
│   │   │   ├── project.rs
│   │   │   ├── event.rs
│   │   │   └── truth.rs
│   │   └── /utils
│   │       ├── mod.rs
│   │       └── time.rs
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── build.rs
├── /binaries                   # Bundled sidecars
│   ├── ffmpeg-x86_64-apple-darwin
│   ├── ffprobe-x86_64-apple-darwin
│   ├── whisper-x86_64-apple-darwin
│   └── ...
├── package.json
├── vite.config.ts
└── tsconfig.json
```

---

## 🚀 Development Setup

### Prerequisites

- **Node.js** 18.x+
- **Rust** 1.70+ with `cargo`
- **FFmpeg** (for development)
- **Tauri CLI**: `cargo install tauri-cli`

### Installation

```bash
# Navigate to desktop directory
cd desktop

# Install JavaScript dependencies
npm install

# Install Rust dependencies (automatic via Cargo)
cd src-tauri && cargo build

# Return to desktop root
cd ..
```

### Running Development

```bash
# Start Tauri dev server (hot reload for both frontend and Rust)
npm run tauri dev

# Or run commands separately:
# Terminal 1: Frontend dev server
npm run dev

# Terminal 2: Rust backend
cd src-tauri && cargo tauri dev
```

### Building for Production

```bash
# Build optimized bundle
npm run tauri build

# Output locations:
# macOS: target/release/bundle/dmg/
# Windows: target/release/bundle/msi/
# Linux: target/release/bundle/appimage/
```

---

## ⚙️ Tauri Configuration

### tauri.conf.json

```json
{
  "$schema": "https://schema.tauri.app/config/2",
  "productName": "GeoTruth",
  "version": "1.0.0",
  "identifier": "com.geotruth.desktop",
  "build": {
    "frontendDist": "../dist",
    "devUrl": "http://localhost:5173",
    "beforeDevCommand": "npm run dev",
    "beforeBuildCommand": "npm run build"
  },
  "app": {
    "windows": [
      {
        "title": "GeoTruth Narrative Engine",
        "width": 1400,
        "height": 900,
        "minWidth": 1000,
        "minHeight": 700,
        "resizable": true,
        "fullscreen": false
      }
    ],
    "security": {
      "csp": "default-src 'self'; img-src 'self' data: https://tile.openstreetmap.org; style-src 'self' 'unsafe-inline'"
    }
  },
  "bundle": {
    "active": true,
    "icon": [
      "icons/32x32.png",
      "icons/128x128.png",
      "icons/128x128@2x.png",
      "icons/icon.icns",
      "icons/icon.ico"
    ],
    "externalBin": [
      "binaries/ffmpeg",
      "binaries/ffprobe",
      "binaries/whisper"
    ],
    "resources": [
      "resources/*"
    ]
  },
  "plugins": {
    "store": {
      "path": ".geotruth-settings.json"
    }
  }
}
```

---

## 🦀 Rust Backend

### Tauri Commands

Commands are the bridge between frontend and backend:

```rust
// src-tauri/src/commands/project.rs

use tauri::State;
use crate::services::database::Database;
use crate::models::project::Project;

#[tauri::command]
pub async fn create_project(
    db: State<'_, Database>,
    name: String,
    path: String,
) -> Result<Project, String> {
    let project = Project::new(name, path);
    db.insert_project(&project)
        .await
        .map_err(|e| e.to_string())?;
    Ok(project)
}

#[tauri::command]
pub async fn get_project(
    db: State<'_, Database>,
    id: String,
) -> Result<Project, String> {
    db.get_project(&id)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_projects(
    db: State<'_, Database>,
) -> Result<Vec<Project>, String> {
    db.list_projects()
        .await
        .map_err(|e| e.to_string())
}
```

### Register Commands

```rust
// src-tauri/src/main.rs

mod commands;
mod services;
mod models;

use commands::{project, ingest, sync, export};
use services::database::Database;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            // Initialize database
            let db = Database::new(app.path().app_data_dir().unwrap())?;
            app.manage(db);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // Project commands
            project::create_project,
            project::get_project,
            project::list_projects,
            project::delete_project,
            
            // Ingest commands
            ingest::import_video,
            ingest::import_gps,
            ingest::analyze_media,
            
            // Sync commands
            sync::calculate_offset,
            sync::sync_timeline,
            
            // Export commands
            export::export_chapters,
            export::export_script,
            export::export_subtitles,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

### DuckDB Integration

```rust
// src-tauri/src/services/database.rs

use duckdb::{Connection, Result as DuckResult};
use std::path::PathBuf;
use std::sync::Mutex;

pub struct Database {
    conn: Mutex<Connection>,
}

impl Database {
    pub fn new(data_dir: PathBuf) -> DuckResult<Self> {
        let db_path = data_dir.join("geotruth.duckdb");
        let conn = Connection::open(db_path)?;
        
        // Initialize schema
        conn.execute_batch(include_str!("../../migrations/001_init.sql"))?;
        
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }
    
    pub async fn insert_event(&self, event: &Event) -> DuckResult<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "INSERT INTO events (id, project_id, video_path, start_time, end_time, geo_lat, geo_lon)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
            [
                &event.id,
                &event.project_id,
                &event.video_path,
                &event.start_time.to_string(),
                &event.end_time.to_string(),
                &event.geo_lat.to_string(),
                &event.geo_lon.to_string(),
            ],
        )?;
        Ok(())
    }
}
```

### Sidecar Execution

```rust
// src-tauri/src/services/sidecar.rs

use tauri::api::process::{Command, CommandEvent};
use tokio::sync::mpsc;

pub struct SidecarRunner;

impl SidecarRunner {
    /// Extract video metadata using FFprobe
    pub async fn get_video_info(video_path: &str) -> Result<VideoInfo, String> {
        let (mut rx, _child) = Command::new_sidecar("ffprobe")
            .map_err(|e| e.to_string())?
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
                video_path,
            ])
            .spawn()
            .map_err(|e| e.to_string())?;
        
        let mut output = String::new();
        while let Some(event) = rx.recv().await {
            if let CommandEvent::Stdout(line) = event {
                output.push_str(&line);
            }
        }
        
        serde_json::from_str(&output).map_err(|e| e.to_string())
    }
    
    /// Transcribe audio using Whisper
    pub async fn transcribe(
        audio_path: &str,
        on_progress: impl Fn(f32),
    ) -> Result<Transcript, String> {
        let (mut rx, _child) = Command::new_sidecar("whisper")
            .map_err(|e| e.to_string())?
            .args([
                "--model", "base",
                "--output-format", "json",
                "--language", "auto",
                audio_path,
            ])
            .spawn()
            .map_err(|e| e.to_string())?;
        
        while let Some(event) = rx.recv().await {
            match event {
                CommandEvent::Stdout(line) => {
                    // Parse progress from output
                    if let Some(progress) = parse_whisper_progress(&line) {
                        on_progress(progress);
                    }
                }
                CommandEvent::Terminated { code, .. } => {
                    if code != Some(0) {
                        return Err("Whisper failed".to_string());
                    }
                }
                _ => {}
            }
        }
        
        // Load output file
        let output_path = format!("{}.json", audio_path);
        let content = std::fs::read_to_string(&output_path)
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&content).map_err(|e| e.to_string())
    }
}
```

---

## ⚛️ React Frontend

### Tauri IPC from React

```typescript
// src/lib/tauri.ts

import { invoke } from '@tauri-apps/api/core';

export interface Project {
  id: string;
  name: string;
  path: string;
  createdAt: string;
}

export interface Event {
  id: string;
  projectId: string;
  videoPath: string;
  startTime: string;
  endTime: string;
  geoLat: number;
  geoLon: number;
  truthJson?: TruthBundle;
}

// Project commands
export const createProject = (name: string, path: string) =>
  invoke<Project>('create_project', { name, path });

export const getProject = (id: string) =>
  invoke<Project>('get_project', { id });

export const listProjects = () =>
  invoke<Project[]>('list_projects');

// Ingest commands
export const importVideo = (projectId: string, videoPath: string) =>
  invoke<void>('import_video', { projectId, videoPath });

export const importGps = (projectId: string, gpsPath: string) =>
  invoke<void>('import_gps', { projectId, gpsPath });

// Sync commands
export const calculateOffset = (projectId: string) =>
  invoke<number>('calculate_offset', { projectId });

// Export commands
export const exportChapters = (projectId: string, outputPath: string) =>
  invoke<void>('export_chapters', { projectId, outputPath });
```

### TanStack Query Integration

```typescript
// src/hooks/useProject.ts

import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import * as tauri from '../lib/tauri';

export function useProjects() {
  return useQuery({
    queryKey: ['projects'],
    queryFn: tauri.listProjects,
  });
}

export function useProject(id: string) {
  return useQuery({
    queryKey: ['project', id],
    queryFn: () => tauri.getProject(id),
    enabled: !!id,
  });
}

export function useCreateProject() {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: ({ name, path }: { name: string; path: string }) =>
      tauri.createProject(name, path),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['projects'] });
    },
  });
}

export function useImportVideo(projectId: string) {
  const queryClient = useQueryClient();
  
  return useMutation({
    mutationFn: (videoPath: string) =>
      tauri.importVideo(projectId, videoPath),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['project', projectId] });
      queryClient.invalidateQueries({ queryKey: ['events', projectId] });
    },
  });
}
```

### Event Listening

```typescript
// src/hooks/useProgress.ts

import { useEffect, useState } from 'react';
import { listen } from '@tauri-apps/api/event';

interface ProgressPayload {
  stage: string;
  progress: number;
  message: string;
}

export function useProgress() {
  const [progress, setProgress] = useState<ProgressPayload | null>(null);

  useEffect(() => {
    const unlisten = listen<ProgressPayload>('processing-progress', (event) => {
      setProgress(event.payload);
    });

    return () => {
      unlisten.then((fn) => fn());
    };
  }, []);

  return progress;
}
```

---

## 🔐 Security

### API Key Storage

```typescript
// Using tauri-plugin-store for preferences
import { Store } from '@tauri-apps/plugin-store';

const store = new Store('.settings.json');

// Non-sensitive settings
await store.set('theme', 'dark');
await store.set('defaultExportPath', '/Users/me/exports');
await store.save();

// Sensitive data should use OS keychain
// (via tauri-plugin-stronghold or keytar wrapper)
```

### Keychain Access (Rust)

```rust
// src-tauri/src/services/keychain.rs

use keyring::Entry;

pub fn store_api_key(service: &str, key: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(service, "api_key")?;
    entry.set_password(key)?;
    Ok(())
}

pub fn get_api_key(service: &str) -> Result<String, keyring::Error> {
    let entry = Entry::new(service, "api_key")?;
    entry.get_password()
}

pub fn delete_api_key(service: &str) -> Result<(), keyring::Error> {
    let entry = Entry::new(service, "api_key")?;
    entry.delete_password()?;
    Ok(())
}
```

---

## 🧪 Testing

### Rust Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gps_parsing() {
        let gpx_content = include_str!("../../test_data/sample.gpx");
        let track = parse_gpx(gpx_content).unwrap();
        
        assert!(!track.points.is_empty());
        assert!(track.points[0].lat != 0.0);
    }

    #[test]
    fn test_time_offset_calculation() {
        let video_time = "2024-01-15T10:30:05";
        let gps_time = "2024-01-15T10:30:00";
        
        let offset = calculate_offset(video_time, gps_time);
        assert_eq!(offset, 5);
    }
}
```

### Frontend Tests

```typescript
// src/__tests__/useProject.test.ts

import { renderHook, waitFor } from '@testing-library/react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { useProjects } from '../hooks/useProject';

// Mock Tauri
vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}));

describe('useProjects', () => {
  it('fetches projects list', async () => {
    const { invoke } = await import('@tauri-apps/api/core');
    (invoke as any).mockResolvedValue([
      { id: '1', name: 'Test Project', path: '/test' },
    ]);

    const queryClient = new QueryClient();
    const wrapper = ({ children }) => (
      <QueryClientProvider client={queryClient}>
        {children}
      </QueryClientProvider>
    );

    const { result } = renderHook(() => useProjects(), { wrapper });

    await waitFor(() => expect(result.current.isSuccess).toBe(true));
    expect(result.current.data).toHaveLength(1);
  });
});
```

---

## 📦 Bundling Sidecars

### Sidecar Naming Convention

Tauri requires specific naming for platform-specific binaries:

```
binaries/
├── ffmpeg-x86_64-apple-darwin        # Intel Mac
├── ffmpeg-aarch64-apple-darwin       # Apple Silicon
├── ffmpeg-x86_64-pc-windows-msvc.exe # Windows
├── ffmpeg-x86_64-unknown-linux-gnu   # Linux
└── ...
```

### Download Script

```bash
#!/bin/bash
# scripts/download-sidecars.sh

PLATFORM=$(rustc -vV | sed -n 's/host: //p')
BINARIES_DIR="binaries"

mkdir -p $BINARIES_DIR

# FFmpeg
curl -L "https://github.com/BtbN/FFmpeg-Builds/releases/download/latest/ffmpeg-master-latest-$PLATFORM.tar.xz" | tar xJ
mv ffmpeg-*/bin/ffmpeg "$BINARIES_DIR/ffmpeg-$PLATFORM"
mv ffmpeg-*/bin/ffprobe "$BINARIES_DIR/ffprobe-$PLATFORM"

# Whisper.cpp
# ... similar download logic
```

---

## 📚 Related Documentation

- [Architecture Overview](../architecture/README.md)
- [API Reference](../api/README.md)
- [User Guide](../user-guide/README.md)
