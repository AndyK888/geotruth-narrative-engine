use crate::services::Ffmpeg;
use std::path::PathBuf;
use tauri::State;
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
