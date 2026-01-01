# User Guide

Welcome to GeoTruth! This guide will help you get started with turning your travel footage into fact-checked, AI-narrated stories.

---

## 🚀 Getting Started

### Installation

1. **Download** GeoTruth from [geotruth.io/download](https://geotruth.io/download)
2. **Install** by dragging to Applications (Mac) or running the installer (Windows/Linux)
3. **Launch** and complete the initial setup

### First-Time Setup

On first launch, you'll be asked to:

1. **Sign in** or create an account
2. **Configure API keys** (optional - for BYOK users)
3. **Set default export location**
4. **Choose your theme** (Light/Dark)

---

## 📁 Creating a Project

### Step 1: New Project

1. Click **"New Project"** on the home screen
2. Enter a **project name** (e.g., "Grand Canyon Road Trip")
3. Choose a **save location**
4. Click **"Create"**

### Step 2: Import Media

Drag and drop your files into the project window:

| File Type | Formats Supported |
|-----------|-------------------|
| **Video** | MP4, MOV, MKV, AVI, WebM |
| **GPS** | GPX, NMEA, FIT, KML |
| **Images** | JPG, PNG (for reference) |

> **💡 Tip:** GeoTruth works best when video and GPS are from the same recording session.

### Step 3: Synchronization

GeoTruth automatically aligns your video with GPS data:

```
┌─────────────────────────────────────────────────────────┐
│                  Synchronization                         │
├─────────────────────────────────────────────────────────┤
│  ✅ Video detected: GoPro_001.MP4 (2:34:15)             │
│  ✅ GPS detected: 2024-01-15.gpx (15,234 points)        │
│                                                          │
│  🔄 Calculating time offset...                          │
│     Method: OCR timestamp detection                      │
│     Offset: +5 seconds                                   │
│                                                          │
│  [●●●●●●●●●●○○○○○○○○○○] 50%                              │
└─────────────────────────────────────────────────────────┘
```

**If automatic sync fails:**
1. Click **"Manual Sync"**
2. Find a recognizable moment in the video (car door, specific turn)
3. Match it to the corresponding GPS point
4. GeoTruth will calculate the offset

---

## 🗺️ The Truth Timeline

The Truth Timeline is your workspace for reviewing and editing detected events.

### Interface Overview

```
┌─────────────────────────────────────────────────────────────┐
│  [Video Player]                      [Map View]             │
│  ┌─────────────────────┐            ┌──────────────────┐   │
│  │                     │            │    🗺️ Map        │   │
│  │    ▶️ Video         │            │   with route     │   │
│  │                     │            │   and POIs       │   │
│  └─────────────────────┘            └──────────────────┘   │
├─────────────────────────────────────────────────────────────┤
│  [Truth Timeline]                                           │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ 🎬 ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━││
│  │ 📍 ──○──────○────────○──────────○────────○───────────── ││
│  │ 🏔️     [Grand Canyon]        [Gas Station]    [Sunset] ││
│  └─────────────────────────────────────────────────────────┘│
│  0:00                                                 2:34:15│
└─────────────────────────────────────────────────────────────┘
```

### Timeline Tracks

| Track | Description |
|-------|-------------|
| 🎬 **Video** | Thumbnail strip with playhead |
| 📍 **GPS** | Speed/elevation graph |
| 🏔️ **Events** | Detected POIs and stops |

### Interacting with Events

- **Click** an event to seek to that moment
- **Hover** to see quick info
- **Right-click** for options:
  - ✏️ Edit name/description
  - ✅ Mark as verified
  - ❌ Delete event
  - ➕ Add to script

---

## ✏️ Editing Events

### Event Details Panel

When you select an event, the details panel shows:

```
┌────────────────────────────────────────┐
│ 📍 Grand Canyon South Rim              │
├────────────────────────────────────────┤
│ Type: Landmark                         │
│ Time: 00:45:30 - 00:47:15              │
│ Confidence: 98%                        │
│                                        │
│ 📝 Evidence:                           │
│ • GPS: 36.1069°N, 112.1129°W          │
│ • Road: AZ-64 (Highway)               │
│ • Distance: 150m from viewpoint       │
│                                        │
│ 📚 Facts:                              │
│ • Established: 1919                    │
│ • Depth: 1,857 meters                  │
│ • UNESCO World Heritage Site           │
│                                        │
│ [✅ Verified] [✏️ Edit] [🗑️ Delete]   │
└────────────────────────────────────────┘
```

### Adding Custom Events

1. Pause the video at the desired moment
2. Click **"Add Event"** (+ button)
3. Choose event type:
   - 📍 Landmark
   - 🛑 Stop
   - 📝 Note
   - 🎬 Chapter marker
4. Enter details and save

---

## 🤖 AI Narration

### Generating a Script

1. Review and verify your events
2. Click **"Generate Narration"** 
3. Choose your style:

| Style | Best For |
|-------|----------|
| 🎬 **Documentary** | YouTube travel videos |
| 💬 **Casual** | Vlogs, personal journals |
| 📚 **Educational** | Tutorials, guides |
| ⚡ **Minimal** | Quick chapters only |

4. Click **"Generate"**

### Script Editor

```
┌─────────────────────────────────────────────────────────────┐
│  Script: Grand Canyon Road Trip                              │
├─────────────────────────────────────────────────────────────┤
│                                                              │
│  [00:00:00] We're setting off from Phoenix, Arizona,         │
│  heading north on Interstate 17. The desert landscape        │
│  stretches endlessly ahead...                                │
│                                                              │
│  [00:45:30] And here it is - the Grand Canyon South Rim.     │
│  Established as a National Park in 1919, this natural        │
│  wonder plunges 1,857 meters into the earth...               │
│                                                              │
│  ────────────────────────────────────────────────────────    │
│  [✏️ Edit]  [🔄 Regenerate Section]  [▶️ Preview TTS]       │
└─────────────────────────────────────────────────────────────┘
```

### Editing Tips

- **Click any paragraph** to edit directly
- **Regenerate sections** you don't like
- **Add custom text** between AI-generated content
- **Preview with TTS** to check pacing

---

## 📤 Exporting

### Export Options

Click **"Export"** to open the export panel:

| Format | Use Case |
|--------|----------|
| 📋 **chapters.txt** | YouTube video description |
| 📝 **script.md** | Blog posts, transcripts |
| 🎬 **subtitles.srt** | Video subtitles with location |
| 🗺️ **map.html** | Interactive route map |
| 📦 **project.json** | Full project backup |

### YouTube Chapters Format

```
00:00 - Starting from Phoenix
04:15 - Entering Coconino National Forest  
28:30 - First glimpse of the Canyon
45:30 - Arriving at South Rim Viewpoint
1:12:45 - Sunset at Mather Point
```

### Blog Export

```markdown
# Grand Canyon Road Trip

## The Journey Begins

We set off from Phoenix at dawn, the city lights fading 
in the rearview mirror as the desert awakened...

## The Grand Reveal (45:30)

📍 *Grand Canyon South Rim, Arizona*

Nothing quite prepares you for this moment. Established 
as a National Park in 1919, the canyon stretches...
```

---

## ⚙️ Settings

### General Settings

| Setting | Description |
|---------|-------------|
| **Theme** | Light, Dark, or System |
| **Default Export Path** | Where exports are saved |
| **Auto-save** | Save projects automatically |
| **Language** | Interface language |

### API Settings

For users who bring their own API keys:

1. Go to **Settings → API Keys**
2. Enter your **Gemini API Key**
3. Keys are stored securely in your OS keychain

### Processing Settings

| Setting | Options |
|---------|---------|
| **Transcription Model** | Tiny, Base, Small, Medium |
| **OCR Sync** | Enabled/Disabled |
| **POI Search Radius** | 100m - 5000m |

---

## 🔧 Troubleshooting

### Common Issues

#### Video Won't Import
- **Check format**: Use MP4, MOV, or MKV
- **Check codec**: H.264/H.265 work best
- **Try converting**: Use HandBrake if needed

#### GPS Not Syncing
- **Time zones**: Ensure GPS device time was correct
- **File format**: Try converting to GPX
- **Manual sync**: Use a visual reference point

#### Slow Processing
- **Reduce quality**: Use transcription model "Base" instead of "Medium"
- **Smaller chunks**: Process video in segments
- **Check resources**: Close other heavy applications

### Getting Help

- 📖 [FAQ](https://geotruth.io/faq)
- 💬 [Community Discord](https://discord.gg/geotruth)
- 📧 [Support](mailto:support@geotruth.io)

---

## ⌨️ Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Space` | Play/Pause video |
| `←` / `→` | Seek 5 seconds |
| `Shift+←` / `→` | Seek 30 seconds |
| `E` | Add event at current time |
| `M` | Toggle map view |
| `T` | Focus timeline |
| `Cmd/Ctrl+S` | Save project |
| `Cmd/Ctrl+E` | Export |
| `Cmd/Ctrl+G` | Generate narration |

---

## 📚 Related Documentation

- [Architecture Overview](../architecture/README.md)
- [API Reference](../api/README.md)
- [Development Guide](../development/README.md)
