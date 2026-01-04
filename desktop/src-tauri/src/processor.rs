use crate::services::{Ffmpeg, Whisper, parse_gps_file, WhisperModel};
use crate::types::{TruthBundle, TruthEvent, LocationResult};
use anyhow::{Context, Result};
use chrono::Utc;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::{info, debug};
use uuid::Uuid;

pub struct VideoProcessor {
    ffmpeg: Arc<Ffmpeg>,
    whisper: Arc<Whisper>,
    temp_dir: PathBuf,
}

impl VideoProcessor {
    pub fn new(ffmpeg: Arc<Ffmpeg>, whisper: Arc<Whisper>, temp_dir: PathBuf) -> Self {
        Self { ffmpeg, whisper, temp_dir }
    }

    pub async fn process_video(&self, video_path: PathBuf, gps_path: Option<PathBuf>) -> Result<TruthBundle> {
        info!("Processing video: {:?}", video_path);
        
        let video_id = Uuid::new_v4();
        
        // 1. Extract Metadata
        let metadata = self.ffmpeg.extract_metadata(&video_path).await
            .context("Failed to extract video metadata")?;
        debug!("Metadata extracted: {:?}", metadata);

        // 2. Extract Audio
        let audio_filename = format!("{}.wav", video_id);
        let audio_path = self.temp_dir.join(&audio_filename);
        self.ffmpeg.extract_audio(&video_path, &audio_path).await
            .context("Failed to extract audio")?;
        
        // 3. Transcribe Audio
        info!("Transcribing audio...");
        let transcription = self.whisper.transcribe(
            &audio_path, 
            WhisperModel::Base, // Default model
            Some("en")
        ).await.context("Failed to transcribe audio")?;
        
        // Clean up audio file
        if audio_path.exists() {
            let _ = std::fs::remove_file(&audio_path);
        }

        // 4. Parse GPS
        let _gps_track = if let Some(path) = gps_path {
            info!("Parsing GPS track: {:?}", path);
            Some(parse_gps_file(&path).await?)
        } else {
            None
        };

        // 5. Build Truth Bundle
        // This is a simplified merge logic. 
        // Real implementation would sync timestamps of transcription segments with GPS points if possible.
        // For now, we create events from transcription segments.
        
        let mut events = Vec::new();
        
        // Create an event for each transcription segment
        for segment in transcription.segments {
             // Basic location interpolation could happen here if we had GPS timestamps
             let location = LocationResult {
                 lat: 0.0, // Placeholder
                 lon: 0.0,
                 // mismatched fields might need updates in types.rs or here
             };
             
             let event = TruthEvent {
                 id: Uuid::new_v4().to_string(),
                 timestamp: Utc::now(), // Placeholder, should use segment start time + video start time
                 duration_seconds: Some((segment.end_ms - segment.start_ms) as f64 / 1000.0),
                 location,
                 pois: vec![],
                 detected_objects: vec![],
             };
             events.push(event);
        }

        let bundle = TruthBundle {
            project_id: None,
            video_id: Some(video_id),
            events,
            verification_mode: "offline".to_string(),
            generated_at: Utc::now(),
        };

        info!("Video processing complete. Generated Truth Bundle with {} events.", bundle.events.len());
        Ok(bundle)
    }
}
