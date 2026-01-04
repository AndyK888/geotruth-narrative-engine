use crate::services::Ffmpeg;
use std::path::PathBuf;
use tauri::{State, Manager}; // Import Manager
use std::sync::Arc;

/// Capture a frame from a video at the specified timestamp in milliseconds.
/// Returns a base64 encoded data URI string of the image (JPEG).
#[tauri::command]
pub async fn capture_frame(
    video_path: String,
    timestamp_ms: u64,
    ffmpeg: State<'_, Arc<Ffmpeg>>,
) -> Result<String, String> {
    let video_path = PathBuf::from(video_path);
    
    // Check if file exists
    if !video_path.exists() {
        return Err(format!("Video file not found: {:?}", video_path));
    }

    ffmpeg.capture_frame(&video_path, timestamp_ms)
        .await
        .map_err(|e| e.to_string())
}

#[derive(serde::Serialize)]
pub struct ScannedMoment {
    pub timestamp: f64,
    pub image_path: String,
}

/// Automatically scan the video and extract moments (keyframes/thumbnails) at intervals.
#[tauri::command]
pub async fn auto_scan_moments(
    video_path: String,
    ffmpeg: State<'_, Arc<Ffmpeg>>,
    app_handle: tauri::AppHandle,
) -> Result<Vec<ScannedMoment>, String> {
    let video_path = PathBuf::from(video_path);
    if !video_path.exists() {
        return Err(format!("Video file not found: {:?}", video_path));
    }

    // Create a unique directory for this scan in temp or app_cache
    let file_stem = video_path.file_stem().unwrap_or_default().to_string_lossy();
    let cache_dir = app_handle.path().app_cache_dir().map_err(|e: tauri::Error| e.to_string())?;
    let output_dir = cache_dir.join("moments").join(&*file_stem);
    
    if !output_dir.exists() {
        std::fs::create_dir_all(&output_dir).map_err(|e| e.to_string())?;
    }

    // Extract key moments using scene detection (threshold 0.4)
    let thumbnails = ffmpeg.extract_key_moments(&video_path, &output_dir, 0.4)
        .await
        .map_err(|e| e.to_string())?;

    // Map paths to moments
    let moments = thumbnails.into_iter().map(|m| ScannedMoment {
        timestamp: m.timestamp,
        image_path: m.path.to_string_lossy().to_string(),
    }).collect();

    Ok(moments)
}
