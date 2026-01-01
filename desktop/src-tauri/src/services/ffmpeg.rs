//! FFmpeg Sidecar Interface
//!
//! Rust interface for executing FFmpeg and FFprobe as sidecars.

use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Error, Debug)]
pub enum FfmpegError {
    #[error("FFmpeg binary not found at {0}")]
    BinaryNotFound(PathBuf),
    
    #[error("FFmpeg execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Failed to parse output: {0}")]
    ParseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Video metadata extracted by FFprobe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMetadata {
    pub filename: String,
    pub duration_seconds: Option<f64>,
    pub fps: Option<f64>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub codec: Option<String>,
    pub file_size_bytes: Option<u64>,
    pub has_audio: bool,
    pub audio_codec: Option<String>,
    pub creation_time: Option<String>,
}

/// FFprobe JSON output format
#[derive(Debug, Deserialize)]
struct FfprobeOutput {
    format: Option<FfprobeFormat>,
    streams: Option<Vec<FfprobeStream>>,
}

#[derive(Debug, Deserialize)]
struct FfprobeFormat {
    filename: Option<String>,
    duration: Option<String>,
    size: Option<String>,
    tags: Option<FfprobeTags>,
}

#[derive(Debug, Deserialize)]
struct FfprobeTags {
    creation_time: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FfprobeStream {
    codec_type: Option<String>,
    codec_name: Option<String>,
    width: Option<u32>,
    height: Option<u32>,
    r_frame_rate: Option<String>,
    avg_frame_rate: Option<String>,
}

/// FFmpeg/FFprobe sidecar manager
pub struct Ffmpeg {
    ffmpeg_path: PathBuf,
    ffprobe_path: PathBuf,
}

impl Ffmpeg {
    /// Create new FFmpeg instance with paths to binaries
    pub fn new(binaries_dir: PathBuf) -> Result<Self, FfmpegError> {
        let ffmpeg_path = if cfg!(windows) {
            binaries_dir.join("ffmpeg.exe")
        } else {
            binaries_dir.join("ffmpeg")
        };
        
        let ffprobe_path = if cfg!(windows) {
            binaries_dir.join("ffprobe.exe")
        } else {
            binaries_dir.join("ffprobe")
        };
        
        // Verify binaries exist
        if !ffmpeg_path.exists() {
            warn!("FFmpeg binary not found: {:?}", ffmpeg_path);
        }
        if !ffprobe_path.exists() {
            warn!("FFprobe binary not found: {:?}", ffprobe_path);
        }
        
        Ok(Self {
            ffmpeg_path,
            ffprobe_path,
        })
    }
    
    /// Extract video metadata using FFprobe
    pub async fn extract_metadata(&self, video_path: &PathBuf) -> Result<VideoMetadata, FfmpegError> {
        if !self.ffprobe_path.exists() {
            return Err(FfmpegError::BinaryNotFound(self.ffprobe_path.clone()));
        }
        
        debug!("Extracting metadata from: {:?}", video_path);
        
        let output = Command::new(&self.ffprobe_path)
            .args([
                "-v", "quiet",
                "-print_format", "json",
                "-show_format",
                "-show_streams",
            ])
            .arg(video_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FfmpegError::ExecutionFailed(stderr.to_string()));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let probe: FfprobeOutput = serde_json::from_str(&stdout)
            .map_err(|e| FfmpegError::ParseError(e.to_string()))?;
        
        // Extract video stream info
        let video_stream = probe.streams.as_ref()
            .and_then(|s| s.iter().find(|s| s.codec_type.as_deref() == Some("video")));
        
        let audio_stream = probe.streams.as_ref()
            .and_then(|s| s.iter().find(|s| s.codec_type.as_deref() == Some("audio")));
        
        // Parse FPS from frame rate string (e.g., "30000/1001" or "30/1")
        let fps = video_stream
            .and_then(|s| s.avg_frame_rate.as_ref().or(s.r_frame_rate.as_ref()))
            .and_then(|rate| {
                let parts: Vec<&str> = rate.split('/').collect();
                if parts.len() == 2 {
                    let num: f64 = parts[0].parse().ok()?;
                    let den: f64 = parts[1].parse().ok()?;
                    if den > 0.0 { Some(num / den) } else { None }
                } else {
                    rate.parse().ok()
                }
            });
        
        let metadata = VideoMetadata {
            filename: video_path.file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_default(),
            duration_seconds: probe.format.as_ref()
                .and_then(|f| f.duration.as_ref())
                .and_then(|d| d.parse().ok()),
            fps,
            width: video_stream.and_then(|s| s.width),
            height: video_stream.and_then(|s| s.height),
            codec: video_stream.and_then(|s| s.codec_name.clone()),
            file_size_bytes: probe.format.as_ref()
                .and_then(|f| f.size.as_ref())
                .and_then(|s| s.parse().ok()),
            has_audio: audio_stream.is_some(),
            audio_codec: audio_stream.and_then(|s| s.codec_name.clone()),
            creation_time: probe.format
                .and_then(|f| f.tags)
                .and_then(|t| t.creation_time),
        };
        
        info!("Extracted metadata: {:?}", metadata);
        Ok(metadata)
    }
    
    /// Extract thumbnail frames from video
    pub async fn extract_thumbnails(
        &self,
        video_path: &PathBuf,
        output_dir: &PathBuf,
        interval_seconds: f64,
    ) -> Result<Vec<PathBuf>, FfmpegError> {
        if !self.ffmpeg_path.exists() {
            return Err(FfmpegError::BinaryNotFound(self.ffmpeg_path.clone()));
        }
        
        debug!("Extracting thumbnails from: {:?}", video_path);
        
        let output_pattern = output_dir.join("thumb_%04d.jpg");
        
        let output = Command::new(&self.ffmpeg_path)
            .args([
                "-i",
            ])
            .arg(video_path)
            .args([
                "-vf", &format!("fps=1/{}", interval_seconds),
                "-q:v", "2",
                "-y",
            ])
            .arg(&output_pattern)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FfmpegError::ExecutionFailed(stderr.to_string()));
        }
        
        // Collect generated thumbnails
        let mut thumbnails = Vec::new();
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.file_name()
                    .map(|n| n.to_string_lossy().starts_with("thumb_"))
                    .unwrap_or(false)
                {
                    thumbnails.push(path);
                }
            }
        }
        
        thumbnails.sort();
        info!("Extracted {} thumbnails", thumbnails.len());
        Ok(thumbnails)
    }
    
    /// Extract audio from video as WAV (for Whisper)
    pub async fn extract_audio(
        &self,
        video_path: &PathBuf,
        output_path: &PathBuf,
    ) -> Result<(), FfmpegError> {
        if !self.ffmpeg_path.exists() {
            return Err(FfmpegError::BinaryNotFound(self.ffmpeg_path.clone()));
        }
        
        debug!("Extracting audio from: {:?}", video_path);
        
        let output = Command::new(&self.ffmpeg_path)
            .args(["-i"])
            .arg(video_path)
            .args([
                "-vn",                  // No video
                "-acodec", "pcm_s16le", // PCM 16-bit
                "-ar", "16000",         // 16kHz for Whisper
                "-ac", "1",             // Mono
                "-y",                   // Overwrite
            ])
            .arg(output_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FfmpegError::ExecutionFailed(stderr.to_string()));
        }
        
        info!("Audio extracted to: {:?}", output_path);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_fps() {
        // Test rational fps parsing
        let rate = "30000/1001";
        let parts: Vec<&str> = rate.split('/').collect();
        let num: f64 = parts[0].parse().unwrap();
        let den: f64 = parts[1].parse().unwrap();
        let fps = num / den;
        assert!((fps - 29.97).abs() < 0.01);
    }
}
