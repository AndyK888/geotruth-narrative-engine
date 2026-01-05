#![allow(unused)]
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
#[derive(Clone)]
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
    
    /// Extract thumbnails from video at fixed intervals
    pub async fn extract_thumbnails(
        &self,
        video_path: &PathBuf,
        output_dir: &PathBuf,
        interval_seconds: f64,
    ) -> Result<Vec<VideoMoment>, FfmpegError> {
        self.run_extraction(video_path, output_dir, FilterMode::Interval(interval_seconds)).await
    }

    /// Extract key moments using scene detection
    pub async fn extract_key_moments(
        &self,
        video_path: &PathBuf,
        output_dir: &PathBuf,
        threshold: f32, // 0.0 to 1.0 (0.4 is good default)
    ) -> Result<Vec<VideoMoment>, FfmpegError> {
        self.run_extraction(video_path, output_dir, FilterMode::Scene(threshold)).await
    }

    async fn run_extraction(
        &self,
        video_path: &PathBuf,
        output_dir: &PathBuf,
        mode: FilterMode,
    ) -> Result<Vec<VideoMoment>, FfmpegError> {
        if !self.ffmpeg_path.exists() {
            return Err(FfmpegError::BinaryNotFound(self.ffmpeg_path.clone()));
        }
        
        debug!("Extracting frames from: {:?} (Mode: {:?})", video_path, mode);
        
        // Ensure output dir exists
        if !output_dir.exists() {
            std::fs::create_dir_all(output_dir)?;
        }

        let output_pattern = output_dir.join("thumb_%04d.jpg");
        
        let filter = match mode {
            FilterMode::Interval(seconds) => format!("fps=1/{},showinfo", seconds),
            FilterMode::Scene(threshold) => format!("select='gt(scene,{})',showinfo", threshold),
        };

        let args = vec![
            "-i".to_string(),
            video_path.to_string_lossy().to_string(),
            "-vf".to_string(), filter,
            "-vsync".to_string(), "vfr".to_string(),
            "-q:v".to_string(), "2".to_string(),
            "-y".to_string(),
            output_pattern.to_string_lossy().to_string(),
        ];

        let output = Command::new(&self.ffmpeg_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FfmpegError::ExecutionFailed(stderr.to_string()));
        }

        // Parse timestamps from stderr
        let stderr = String::from_utf8_lossy(&output.stderr);
        let mut timestamps: Vec<f64> = Vec::new();

        for line in stderr.lines() {
            if line.contains("Parsed_showinfo") && line.contains("pts_time:") {
                // Example: ... pts_time:12.345 ...
                if let Some(idx) = line.find("pts_time:") {
                    let rest = &line[idx + 9..];
                    let end = rest.find(' ').unwrap_or(rest.len());
                    let val_str = &rest[..end];
                    if let Ok(ts) = val_str.parse::<f64>() {
                        timestamps.push(ts);
                    }
                }
            }
        }
        
        // Collect generated thumbnails
        let mut moments = Vec::new();
        if let Ok(entries) = std::fs::read_dir(output_dir) {
            let mut paths: Vec<PathBuf> = entries
                .flatten()
                .map(|e| e.path())
                .filter(|p| p.file_name().map(|n| n.to_string_lossy().starts_with("thumb_")).unwrap_or(false))
                .collect();
            
            paths.sort(); // thumb_0001, thumb_0002... matches timestamp order
            
            for (i, path) in paths.into_iter().enumerate() {
                let timestamp = if i < timestamps.len() { timestamps[i] } else { 0.0 };
                moments.push(VideoMoment {
                    path,
                    timestamp
                });
            }
        }
        
        info!("Extracted {} frames", moments.len());
        Ok(moments)
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

    /// Capture a single frame at timestamp (ms) and return base64 string
    pub async fn capture_frame(
        &self,
        video_path: &PathBuf,
        timestamp_ms: u64,
    ) -> Result<String, FfmpegError> {
        if !self.ffmpeg_path.exists() {
            return Err(FfmpegError::BinaryNotFound(self.ffmpeg_path.clone()));
        }

        let timestamp_seconds = timestamp_ms as f64 / 1000.0;
        debug!("Capturing frame from: {:?} at {}s", video_path, timestamp_seconds);

        // Usage: ffmpeg -ss <time> -i <input> -frames:v 1 -f image2 pipe:1
        // Placing -ss before -i is faster (input seeking)
        let output = Command::new(&self.ffmpeg_path)
            .args(["-ss", &timestamp_seconds.to_string()])
            .args(["-i"])
            .arg(video_path)
            .args([
                "-frames:v", "1",
                "-f", "image2", // Output format image
                "-c:v", "mjpeg", // JPEG encoding
                "-q:v", "2", // High quality
                "pipe:1", // Output to stdout
            ])
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(FfmpegError::ExecutionFailed(stderr.to_string()));
        }

        use base64::{Engine as _, engine::general_purpose};
        let b64 = general_purpose::STANDARD.encode(&output.stdout);
        let data_uri = format!("data:image/jpeg;base64,{}", b64);

        Ok(data_uri)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoMoment {
    pub path: PathBuf,
    pub timestamp: f64,
}

#[derive(Debug)]
enum FilterMode {
    Interval(f64),
    Scene(f32),
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
