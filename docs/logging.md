# Logging Guide

GeoTruth uses comprehensive **structured JSON logging** across all components for debugging and observability.

---

## 🎯 Logging Philosophy

1. **Structured JSON** - All logs are machine-parseable
2. **Correlation IDs** - Track requests across services
3. **Context-Rich** - Include relevant data in every log
4. **Level-Based** - Filter by severity as needed
5. **Unified** - Same format in backend, desktop, and frontend

---

## 📊 Log Format

All logs follow this structure:

```json
{
  "timestamp": "2024-01-15T10:30:00.123456Z",
  "level": "INFO",
  "service": "api",
  "logger": "app.services.enrichment",
  "message": "Human-readable message",
  "correlation_id": "req-abc12345",
  "context": {
    "key": "value",
    "duration_ms": 45
  },
  "source": {
    "file": "/app/services/enrichment.py",
    "line": 89,
    "function": "enrich_point"
  }
}
```

---

## 🔗 Correlation IDs

Every request gets a unique correlation ID that flows through all services:

```
Desktop App                     Backend API                    Database
    │                              │                              │
    │  X-Correlation-ID: abc123    │                              │
    ├─────────────────────────────►│                              │
    │                              │  correlation_id: abc123       │
    │                              ├─────────────────────────────►│
    │                              │                              │
    │                              │◄─────────────────────────────┤
    │◄─────────────────────────────┤                              │
    │                              │                              │
```

### Searching Logs by Correlation ID

```bash
# Backend (Docker)
docker compose logs api | grep "abc123"

# Desktop (file)
grep "abc123" ~/Library/Application\ Support/GeoTruth/logs/*.log

# Using jq for structured parsing
docker compose logs api --no-log-prefix | jq 'select(.correlation_id == "abc123")'
```

---

## 📝 Log Levels

| Level | When to Use | Example |
|-------|-------------|---------|
| **DEBUG** | Detailed diagnostic info | `Parsing GPS point 1 of 15234` |
| **INFO** | Normal operations | `Enrichment completed for 3 POIs` |
| **WARN** | Unexpected but handled | `Cache miss, querying database` |
| **ERROR** | Failures requiring attention | `Database connection failed` |

### Configuring Log Level

**Backend:**
```bash
# Environment variable
LOG_LEVEL=DEBUG docker compose up
```

**Desktop:**
```bash
# Environment variable
RUST_LOG=debug ./GeoTruth

# Or in app settings
Settings → Developer → Log Level → Debug
```

---

## 🐳 Backend Logging

### Configuration

```python
# app/config.py
class Settings:
    log_level: str = "INFO"
    log_format: str = "json"  # "json" or "pretty"
```

### Example Logs

**Request lifecycle:**
```json
{"timestamp":"2024-01-15T10:30:00.123Z","level":"INFO","message":"Request started","correlation_id":"abc123","context":{"method":"POST","path":"/v1/enrich","client_ip":"172.18.0.1"}}
{"timestamp":"2024-01-15T10:30:00.125Z","level":"DEBUG","message":"Checking cache","correlation_id":"abc123","context":{"cache_key":"enrich:36.1069:-112.1129"}}
{"timestamp":"2024-01-15T10:30:00.126Z","level":"DEBUG","message":"Cache miss","correlation_id":"abc123"}
{"timestamp":"2024-01-15T10:30:00.145Z","level":"DEBUG","message":"PostGIS query completed","correlation_id":"abc123","context":{"query_ms":19,"rows":3}}
{"timestamp":"2024-01-15T10:30:00.150Z","level":"INFO","message":"Request completed","correlation_id":"abc123","context":{"status":200,"duration_ms":27}}
```

### Viewing Logs

```bash
# Follow all logs
docker compose logs -f

# Filter by service
docker compose logs -f api

# Filter by level (requires jq)
docker compose logs api --no-log-prefix | jq 'select(.level == "ERROR")'

# Filter by time range
docker compose logs api --since="2024-01-15T10:00:00" --until="2024-01-15T11:00:00"
```

---

## 🖥️ Desktop Logging

### Log File Location

| Platform | Path |
|----------|------|
| **macOS** | `~/Library/Application Support/GeoTruth/logs/` |
| **Windows** | `%APPDATA%\GeoTruth\logs\` |
| **Linux** | `~/.local/share/GeoTruth/logs/` |

### Log Rotation

- **Max file size**: 50MB per file
- **Max files**: 5 (rolling)
- **Naming**: `geotruth.2024-01-15.log`

### Example Logs

**Video import:**
```json
{"timestamp":"2024-01-15T10:30:00.123Z","level":"INFO","target":"geotruth::commands::ingest","message":"Starting video import","correlation_id":"vid-abc123","video_path":"/Users/john/Videos/roadtrip.mp4"}
{"timestamp":"2024-01-15T10:30:00.125Z","level":"DEBUG","target":"geotruth::services::sidecar","message":"Spawning FFprobe","binary":"/Applications/GeoTruth.app/Contents/Resources/binaries/ffprobe"}
{"timestamp":"2024-01-15T10:30:00.450Z","level":"INFO","target":"geotruth::services::sidecar","message":"FFprobe completed","duration_ms":325,"exit_code":0}
{"timestamp":"2024-01-15T10:30:00.455Z","level":"INFO","target":"geotruth::commands::ingest","message":"Video imported","video_id":"vid-xyz789","duration_secs":3823,"resolution":"3840x2160"}
```

### Opening Log Folder

In the app: **Help → Open Log Folder**

Or via menu: **Debug → Show Logs**

---

## 🌐 Frontend Logging

Browser console logs are also structured:

```javascript
// Development (pretty)
[INFO] Timeline: Event selected { eventId: 'abc123', timestamp: '00:45:30' }

// Production (JSON)
{"timestamp":"2024-01-15T10:30:00.123Z","level":"info","component":"Timeline","message":"Event selected","eventId":"abc123","timestamp":"00:45:30"}
```

### Enabling Debug Logs

**Development:**
```typescript
// Already enabled by default
```

**Production:**
Open DevTools: **View → Toggle Developer Tools** (Cmd+Option+I)

---

## 🔍 Debugging Common Issues

### Issue: Video Import Fails

**Look for:**
```bash
grep -i "error" ~/Library/Application\ Support/GeoTruth/logs/*.log | grep -i "ffprobe\|video\|import"
```

**Example error log:**
```json
{"level":"ERROR","message":"FFprobe execution failed","error":"sidecar not found","expected_path":"/Applications/GeoTruth.app/Contents/Resources/binaries/ffprobe"}
```

### Issue: GPS Sync Incorrect

**Look for:**
```bash
grep "offset\|sync\|time" ~/Library/Application\ Support/GeoTruth/logs/*.log
```

**Example debug log:**
```json
{"level":"DEBUG","message":"Calculated time offset","video_first_frame":"2024-01-15T10:30:05","gps_first_point":"2024-01-15T10:30:00","offset_seconds":5}
```

### Issue: API Enrichment Slow

**Look for:**
```bash
docker compose logs api | jq 'select(.message == "Request completed") | select(.context.duration_ms > 1000)'
```

**Example slow request:**
```json
{"level":"WARN","message":"Slow request detected","path":"/v1/enrich_batch","duration_ms":5234,"points_count":500}
```

---

## 📈 Log Aggregation

For production deployments, forward logs to centralized systems:

### Docker Logging Drivers

```yaml
# docker-compose.yml
services:
  api:
    logging:
      driver: "json-file"
      options:
        max-size: "100m"
        max-file: "5"
```

### Supported Backends

| Backend | Driver |
|---------|--------|
| **ELK Stack** | `json-file` + Filebeat |
| **Grafana Loki** | `loki` driver |
| **CloudWatch** | `awslogs` driver |
| **Datadog** | `datadog` driver |

---

## 🛠️ Troubleshooting Logging

### Logs Not Appearing

1. Check log level: `LOG_LEVEL=DEBUG`
2. Check log directory permissions
3. Check disk space

### Logs Too Verbose

1. Increase log level: `LOG_LEVEL=WARN`
2. Add filters for noisy loggers:
   ```python
   logging.getLogger("httpx").setLevel(logging.WARNING)
   ```

### JSON Parsing Issues

Use `jq` for reliable parsing:
```bash
docker compose logs api --no-log-prefix | jq '.'
```

---

## 📚 Related Documentation

- [Backend Services](backend/README.md)
- [Desktop Application](desktop/README.md)
- [Development Guide](development/README.md)
