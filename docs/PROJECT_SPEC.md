# GeoTruth Narrative Engine - Project Specification

## Overview

GeoTruth is a desktop application that transforms raw travel footage into fact-checked, AI-narrated stories using a "Verify First, Narrate Second" architecture.

## Problem Statement

- Travelers, dashcam users, and content creators have hours of footage but no time to edit
- Standard AI tools hallucinate facts about locations
- Video editing is time-consuming and requires technical skills
- GPS data and video are often desynchronized

## Solution

A privacy-first desktop application that:
1. Mathematically synchronizes video with GPS data
2. Validates visible landmarks using geospatial databases
3. Generates AI narration based on verified evidence only
4. Produces ready-to-use outputs (chapters, scripts, subtitles)

## Target Users

1. **Road Trippers** - Converting long drives into chaptered videos
2. **Dashcam Archivists** - Cataloging eventful moments automatically
3. **Zoo/Park Visitors** - Creating educational walking tour timelines

## Core Architectural Principles

### 1. Privacy by Design
- Video files never leave the user's device
- Only anonymized GPS coordinates sent to cloud
- Local transcription and frame extraction

### 2. Truth Before Generation
- GPS map-matched to actual roads before AI sees it
- POIs filtered by field-of-view calculations
- AI receives "Truth Bundle" with verified facts only

### 3. Human Verification
- All detected events shown on timeline
- User can verify/correct before script generation
- Audit trail of AI decisions

## Technical Architecture

### Desktop Layer (Tauri v2)
- **Frontend**: React + TypeScript + Vite
- **Backend**: Rust
- **Local DB**: DuckDB
- **Sidecars**: FFmpeg, Whisper.cpp, Tesseract

### Cloud Layer (Docker Compose)
- **API**: FastAPI (Python)
- **Database**: PostGIS
- **Routing**: Valhalla
- **Cache**: Redis

### AI Layer
- **Provider**: Google Gemini
- **Grounding**: Constraint-based prompts with Truth Bundle

## Key Features

| Feature | Priority | Status |
|---------|----------|--------|
| Video/GPS import | P0 | Planned |
| Time synchronization | P0 | Planned |
| Map matching | P0 | Planned |
| POI discovery | P0 | Planned |
| FOV filtering | P1 | Planned |
| AI narration | P0 | Planned |
| Chapter export | P0 | Planned |
| Subtitle export | P1 | Planned |
| Interactive map | P2 | Planned |
| Visual verification (Zoo mode) | P2 | Planned |

## Success Metrics

1. **Accuracy**: 100% of mentioned locations are geographically correct
2. **Processing**: 1 hour of video processed in < 10 minutes
3. **Usability**: Single-click from import to export
4. **Privacy**: Zero video data on external servers

## Timeline

### Phase 1: Foundation (Weeks 1-4)
- Project structure and CI/CD
- Desktop app shell
- Backend services (PostGIS, Valhalla)

### Phase 2: Core Pipeline (Weeks 5-8)
- Video/GPS ingestion
- Time synchronization
- Map matching integration

### Phase 3: Intelligence (Weeks 9-12)
- POI discovery and filtering
- Truth Bundle generation
- AI narration integration

### Phase 4: Polish (Weeks 13-16)
- Timeline UI
- Export formats
- Testing and optimization

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Video format compatibility | Use FFmpeg with wide codec support |
| GPS accuracy issues | Map matching + visual verification fallback |
| AI hallucination | Strict constraint prompts, Truth Bundle only |
| Privacy concerns | Local-first architecture, clear data policy |
| Performance (large files) | Chunked processing, DuckDB for analytics |

## Open Questions

1. Monetization model (one-time purchase vs subscription)?
2. Regional map data coverage priority?
3. Third-party integration requirements (YouTube, Vimeo)?
4. Mobile companion app needed?

---

*Last Updated: 2024-01-15*
*Version: 1.0.0*
