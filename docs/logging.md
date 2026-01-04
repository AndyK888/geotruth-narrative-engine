# Logging Guide

GeoTruth uses **structured logging** for debugging and observability in its monolithic Native Desktop application.

## 🎯 Logging Philosophy

1.  **Unified**: Frontend and Backend logs are consolidated where possible (or at least consistent).
2.  **Structured**: Rust backend uses `tracing` for JSON/structured output.
3.  **Local**: Logs are stored in the user's data directory.

## 🖥 Backend Logging (Rust)

The Rust backend uses the `tracing` ecosystem.

### Log Levels

| Level | Usage |
|-------|-------|
| `ERROR` | Critical failures (DB corruption, API auth failure) |
| `WARN` | Recoverable issues (Network timeout, missing metadata) |
| `INFO` | High-level flow (App start, Project loaded, processing finished) |
| `DEBUG` | Detailed flow (SQL queries, API payloads) |
| `TRACE` | Byte-level details (Parsing loops) |

### Configuration

Log level is controlled by the `RUST_LOG` environment variable.

```bash
# Run with debug logs
RUST_LOG=debug ./GeoTruth
```

The application initializes `tracing-subscriber` which outputs to stdout (and optionally a file in the data directory).

## 🌐 Frontend Logging (React)

The frontend uses standard `console` logging, which in Tauri production builds is often stripped or directed to the system WebView console.

### Development
Logs appear in the Browser Console (Inspect Element).

### Production
For debugging production builds, we recommend using `tauri-plugin-log` (future enhancement) or relying on Backend logs for critical errors.

## 📂 Log Location

Logs are typically written to `stdout/stderr` when run from terminal.
For persistence, the application can be configured to write to:

-   **macOS**: `~/Library/Application Support/com.geotruth.app/logs/`
-   **Windows**: `%APPDATA%\com.geotruth.app\logs\`
-   **Linux**: `~/.local/share/com.geotruth.app/logs/`

*(Note: File logging must be explicitly enabled in `src/lib.rs` configuration)*.
