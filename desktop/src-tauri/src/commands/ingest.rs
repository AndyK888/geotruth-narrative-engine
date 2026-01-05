//! Video Ingestion Commands
//!
//! Tauri commands for importing and managing videos.

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use tauri::{State, AppHandle, Emitter};
use tracing::{info, debug, error};
use tokio::sync::Mutex;

use crate::services::{Ffmpeg, parse_gps_file, LocalDatabase, GpsTrack};

/// Application state
#[allow(dead_code)]
pub struct AppState {
    pub db: Mutex<Option<LocalDatabase>>,
    pub ffmpeg: Mutex<Option<Ffmpeg>>,
}

/// Import progress event payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportProgress {
    pub stage: String,
    pub progress: u8,
    pub message: String,
}

/// Video import result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportResult {
    pub video_id: String,
    pub project_id: String,
    pub filename: String,
    pub duration_seconds: Option<f64>,
    pub fps: Option<f64>,
    pub resolution: Option<String>,
    pub has_audio: bool,
    pub gps_track: Option<GpsTrackSummary>,
}

/// GPS track summary for frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsTrackSummary {
    pub point_count: usize,
    pub duration_seconds: Option<f64>,
    pub distance_km: Option<f64>,
}

/// Import a video file with optional GPS track
#[tauri::command]
pub async fn import_video(
    app: AppHandle,
    db: State<'_, LocalDatabase>,
    ffmpeg_state: State<'_, AppState>,
    project_id: String,
    video_path: String,
    gps_path: Option<String>,
) -> Result<ImportResult, String> {
    info!("Importing video: {} to project {}", video_path, project_id);
    
    let video_path_buf = PathBuf::from(&video_path);
    
    // Check file exists
    if !video_path_buf.exists() {
        return Err(format!("Video file not found: {:?}", video_path_buf));
    }
    
    // Emit: Starting
    let _ = app.emit("import-progress", ImportProgress {
        stage: "start".into(),
        progress: 0,
        message: "Starting import...".into(),
    });
    
    // Emit: Extracting metadata
    let _ = app.emit("import-progress", ImportProgress {
        stage: "metadata".into(),
        progress: 20,
        message: "Extracting video metadata...".into(),
    });
    
    // Extract metadata with FFmpeg
    let metadata = {
        let ffmpeg_guard = ffmpeg_state.ffmpeg.lock().await;
        if let Some(ref ffmpeg) = *ffmpeg_guard {
            match ffmpeg.extract_metadata(&video_path_buf).await {
                Ok(m) => Some(m),
                Err(e) => {
                    error!("Failed to extract metadata: {}", e);
                    None
                }
            }
        } else {
            error!("FFmpeg not initialized in state");
            None
        }
    };
    
    // Emit: GPS parsing
    let _ = app.emit("import-progress", ImportProgress {
        stage: "gps".into(),
        progress: 50,
        message: "Parsing GPS data...".into(),
    });
    
    // Parse GPS track if provided
    let gps_track = if let Some(gps_path_str) = gps_path {
        let gps_path = PathBuf::from(&gps_path_str);
        match parse_gps_file(&gps_path).await {
            Ok(track) => {
                let duration = match (&track.start_time, &track.end_time) {
                    (Some(start), Some(end)) => {
                        Some((*end - *start).num_seconds() as f64)
                    }
                    _ => None
                };
                
                Some(GpsTrackSummary {
                    point_count: track.point_count,
                    duration_seconds: duration,
                    distance_km: calculate_track_distance(&track),
                })
            }
            Err(e) => {
                error!("Failed to parse GPS: {}", e);
                None
            }
        }
    } else {
        None
    };
    
    // Emit: Database
    let _ = app.emit("import-progress", ImportProgress {
        stage: "database".into(),
        progress: 80,
        message: "Saving to database...".into(),
    });
    
    // Store in database
    let video_id = {
        let filename = video_path_buf.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        
        let video_metadata = metadata.as_ref().map(|m| {
            crate::services::database::VideoMetadata {
                duration_seconds: m.duration_seconds,
                fps: m.fps,
                width: m.width,
                height: m.height,
                codec: m.codec.clone(),
                file_size_bytes: m.file_size_bytes.map(|s| s as i64),
            }
        });
        
        match db.add_video(
            &project_id,
            &filename,
            &video_path_buf.to_string_lossy(),
            video_metadata,
        ).await {
            Ok(video) => video.id,
            Err(e) => return Err(format!("Database error: {}", e)),
        }
    };
    
    let resolution = metadata.as_ref()
        .and_then(|m| {
            match (m.width, m.height) {
                (Some(w), Some(h)) => Some(format!("{}x{}", w, h)),
                _ => None
            }
        });
    
    // Emit: Complete
    let _ = app.emit("import-progress", ImportProgress {
        stage: "complete".into(),
        progress: 100,
        message: "Import complete!".into(),
    });
    
    info!("Video imported successfully: {}", video_id);
    
    Ok(ImportResult {
        video_id,
        project_id: project_id.clone(),
        filename: video_path_buf.file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        duration_seconds: metadata.as_ref().and_then(|m| m.duration_seconds),
        fps: metadata.as_ref().and_then(|m| m.fps),
        resolution,
        has_audio: metadata.as_ref().map(|m| m.has_audio).unwrap_or(false),
        gps_track,
    })
}

/// Calculate total distance of GPS track in kilometers
fn calculate_track_distance(track: &GpsTrack) -> Option<f64> {
    if track.points.len() < 2 {
        return None;
    }
    
    let mut total_distance = 0.0;
    
    for i in 1..track.points.len() {
        let p1 = &track.points[i - 1];
        let p2 = &track.points[i];
        total_distance += haversine_distance(p1.lat, p1.lon, p2.lat, p2.lon);
    }
    
    Some(total_distance)
}

/// Calculate distance between two GPS points using Haversine formula
fn haversine_distance(lat1: f64, lon1: f64, lat2: f64, lon2: f64) -> f64 {
    const R: f64 = 6371.0; // Earth radius in km
    
    let lat1_rad = lat1.to_radians();
    let lat2_rad = lat2.to_radians();
    let delta_lat = (lat2 - lat1).to_radians();
    let delta_lon = (lon2 - lon1).to_radians();
    
    let a = (delta_lat / 2.0).sin().powi(2)
        + lat1_rad.cos() * lat2_rad.cos() * (delta_lon / 2.0).sin().powi(2);
    let c = 2.0 * a.sqrt().asin();
    
    R * c
}

/// Get project videos
#[tauri::command]
pub async fn get_project_videos(
    db: State<'_, LocalDatabase>,
    project_id: String,
) -> Result<Vec<crate::services::database::Video>, String> {
    debug!("Getting videos for project: {}", project_id);
    
    db.get_project_videos(&project_id)
        .await
        .map_err(|e| format!("Database error: {}", e))
}

/// Create a new project
#[tauri::command]
pub async fn create_project(
    db: State<'_, LocalDatabase>,
    name: String,
    description: Option<String>,
) -> Result<crate::services::database::Project, String> {
    info!("Creating project: {}", name);
    
    db.create_project(&name, description.as_deref())
        .await
        .map_err(|e| format!("Database error: {}", e))
}

/// Get all projects
#[tauri::command]
pub async fn get_projects(
    db: State<'_, LocalDatabase>,
) -> Result<Vec<crate::services::database::Project>, String> {
    debug!("Getting all projects");
    
    db.get_projects()
        .await
        .map_err(|e| format!("Database error: {}", e))
}
