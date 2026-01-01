# GeoTruth Narrative Engine - Project Specification

## Overview

GeoTruth is a desktop application that transforms raw travel footage into fact-checked, AI-narrated stories using a "Verify First, Narrate Second" architecture. It prioritizes local processing and privacy, offering a **Hybrid Architecture** that seamlessly transitions between powerful cloud-based verification and lightweight offline capabilities for remote travelers.

## Problem Statement

- Travelers and content creators have hours of footage but no time to edit
- Standard AI tools "hallucinate" facts about locations
- Most tools require constant internet connection, failing in remote areas (van life, hiking)
- Privacy is often compromised by uploading personal footage to the cloud

## Solution

A privacy-first desktop application that:
1.  **Syncs**: Mathematically matches video timestamps with GPS data
2.  **Verifies**: Validates locations using trusted geospatial databases (Online or Offline)
3.  **Narrates**: Generates scripts based *only* on verified evidence
4.  **Publishes**: Exports chapters, subtitles, and finalized videos

## Target Users

1.  **The Standalone Traveler** - Van life/overlanders often without internet for weeks. Needs full offline functionality.
2.  **The Dashcam Archivist** - Wants automated cataloging of drives without subscription fees.
3.  **The Content Creator** - Needs accurate, fact-checked foundations for travel vlogs.

## Core Architectural Principles

### 1. Privacy by Design
- Video files never leave the user's device
- Only anonymized GPS coordinates sent to cloud (in Online Mode)
- Zero data leaves device in **Offline Mode**

### 2. Truth Before Generation
- "Truth Bundle" created *before* AI sees any prompt
- GPS matched to road networks (Valhalla online / PMTiles offline)
- AI restricted to narrating only facts present in the Truth Bundle

### 3. Hybrid Connectivity
- **Online Mode**: High-definition verification via Cloud API (Valhalla/Gemini)
- **Offline Mode**: Lightweight verification via Local Truth Engine (Rust/Llama)
- Graceful degradation: The app never "bricks" without internet

### 4. Human in the Loop
- Visual timeline of all detected events
- User must approve Truth Bundle before final narration
- "Trust but Verify" approach to AI outputs

## Technical Architecture

### Desktop Layer (Tauri v2)
- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust (Core logic + Local Intelligence)
- **Database**: DuckDB (Local analytics)
- **Bundled Binaries**: FFmpeg, Whisper.cpp, Tesseract

### Local Intelligence Layer (Offline)
- **Maps**: PMTiles (Vector tiles for local rendering & snapping)
- **Search**: Tantivy/SQLite (Local reverse geocoding)
- **AI**: Llama.cpp (Optional download for offline narration)

### Cloud Layer (Docker/API)
- **API**: FastAPI (Python)
- **Geospatial**: PostGIS + Valhalla
- **Services**: Google Gemini (AI)

## Key Features

| Feature | Priority | Connectivity | Status |
|---------|----------|--------------|--------|
| Video/GPS Import | P0 | Offline | Planned |
| Time Sync | P0 | Offline | Planned |
| Map Matching | P0 | Hybrid | Planned |
| POI Discovery | P0 | Hybrid | Planned |
| **Offline Mode** | **P0** | **Offline** | **Planned** |
| AI Narration | P0 | Hybrid | Planned |
| Chapter Export | P0 | Offline | Planned |
| Interactive Map | P2 | Hybrid | Planned |

## Success Metrics

1.  **Accuracy**: 100% of mentioned locations are geographically correct (Online Mode)
2.  **Offline Capability**: **100% of core flows (Import -> Verify -> Narrate) work without internet**
3.  **Processing**: 1 hour of video processed in < 10 mins (Online) / < 20 mins (Offline)
4.  **Privacy**: 0 bytes of data leave device in Offline Mode

## Timeline

### Phase 1: Foundation (Weeks 1-4)
- Project skeleton (Monorepo)
- Desktop shell (Tauri)
- Docker backend setup

### Phase 2: Core Pipeline (Weeks 5-8)
- Video ingestion & Sync
- Online Map Matching (API)
- **Local Intelligence Foundation (Offline Geocoder)**

### Phase 3: Intelligence (Weeks 9-12)
- POI Discovery (Hybrid)
- AI Narration (Gemini + Llama)
- **Offline Data Manager (Map Packs)**

### Phase 4: Polish (Weeks 13-16)
- UI/UX Refinement
- Export Formats
- Performance Optimization

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Offline model quality | Use constraints strictly; warn user of lower fidelity vs Gemini |
| Map pack size | Use vector tiles (PMTiles) over raster; region-based downloads |
| Local performance | Offload to Rust threads; support GPU acceleration for Llama |
| API Costs | Heavy reliance on offline first reduces cloud API bills |

---

*Last Updated: 2024-01-15*
*Version: 1.1.0*
