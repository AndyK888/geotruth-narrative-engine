# Local Intelligence Layer

The **Local Intelligence Layer** is the subsystem responsible for "True Offline" capabilities in GeoTruth. It allows the application to perform verification, geocoding, and narration without any internet connection.

---

## 🏗️ Architecture

It runs entirely within the **Rust Backend** of the Tauri application, leveraging embedded databases and bundled inference engines.

```
┌─────────────────────────────────────────────────────────────┐
│                   Rust Backend (src-tauri)                   │
│                                                             │
│  ┌─────────────────┐   ┌────────────────────────────────┐   │
│  │  Orchestrator   ├──►│      Offline Truth Engine      │   │
│  └────────┬────────┘   │  (Reverse Geo / Snapping)      │   │
│           │            └──────────────┬─────────────────┘   │
│           │                           │                     │
│           │                    ┌──────▼──────┐              │
│           │                    │   PMTiles   │              │
│           │                    │   Reader    │              │
│           │                    └─────────────┘              │
│           │                                                 │
│  ┌────────▼────────┐   ┌────────────────────────────────┐   │
│  │   AI Manager    ├──►│         Llama.cpp              │   │
│  └─────────────────┘   │    (GGUF Model Wrapper)        │   │
│                        └────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
```

---

## 🗺️ Offline Maps & Geocoding

### Technology Stack
- **Format**: [PMTiles](https://github.com/protomaps/PMTiles) (Cloud-Optimized single-file archives of vector tiles)
- **Data Source**: OpenStreetMap (via Protomaps builds)
- **Engine**: Custom Rust reader embedded in Tauri app

### Data Management
Users download specific "Map Packs" for their region via the **Data Manager** UI.
- **California**: ~200MB (Roads + POIs)
- **North America**: ~3GB

### Reverse Geocoding Logic
1.  **Input**: GPS Coordinate (lat/lon)
2.  **Lookup**: Query local PMTiles for features at zoom level 14-16
3.  **Filter**: Prioritize features with `name` tag and `highway` or `amenity` tags
4.  **Output**: "Near Grand Canyon Visitor Center"

### Road Snapping (Simplified)
Unlike Valhalla's full A* routing engine, the local snapper uses a simpler "Nearest Point on Edge" algorithm against the vector road network in the PMTiles archive. It is sufficient for identifying which road a car is on, but not for complex route reconstruction.

---

## 🤖 Offline AI Narration

### Technology Stack
- **Engine**: [Llama.cpp](https://github.com/ggerganov/llama.cpp) (bundled as shared library)
- **Model Format**: GGUF (Quantized)
- **Recommended Model**: `Mistral-7B-Instruct-v0.2.Q4_K_M.gguf` (~4GB) or `Phi-2` (~1.7GB) for lighter devices.

### Integration
The `geotruth-desktop` Rust crate links against `libllama` and exposes a `narrate_offline` command.

```rust
// src-tauri/src/services/ai/offline.rs

pub fn generate_narration(prompt: &str, model_path: &Path) -> Result<String> {
    let params = LlamaParams::default();
    let model = LlamaModel::load(model_path, &params)?;
    let mut session = model.create_session();
    
    session.predict(prompt, PredictionConfig::default())
}
```

### Constraints
Offline models are less capable than Gemini/GPT-4 so prompt engineering is stricter:
- **No creative writing**: "Summarize the Truth Bundle."
- **Strict formatting**: "List locations visited."

---

## 💾 Data Manager

A new module in the Desktop App to manage large offline assets.

### Features
- **Map Store**: Browse and download region packs.
- **Model Zoo**: Download optimized quantization levels (Q4, Q5, Q8) based on user's RAM.
- **Storage Management**: Auto-delete old cache, move assets to external SSD.

### Directory Structure
```
~/Library/Application Support/GeoTruth/data/
├── maps/
│   ├── california-2024.pmtiles
│   └── nevada-2024.pmtiles
└── models/
    └── mistral-7b-v0.2.Q4_K_M.gguf
```
