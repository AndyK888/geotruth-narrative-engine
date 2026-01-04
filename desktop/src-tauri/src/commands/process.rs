use crate::processor::VideoProcessor;
use crate::types::TruthBundle;
use std::path::PathBuf;
use tauri::State;
use std::sync::Arc;

#[tauri::command]
pub async fn process_video(
    video_path: String,
    gps_path: Option<String>,
    processor: State<'_, Arc<VideoProcessor>>,
) -> Result<TruthBundle, String> {
    let video_path = PathBuf::from(video_path);
    let gps_path = gps_path.map(PathBuf::from);
    
    processor.process_video(video_path, gps_path)
        .await
        .map_err(|e| e.to_string())
}
