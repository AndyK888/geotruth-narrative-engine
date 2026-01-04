# Development Guide

The GeoTruth Narrative Engine is a **Monolithic Native Desktop Application** built with Tauri, Rust, and React.

## 🛠 Prerequisites

To develop this application, you need the standard Tauri development stack:

1.  **Rust**: Stable toolchain (install via [rustup](https://rustup.rs/)).
2.  **Node.js**: Version 20+ (LTS recommended) and `pnpm`.
3.  **Build Tools**:
    *   **macOS**: Xcode Command Line Tools (`xcode-select --install`).
    *   **Windows**: Visual Studio C++ Build Tools.
    *   **Linux**: `build-essential`, `libwebkit2gtk-4.0-dev`, `libssl-dev`, etc.

## 🚀 Quick Start

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/your-org/geotruth-narrative-engine.git
    cd geotruth-narrative-engine
    ```

2.  **Install Frontend Dependencies**:
    ```bash
    cd desktop
    pnpm install
    ```

3.  **Run Development Mode**:
    This command runs the React frontend and compiles the Rust backend, opening the desktop app with hot-reload enabled.
    ```bash
    npm run tauri dev
    # or
    pnpm tauri dev
    ```

## 🏗 Project Structure

- **`desktop/src`**: React Frontend (UI, Components, State).
- **`desktop/src-tauri`**: Rust Backend (Core Logic, Database, AI, Geospatial).
  - **`src/db.rs`**: DuckDB persistence layer.
  - **`src/geo.rs`**: Geospatial engine (PMTiles).
  - **`src/narrative.rs`**: Narrative generation logic.
  - **`src/enrich.rs`**: Location enrichment logic.
  - **`src/processor.rs`**: Media processing pipeline.
  - **`src/gemini.rs`**: Google Gemini API client.

## 🧪 Testing

### Backend (Rust)
Run Rust unit and integration tests:

```bash
cd desktop/src-tauri
cargo test
```

### Frontend (React/TypeScript)
Run Vitest for frontend components:

```bash
cd desktop
pnpm test
```

## 📦 Building for Production

To create a distributable application bundle (installer/DMG/exe):

```bash
cd desktop
pnpm tauri build
```
The output will be in `desktop/src-tauri/target/release/bundle/`.
