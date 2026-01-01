//! GPS Track Parser
//!
//! Parses GPX, NMEA, and other GPS file formats.

use std::path::PathBuf;
use std::fs::File;
use std::io::{BufRead, BufReader};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, TimeZone, NaiveDateTime};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Error, Debug)]
pub enum GpsError {
    #[error("Failed to read file: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Failed to parse GPX: {0}")]
    GpxParseError(String),
    
    #[error("Failed to parse NMEA: {0}")]
    NmeaParseError(String),
    
    #[error("Unknown file format")]
    UnknownFormat,
    
    #[error("No GPS points found")]
    NoPoints,
}

/// GPS track point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsPoint {
    pub timestamp: DateTime<Utc>,
    pub lat: f64,
    pub lon: f64,
    pub elevation_m: Option<f64>,
    pub speed_kmh: Option<f64>,
    pub heading_deg: Option<f64>,
    pub accuracy_m: Option<f64>,
}

/// GPS track metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsTrack {
    pub name: Option<String>,
    pub source_file: String,
    pub track_type: String,
    pub point_count: usize,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
    pub bounds: Option<GpsBounds>,
    pub points: Vec<GpsPoint>,
}

/// Bounding box for GPS track
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsBounds {
    pub min_lat: f64,
    pub max_lat: f64,
    pub min_lon: f64,
    pub max_lon: f64,
}

/// Parse GPS file and return track
pub async fn parse_gps_file(path: &PathBuf) -> Result<GpsTrack, GpsError> {
    let extension = path.extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());
    
    match extension.as_deref() {
        Some("gpx") => parse_gpx(path).await,
        Some("nmea") | Some("log") | Some("txt") => parse_nmea(path).await,
        _ => {
            // Try to detect format from content
            let content = std::fs::read_to_string(path)?;
            if content.contains("<gpx") {
                parse_gpx(path).await
            } else if content.contains("$GPRMC") || content.contains("$GPGGA") {
                parse_nmea(path).await
            } else {
                Err(GpsError::UnknownFormat)
            }
        }
    }
}

/// Parse GPX file
async fn parse_gpx(path: &PathBuf) -> Result<GpsTrack, GpsError> {
    debug!("Parsing GPX file: {:?}", path);
    
    let content = std::fs::read_to_string(path)?;
    let mut points = Vec::new();
    let mut name = None;
    
    // Simple GPX parser (for production, use a proper XML parser)
    // This handles basic GPX 1.1 format
    
    // Extract track name
    if let Some(start) = content.find("<name>") {
        if let Some(end) = content[start..].find("</name>") {
            name = Some(content[start + 6..start + end].to_string());
        }
    }
    
    // Parse track points
    for segment in content.split("<trkpt").skip(1) {
        if let Some(point) = parse_gpx_point(segment) {
            points.push(point);
        }
    }
    
    // Also parse waypoints
    for segment in content.split("<wpt").skip(1) {
        if let Some(point) = parse_gpx_point(segment) {
            points.push(point);
        }
    }
    
    if points.is_empty() {
        return Err(GpsError::NoPoints);
    }
    
    // Sort by timestamp
    points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    
    // Calculate bounds
    let bounds = calculate_bounds(&points);
    
    info!("Parsed {} GPS points from GPX", points.len());
    
    Ok(GpsTrack {
        name,
        source_file: path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        track_type: "gpx".to_string(),
        point_count: points.len(),
        start_time: points.first().map(|p| p.timestamp),
        end_time: points.last().map(|p| p.timestamp),
        bounds: Some(bounds),
        points,
    })
}

/// Parse a single GPX track point
fn parse_gpx_point(segment: &str) -> Option<GpsPoint> {
    // Extract lat
    let lat_start = segment.find("lat=\"")? + 5;
    let lat_end = segment[lat_start..].find('"')? + lat_start;
    let lat: f64 = segment[lat_start..lat_end].parse().ok()?;
    
    // Extract lon
    let lon_start = segment.find("lon=\"")? + 5;
    let lon_end = segment[lon_start..].find('"')? + lon_start;
    let lon: f64 = segment[lon_start..lon_end].parse().ok()?;
    
    // Extract elevation
    let elevation_m = segment.find("<ele>")
        .and_then(|start| {
            let end = segment[start..].find("</ele>")?;
            segment[start + 5..start + end].parse().ok()
        });
    
    // Extract time
    let timestamp = segment.find("<time>")
        .and_then(|start| {
            let end = segment[start..].find("</time>")?;
            let time_str = &segment[start + 6..start + end];
            DateTime::parse_from_rfc3339(time_str).ok()
                .map(|dt| dt.with_timezone(&Utc))
        })
        .unwrap_or_else(Utc::now);
    
    Some(GpsPoint {
        timestamp,
        lat,
        lon,
        elevation_m,
        speed_kmh: None,
        heading_deg: None,
        accuracy_m: None,
    })
}

/// Parse NMEA file
async fn parse_nmea(path: &PathBuf) -> Result<GpsTrack, GpsError> {
    debug!("Parsing NMEA file: {:?}", path);
    
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut points = Vec::new();
    
    for line in reader.lines() {
        let line = line?;
        
        // Parse GPRMC sentences (most common)
        if line.starts_with("$GPRMC") || line.starts_with("$GNRMC") {
            if let Some(point) = parse_nmea_rmc(&line) {
                points.push(point);
            }
        }
        // Parse GPGGA sentences (has elevation)
        else if line.starts_with("$GPGGA") || line.starts_with("$GNGGA") {
            if let Some(point) = parse_nmea_gga(&line) {
                points.push(point);
            }
        }
    }
    
    if points.is_empty() {
        return Err(GpsError::NoPoints);
    }
    
    // Sort and deduplicate by timestamp
    points.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    
    let bounds = calculate_bounds(&points);
    
    info!("Parsed {} GPS points from NMEA", points.len());
    
    Ok(GpsTrack {
        name: None,
        source_file: path.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        track_type: "nmea".to_string(),
        point_count: points.len(),
        start_time: points.first().map(|p| p.timestamp),
        end_time: points.last().map(|p| p.timestamp),
        bounds: Some(bounds),
        points,
    })
}

/// Parse NMEA RMC sentence
fn parse_nmea_rmc(line: &str) -> Option<GpsPoint> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 10 {
        return None;
    }
    
    // Check validity
    if parts[2] != "A" {
        return None; // Invalid fix
    }
    
    // Parse time and date
    let time_str = parts[1];
    let date_str = parts[9];
    
    if time_str.len() < 6 || date_str.len() < 6 {
        return None;
    }
    
    let hour: u32 = time_str[0..2].parse().ok()?;
    let min: u32 = time_str[2..4].parse().ok()?;
    let sec: u32 = time_str[4..6].parse().ok()?;
    
    let day: u32 = date_str[0..2].parse().ok()?;
    let month: u32 = date_str[2..4].parse().ok()?;
    let year: i32 = 2000 + date_str[4..6].parse::<i32>().ok()?;
    
    let naive = NaiveDateTime::parse_from_str(
        &format!("{:04}-{:02}-{:02} {:02}:{:02}:{:02}", year, month, day, hour, min, sec),
        "%Y-%m-%d %H:%M:%S"
    ).ok()?;
    
    let timestamp = Utc.from_utc_datetime(&naive);
    
    // Parse latitude
    let lat_raw: f64 = parts[3].parse().ok()?;
    let lat_deg = (lat_raw / 100.0).floor();
    let lat_min = lat_raw - (lat_deg * 100.0);
    let mut lat = lat_deg + (lat_min / 60.0);
    if parts[4] == "S" {
        lat = -lat;
    }
    
    // Parse longitude
    let lon_raw: f64 = parts[5].parse().ok()?;
    let lon_deg = (lon_raw / 100.0).floor();
    let lon_min = lon_raw - (lon_deg * 100.0);
    let mut lon = lon_deg + (lon_min / 60.0);
    if parts[6] == "W" {
        lon = -lon;
    }
    
    // Parse speed (knots to km/h)
    let speed_kmh = parts.get(7)
        .and_then(|s| s.parse::<f64>().ok())
        .map(|knots| knots * 1.852);
    
    // Parse heading
    let heading_deg = parts.get(8)
        .and_then(|s| s.parse::<f64>().ok());
    
    Some(GpsPoint {
        timestamp,
        lat,
        lon,
        elevation_m: None,
        speed_kmh,
        heading_deg,
        accuracy_m: None,
    })
}

/// Parse NMEA GGA sentence
fn parse_nmea_gga(line: &str) -> Option<GpsPoint> {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() < 10 {
        return None;
    }
    
    // Check fix quality
    let fix_quality: u32 = parts[6].parse().ok()?;
    if fix_quality == 0 {
        return None; // No fix
    }
    
    // Parse time only (no date in GGA)
    let time_str = parts[1];
    if time_str.len() < 6 {
        return None;
    }
    
    let hour: u32 = time_str[0..2].parse().ok()?;
    let min: u32 = time_str[2..4].parse().ok()?;
    let sec: u32 = time_str[4..6].parse().ok()?;
    
    // Use today's date (will need to be merged with RMC for accurate date)
    let today = Utc::now().date_naive();
    let naive = today.and_hms_opt(hour, min, sec)?;
    let timestamp = Utc.from_utc_datetime(&naive);
    
    // Parse latitude
    let lat_raw: f64 = parts[2].parse().ok()?;
    let lat_deg = (lat_raw / 100.0).floor();
    let lat_min = lat_raw - (lat_deg * 100.0);
    let mut lat = lat_deg + (lat_min / 60.0);
    if parts[3] == "S" {
        lat = -lat;
    }
    
    // Parse longitude
    let lon_raw: f64 = parts[4].parse().ok()?;
    let lon_deg = (lon_raw / 100.0).floor();
    let lon_min = lon_raw - (lon_deg * 100.0);
    let mut lon = lon_deg + (lon_min / 60.0);
    if parts[5] == "W" {
        lon = -lon;
    }
    
    // Parse elevation
    let elevation_m = parts.get(9)
        .and_then(|s| s.parse::<f64>().ok());
    
    Some(GpsPoint {
        timestamp,
        lat,
        lon,
        elevation_m,
        speed_kmh: None,
        heading_deg: None,
        accuracy_m: None,
    })
}

/// Calculate bounding box for points
fn calculate_bounds(points: &[GpsPoint]) -> GpsBounds {
    let min_lat = points.iter().map(|p| p.lat).fold(f64::INFINITY, f64::min);
    let max_lat = points.iter().map(|p| p.lat).fold(f64::NEG_INFINITY, f64::max);
    let min_lon = points.iter().map(|p| p.lon).fold(f64::INFINITY, f64::min);
    let max_lon = points.iter().map(|p| p.lon).fold(f64::NEG_INFINITY, f64::max);
    
    GpsBounds {
        min_lat,
        max_lat,
        min_lon,
        max_lon,
    }
}
