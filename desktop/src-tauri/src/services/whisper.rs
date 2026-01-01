//! Whisper.cpp Sidecar Interface
//!
//! Rust interface for executing Whisper.cpp for audio transcription.

use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, warn};

#[derive(Error, Debug)]
pub enum WhisperError {
    #[error("Whisper binary not found at {0}")]
    BinaryNotFound(PathBuf),
    
    #[error("Whisper model not found at {0}")]
    ModelNotFound(PathBuf),
    
    #[error("Whisper execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Failed to parse output: {0}")]
    ParseError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Whisper model sizes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum WhisperModel {
    Tiny,
    TinyEn,
    Base,
    BaseEn,
    Small,
    SmallEn,
    Medium,
    MediumEn,
    Large,
}

impl WhisperModel {
    pub fn filename(&self) -> &'static str {
        match self {
            WhisperModel::Tiny => "ggml-tiny.bin",
            WhisperModel::TinyEn => "ggml-tiny.en.bin",
            WhisperModel::Base => "ggml-base.bin",
            WhisperModel::BaseEn => "ggml-base.en.bin",
            WhisperModel::Small => "ggml-small.bin",
            WhisperModel::SmallEn => "ggml-small.en.bin",
            WhisperModel::Medium => "ggml-medium.bin",
            WhisperModel::MediumEn => "ggml-medium.en.bin",
            WhisperModel::Large => "ggml-large-v3.bin",
        }
    }
    
    pub fn size_mb(&self) -> u32 {
        match self {
            WhisperModel::Tiny | WhisperModel::TinyEn => 75,
            WhisperModel::Base | WhisperModel::BaseEn => 142,
            WhisperModel::Small | WhisperModel::SmallEn => 466,
            WhisperModel::Medium | WhisperModel::MediumEn => 1500,
            WhisperModel::Large => 3100,
        }
    }
}

/// A transcription segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    pub start_ms: i64,
    pub end_ms: i64,
    pub text: String,
}

/// Complete transcription result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transcription {
    pub segments: Vec<TranscriptionSegment>,
    pub language: Option<String>,
    pub full_text: String,
}

/// Whisper.cpp sidecar manager
pub struct Whisper {
    binary_path: PathBuf,
    models_dir: PathBuf,
}

impl Whisper {
    /// Create new Whisper instance
    pub fn new(binaries_dir: PathBuf) -> Result<Self, WhisperError> {
        let binary_path = if cfg!(windows) {
            binaries_dir.join("whisper").join("main.exe")
        } else {
            binaries_dir.join("whisper").join("main")
        };
        
        let models_dir = binaries_dir.join("whisper").join("models");
        
        if !binary_path.exists() {
            warn!("Whisper binary not found: {:?}", binary_path);
        }
        
        Ok(Self {
            binary_path,
            models_dir,
        })
    }
    
    /// Check if a model is available
    pub fn has_model(&self, model: WhisperModel) -> bool {
        self.models_dir.join(model.filename()).exists()
    }
    
    /// Get available models
    pub fn available_models(&self) -> Vec<WhisperModel> {
        use WhisperModel::*;
        
        [Tiny, TinyEn, Base, BaseEn, Small, SmallEn, Medium, MediumEn, Large]
            .into_iter()
            .filter(|m| self.has_model(*m))
            .collect()
    }
    
    /// Transcribe audio file
    pub async fn transcribe(
        &self,
        audio_path: &PathBuf,
        model: WhisperModel,
        language: Option<&str>,
    ) -> Result<Transcription, WhisperError> {
        if !self.binary_path.exists() {
            return Err(WhisperError::BinaryNotFound(self.binary_path.clone()));
        }
        
        let model_path = self.models_dir.join(model.filename());
        if !model_path.exists() {
            return Err(WhisperError::ModelNotFound(model_path));
        }
        
        debug!("Transcribing audio: {:?} with model {:?}", audio_path, model);
        
        let mut args = vec![
            "-m".to_string(),
            model_path.to_string_lossy().to_string(),
            "-f".to_string(),
            audio_path.to_string_lossy().to_string(),
            "-osrt".to_string(),  // Output SRT format
            "-pp".to_string(),    // Print progress
        ];
        
        if let Some(lang) = language {
            args.push("-l".to_string());
            args.push(lang.to_string());
        }
        
        let output = Command::new(&self.binary_path)
            .args(&args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(WhisperError::ExecutionFailed(stderr.to_string()));
        }
        
        let stdout = String::from_utf8_lossy(&output.stdout);
        let segments = self.parse_srt(&stdout)?;
        
        let full_text = segments
            .iter()
            .map(|s| s.text.clone())
            .collect::<Vec<_>>()
            .join(" ");
        
        info!("Transcription complete: {} segments", segments.len());
        
        Ok(Transcription {
            segments,
            language: language.map(|s| s.to_string()),
            full_text,
        })
    }
    
    /// Parse SRT format output
    fn parse_srt(&self, content: &str) -> Result<Vec<TranscriptionSegment>, WhisperError> {
        let mut segments = Vec::new();
        let mut lines = content.lines().peekable();
        
        while lines.peek().is_some() {
            // Skip segment number
            if lines.next().map(|l| l.parse::<u32>().is_ok()).unwrap_or(false) {
                // Parse timestamp line
                if let Some(timestamp_line) = lines.next() {
                    if let Some((start, end)) = self.parse_timestamp_line(timestamp_line) {
                        // Collect text lines until empty line
                        let mut text_lines = Vec::new();
                        while let Some(line) = lines.peek() {
                            if line.trim().is_empty() {
                                lines.next();
                                break;
                            }
                            text_lines.push(lines.next().unwrap().to_string());
                        }
                        
                        segments.push(TranscriptionSegment {
                            start_ms: start,
                            end_ms: end,
                            text: text_lines.join(" ").trim().to_string(),
                        });
                    }
                }
            } else {
                lines.next();
            }
        }
        
        Ok(segments)
    }
    
    /// Parse SRT timestamp line (e.g., "00:00:01,234 --> 00:00:03,456")
    fn parse_timestamp_line(&self, line: &str) -> Option<(i64, i64)> {
        let parts: Vec<&str> = line.split(" --> ").collect();
        if parts.len() == 2 {
            let start = self.parse_timestamp(parts[0])?;
            let end = self.parse_timestamp(parts[1])?;
            Some((start, end))
        } else {
            None
        }
    }
    
    /// Parse single timestamp to milliseconds
    fn parse_timestamp(&self, ts: &str) -> Option<i64> {
        // Format: HH:MM:SS,mmm
        let ts = ts.replace(',', ".");
        let parts: Vec<&str> = ts.split(':').collect();
        if parts.len() == 3 {
            let hours: i64 = parts[0].parse().ok()?;
            let minutes: i64 = parts[1].parse().ok()?;
            let seconds: f64 = parts[2].parse().ok()?;
            Some((hours * 3600 + minutes * 60) * 1000 + (seconds * 1000.0) as i64)
        } else {
            None
        }
    }
}
