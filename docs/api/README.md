# API Reference

Complete REST API documentation for the GeoTruth backend services.

---

## 🔐 Authentication

All endpoints (except `/v1/health`) require JWT authentication.

### Headers

```
Authorization: Bearer <jwt_token>
Content-Type: application/json
```

### Obtaining a Token

```http
POST /v1/auth/token
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "your_password"
}
```

**Response:**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "token_type": "bearer",
  "expires_in": 86400
}
```

---

## 📍 Endpoints

### Health Check

#### `GET /v1/health`

Check service health status.

**Response:**
```json
{
  "status": "healthy",
  "version": "1.0.0",
  "services": {
    "database": "connected",
    "redis": "connected",
    "valhalla": "connected"
  },
  "uptime_seconds": 86400
}
```

---

### Enrichment

#### `POST /v1/enrich`

Enrich a single GPS coordinate with geospatial context.

**Request:**
```json
{
  "lat": 36.1069,
  "lon": -112.1129,
  "timestamp": "2024-01-15T10:30:00Z",
  "heading_deg": 90,
  "fov_deg": 120
}
```

**Response:**
```json
{
  "location": {
    "lat": 36.1069,
    "lon": -112.1129,
    "matched": {
      "lat": 36.1068,
      "lon": -112.1130,
      "road_name": "AZ-64",
      "road_class": "primary"
    }
  },
  "context": {
    "country": "United States",
    "state": "Arizona",
    "county": "Coconino",
    "timezone": "America/Phoenix",
    "elevation_m": 2134
  },
  "pois": [
    {
      "id": "poi_12345",
      "name": "Grand Canyon South Rim",
      "category": "natural_landmark",
      "distance_m": 150,
      "bearing_deg": 45,
      "in_fov": true,
      "confidence": 0.98,
      "facts": {
        "established": "1919",
        "depth_m": 1857,
        "unesco_site": true
      }
    }
  ]
}
```

**Error Responses:**

| Status | Description |
|--------|-------------|
| 400 | Invalid coordinates |
| 401 | Unauthorized |
| 429 | Rate limit exceeded |
| 500 | Internal server error |

---

#### `POST /v1/enrich_batch`

Enrich multiple GPS coordinates in a single request.

**Request:**
```json
{
  "points": [
    {
      "lat": 36.1069,
      "lon": -112.1129,
      "timestamp": "2024-01-15T10:30:00Z"
    },
    {
      "lat": 36.1075,
      "lon": -112.1100,
      "timestamp": "2024-01-15T10:35:00Z"
    }
  ],
  "options": {
    "include_roads": true,
    "include_pois": true,
    "poi_radius_m": 500,
    "poi_categories": ["landmark", "natural", "tourism"]
  }
}
```

**Response:**
```json
{
  "results": [
    {
      "index": 0,
      "location": { ... },
      "context": { ... },
      "pois": [ ... ]
    },
    {
      "index": 1,
      "location": { ... },
      "context": { ... },
      "pois": [ ... ]
    }
  ],
  "meta": {
    "total_points": 2,
    "processing_time_ms": 125,
    "cache_hits": 1
  }
}
```

**Limits:**
- Maximum 100 points per request
- Rate limit: 1000 points/minute

---

### Map Matching

#### `POST /v1/map_match`

Snap GPS coordinates to the road network.

**Request:**
```json
{
  "coordinates": [
    {
      "lat": 36.1069,
      "lon": -112.1129,
      "timestamp": "2024-01-15T10:30:00Z",
      "accuracy_m": 10
    },
    {
      "lat": 36.1070,
      "lon": -112.1125,
      "timestamp": "2024-01-15T10:30:05Z",
      "accuracy_m": 15
    }
  ],
  "costing": "auto",
  "shape_match": "walk_or_snap"
}
```

**Costing Options:**
- `auto` - Car/automobile
- `pedestrian` - Walking
- `bicycle` - Cycling
- `bus` - Public transit

**Response:**
```json
{
  "matched_points": [
    {
      "lat": 36.1068,
      "lon": -112.1130,
      "edge_id": 12345,
      "distance_from_input_m": 15
    },
    {
      "lat": 36.1071,
      "lon": -112.1124,
      "edge_id": 12346,
      "distance_from_input_m": 8
    }
  ],
  "edges": [
    {
      "id": 12345,
      "osm_way_id": 98765432,
      "road_name": "AZ-64",
      "road_class": "primary",
      "length_m": 150,
      "speed_limit_kmh": 80,
      "begin_heading": 85,
      "end_heading": 90
    }
  ],
  "route": {
    "total_distance_m": 500,
    "total_time_s": 45,
    "geometry": {
      "type": "LineString",
      "coordinates": [[-112.1130, 36.1068], [-112.1124, 36.1071]]
    }
  }
}
```

---

### POI Queries

#### `GET /v1/pois`

Query points of interest.

**Query Parameters:**

| Parameter | Type | Required | Description |
|-----------|------|----------|-------------|
| `lat` | float | Yes | Latitude |
| `lon` | float | Yes | Longitude |
| `radius_m` | int | No | Search radius (default: 500, max: 5000) |
| `categories` | string[] | No | Filter by categories |
| `limit` | int | No | Max results (default: 50, max: 100) |
| `heading_deg` | float | No | Camera heading for FOV filter |
| `fov_deg` | float | No | Field of view (default: 180) |

**Example:**
```
GET /v1/pois?lat=36.1069&lon=-112.1129&radius_m=1000&categories=landmark,natural
```

**Response:**
```json
{
  "pois": [
    {
      "id": "poi_12345",
      "name": "Grand Canyon South Rim",
      "name_local": null,
      "category": "natural_landmark",
      "subcategory": "canyon",
      "lat": 36.1055,
      "lon": -112.1125,
      "distance_m": 150,
      "bearing_deg": 180,
      "facts": {
        "established": "1919",
        "depth_m": 1857,
        "unesco_site": true
      },
      "tags": {
        "tourism": "attraction",
        "natural": "canyon"
      }
    }
  ],
  "meta": {
    "total": 1,
    "center": {"lat": 36.1069, "lon": -112.1129},
    "radius_m": 1000
  }
}
```

---

### AI Narration

#### `POST /v1/narrate`

Generate fact-checked AI narration.

**Request:**
```json
{
  "truth_bundle": {
    "events": [
      {
        "id": "evt_001",
        "start_time": "2024-01-15T10:30:00Z",
        "end_time": "2024-01-15T10:32:00Z",
        "location": {
          "road_name": "AZ-64",
          "lat": 36.1069,
          "lon": -112.1129
        },
        "pois": [
          {
            "name": "Grand Canyon South Rim",
            "category": "natural_landmark",
            "distance_m": 150,
            "facts": {
              "established": "1919",
              "depth_m": 1857
            }
          }
        ]
      }
    ]
  },
  "transcript": "Wow, look at that view! This is incredible...",
  "options": {
    "style": "documentary",
    "include_chapters": true,
    "include_script": true,
    "language": "en"
  }
}
```

**Style Options:**
- `documentary` - Informative, narrator-style
- `casual` - Friendly, conversational
- `educational` - Detailed facts and history
- `minimal` - Brief, just the essentials

**Response:**
```json
{
  "chapters": [
    {
      "time_code": "00:00:00",
      "title": "Arriving at the Grand Canyon",
      "description": "First glimpse of the South Rim"
    },
    {
      "time_code": "00:02:00",
      "title": "South Rim Viewpoint",
      "description": "Standing at the edge of one of the world's greatest natural wonders"
    }
  ],
  "script": {
    "full_text": "We've arrived at the Grand Canyon South Rim, a viewpoint that has captivated visitors since the park was established in 1919. At nearly 1,857 meters deep, the canyon stretches as far as the eye can see...",
    "segments": [
      {
        "start_time": "00:00:00",
        "end_time": "00:01:30",
        "text": "We've arrived at the Grand Canyon South Rim..."
      }
    ]
  },
  "meta": {
    "model": "gemini-1.5-pro",
    "tokens_used": 450,
    "processing_time_ms": 2500,
    "grounding_confidence": 0.95
  }
}
```

---

#### `POST /v1/narrate/chapters`

Generate only YouTube-style chapters.

**Request:**
```json
{
  "events": [
    {
      "time_code": "00:04:15",
      "location": "Bixby Creek Bridge",
      "event_type": "landmark"
    },
    {
      "time_code": "00:12:30",
      "location": "Big Sur State Park Entrance",
      "event_type": "stop"
    }
  ],
  "options": {
    "format": "youtube",
    "include_timestamps": true
  }
}
```

**Response:**
```json
{
  "chapters": [
    {
      "time_code": "00:04:15",
      "title": "Crossing Bixby Creek Bridge",
      "formatted": "04:15 - Crossing Bixby Creek Bridge"
    },
    {
      "time_code": "00:12:30",
      "title": "Arriving at Big Sur State Park",
      "formatted": "12:30 - Arriving at Big Sur State Park"
    }
  ],
  "chapters_txt": "04:15 - Crossing Bixby Creek Bridge\n12:30 - Arriving at Big Sur State Park"
}
```

---

### Export

#### `POST /v1/export/srt`

Generate SRT subtitles with location data.

**Request:**
```json
{
  "events": [
    {
      "start_time": "00:00:01,000",
      "end_time": "00:00:05,000",
      "text": "Driving through the Arizona desert",
      "location": "US-89, Arizona"
    }
  ]
}
```

**Response:**
```
1
00:00:01,000 --> 00:00:05,000
Driving through the Arizona desert
📍 US-89, Arizona

```

---

## 📊 Rate Limits

| Endpoint | Limit |
|----------|-------|
| `/v1/enrich` | 100/min |
| `/v1/enrich_batch` | 1000 points/min |
| `/v1/map_match` | 50/min |
| `/v1/pois` | 200/min |
| `/v1/narrate` | 20/min |

Rate limit headers are included in responses:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1705312800
```

---

## 🚨 Error Codes

| Code | Status | Description |
|------|--------|-------------|
| `INVALID_COORDINATES` | 400 | Latitude/longitude out of bounds |
| `INVALID_TIMESTAMP` | 400 | Timestamp format incorrect |
| `UNAUTHORIZED` | 401 | Missing or invalid token |
| `FORBIDDEN` | 403 | Insufficient permissions |
| `NOT_FOUND` | 404 | Resource not found |
| `RATE_LIMITED` | 429 | Too many requests |
| `MAP_MATCH_FAILED` | 422 | Could not match GPS to roads |
| `AI_ERROR` | 502 | AI service unavailable |
| `INTERNAL_ERROR` | 500 | Unexpected server error |

**Error Response Format:**
```json
{
  "error": {
    "code": "INVALID_COORDINATES",
    "message": "Latitude must be between -90 and 90",
    "details": {
      "field": "lat",
      "value": 999.0
    }
  },
  "request_id": "req_abc123"
}
```

---

## 📚 SDKs & Examples

### Python

```python
import httpx

class GeoTruthClient:
    def __init__(self, base_url: str, api_key: str):
        self.base_url = base_url
        self.client = httpx.AsyncClient(
            headers={"Authorization": f"Bearer {api_key}"}
        )
    
    async def enrich(self, lat: float, lon: float) -> dict:
        response = await self.client.post(
            f"{self.base_url}/v1/enrich",
            json={"lat": lat, "lon": lon}
        )
        response.raise_for_status()
        return response.json()

# Usage
client = GeoTruthClient("https://api.geotruth.io", "your_api_key")
result = await client.enrich(36.1069, -112.1129)
```

### TypeScript

```typescript
class GeoTruthClient {
  private baseUrl: string;
  private apiKey: string;

  constructor(baseUrl: string, apiKey: string) {
    this.baseUrl = baseUrl;
    this.apiKey = apiKey;
  }

  async enrich(lat: number, lon: number): Promise<EnrichResponse> {
    const response = await fetch(`${this.baseUrl}/v1/enrich`, {
      method: 'POST',
      headers: {
        'Authorization': `Bearer ${this.apiKey}`,
        'Content-Type': 'application/json',
      },
      body: JSON.stringify({ lat, lon }),
    });

    if (!response.ok) {
      throw new Error(`API error: ${response.status}`);
    }

    return response.json();
  }
}
```

---

## 📚 Related Documentation

- [Backend Services](../backend/README.md)
- [Architecture Overview](../architecture/README.md)
- [Security Guidelines](../security/README.md)
