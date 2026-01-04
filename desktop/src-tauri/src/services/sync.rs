//! Time Synchronization Engine
//!
//! Aligns video timestamps with GPS track data.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info};

use super::gps::{GpsPoint, GpsTrack};

#[derive(Error, Debug)]
pub enum SyncError {
    #[error("No GPS points available")]
    NoGpsPoints,
    
    #[error("Video metadata missing")]
    NoVideoMetadata,
    
    #[error("Time ranges don't overlap")]
    NoOverlap,
    
    #[error("Sync failed: {0}")]
    SyncFailed(String),
}

/// Synchronization result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncResult {
    pub offset_seconds: f64,
    pub confidence: f64,
    pub method: SyncMethod,
    pub aligned_points: Vec<AlignedPoint>,
}

/// Method used for synchronization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncMethod {
    /// Based on video creation time metadata
    VideoMetadata,
    /// Based on first GPS point timestamp
    FirstGpsPoint,
    /// User-provided offset
    Manual,
    /// AI-detected sync point (future)
    AutoDetect,
}

/// A GPS point aligned to video time
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlignedPoint {
    pub video_time_seconds: f64,
    pub gps: GpsPoint,
}

/// Time sync engine
pub struct TimeSyncEngine {
    gps_track: GpsTrack,
    video_duration_seconds: f64,
    video_start_time: Option<DateTime<Utc>>,
}

impl TimeSyncEngine {
    /// Create new sync engine
    pub fn new(
        gps_track: GpsTrack,
        video_duration_seconds: f64,
        video_start_time: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            gps_track,
            video_duration_seconds,
            video_start_time,
        }
    }
    
    /// Synchronize GPS track to video timeline
    pub fn synchronize(&self) -> Result<SyncResult, SyncError> {
        if self.gps_track.points.is_empty() {
            return Err(SyncError::NoGpsPoints);
        }
        
        // Try different sync methods
        if let Some(result) = self.sync_by_video_metadata() {
            return Ok(result);
        }
        
        // Fall back to first GPS point
        self.sync_by_first_point()
    }
    
    /// Sync using video creation time metadata
    fn sync_by_video_metadata(&self) -> Option<SyncResult> {
        let video_start = self.video_start_time?;
        let gps_start = self.gps_track.start_time?;
        
        let offset = (gps_start - video_start).num_milliseconds() as f64 / 1000.0;
        
        debug!("Video metadata sync: offset = {} seconds", offset);
        
        let aligned_points = self.align_points(offset);
        
        if aligned_points.is_empty() {
            return None;
        }
        
        Some(SyncResult {
            offset_seconds: offset,
            confidence: 0.9,
            method: SyncMethod::VideoMetadata,
            aligned_points,
        })
    }
    
    /// Sync assuming first GPS point is at video start
    fn sync_by_first_point(&self) -> Result<SyncResult, SyncError> {
        let gps_start = self.gps_track.start_time
            .ok_or(SyncError::NoGpsPoints)?;
        
        // Offset is 0 - GPS starts at video start
        let offset = 0.0;
        
        let aligned_points = self.align_points_from_start(gps_start);
        
        if aligned_points.is_empty() {
            return Err(SyncError::NoOverlap);
        }
        
        info!("First point sync: {} aligned points", aligned_points.len());
        
        Ok(SyncResult {
            offset_seconds: offset,
            confidence: 0.5, // Lower confidence for this method
            method: SyncMethod::FirstGpsPoint,
            aligned_points,
        })
    }
    
    /// Align GPS points to video timeline with offset
    fn align_points(&self, offset_seconds: f64) -> Vec<AlignedPoint> {
        let video_start = match self.video_start_time {
            Some(t) => t,
            None => return vec![],
        };
        
        self.gps_track.points
            .iter()
            .filter_map(|point| {
                let point_offset = (point.timestamp - video_start).num_milliseconds() as f64 / 1000.0;
                let video_time = point_offset - offset_seconds;
                
                // Only include points within video duration
                if video_time >= 0.0 && video_time <= self.video_duration_seconds {
                    Some(AlignedPoint {
                        video_time_seconds: video_time,
                        gps: point.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Align points assuming GPS track starts at video start
    fn align_points_from_start(&self, gps_start: DateTime<Utc>) -> Vec<AlignedPoint> {
        self.gps_track.points
            .iter()
            .filter_map(|point| {
                let video_time = (point.timestamp - gps_start).num_milliseconds() as f64 / 1000.0;
                
                if video_time >= 0.0 && video_time <= self.video_duration_seconds {
                    Some(AlignedPoint {
                        video_time_seconds: video_time,
                        gps: point.clone(),
                    })
                } else {
                    None
                }
            })
            .collect()
    }
    
    /// Get GPS point at specific video time
    pub fn get_point_at_time(&self, sync_result: &SyncResult, video_time_seconds: f64) -> Option<GpsPoint> {
        // Find closest aligned point
        sync_result.aligned_points
            .iter()
            .min_by(|a, b| {
                let diff_a = (a.video_time_seconds - video_time_seconds).abs();
                let diff_b = (b.video_time_seconds - video_time_seconds).abs();
                diff_a.partial_cmp(&diff_b).unwrap()
            })
            .map(|p| p.gps.clone())
    }
    
    /// Interpolate GPS position at specific video time
    pub fn interpolate_position(
        &self, 
        sync_result: &SyncResult, 
        video_time_seconds: f64
    ) -> Option<(f64, f64, Option<f64>)> {
        if sync_result.aligned_points.is_empty() {
            return None;
        }
        
        // Find bracketing points
        let mut before: Option<&AlignedPoint> = None;
        let mut after: Option<&AlignedPoint> = None;
        
        for point in &sync_result.aligned_points {
            if point.video_time_seconds <= video_time_seconds {
                before = Some(point);
            }
            if point.video_time_seconds > video_time_seconds && after.is_none() {
                after = Some(point);
                break;
            }
        }
        
        match (before, after) {
            (Some(b), Some(a)) => {
                // Linear interpolation
                let t = (video_time_seconds - b.video_time_seconds) 
                    / (a.video_time_seconds - b.video_time_seconds);
                
                let lat = b.gps.lat + t * (a.gps.lat - b.gps.lat);
                let lon = b.gps.lon + t * (a.gps.lon - b.gps.lon);
                let heading = match (b.gps.heading_deg, a.gps.heading_deg) {
                    (Some(h1), Some(h2)) => Some(h1 + t * (h2 - h1)),
                    (Some(h), None) | (None, Some(h)) => Some(h),
                    _ => None,
                };
                
                Some((lat, lon, heading))
            }
            (Some(b), None) => Some((b.gps.lat, b.gps.lon, b.gps.heading_deg)),
            (None, Some(a)) => Some((a.gps.lat, a.gps.lon, a.gps.heading_deg)),
            (None, None) => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_interpolation() {
        // Create test points
        let points = vec![
            GpsPoint {
                timestamp: Utc::now(),
                lat: 36.0,
                lon: -112.0,
                elevation_m: None,
                speed_kmh: None,
                heading_deg: Some(90.0),
                accuracy_m: None,
            },
            GpsPoint {
                timestamp: Utc::now() + Duration::seconds(10),
                lat: 36.1,
                lon: -112.1,
                elevation_m: None,
                speed_kmh: None,
                heading_deg: Some(180.0),
                accuracy_m: None,
            },
        ];
        
        let track = GpsTrack {
            name: None,
            source_file: "test.gpx".to_string(),
            track_type: "gpx".to_string(),
            point_count: 2,
            start_time: Some(points[0].timestamp),
            end_time: Some(points[1].timestamp),
            bounds: None,
            points: points.clone(),
        };
        
        let engine = TimeSyncEngine::new(track, 10.0, Some(points[0].timestamp));
        
        // Sync should work
        let result = engine.synchronize();
        assert!(result.is_ok());
    }
}
