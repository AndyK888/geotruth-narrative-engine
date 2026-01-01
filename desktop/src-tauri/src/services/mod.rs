//! Desktop Application Services
//!
//! This module contains services for file processing, transcription, etc.

pub mod ffmpeg;
pub mod whisper;

pub use ffmpeg::Ffmpeg;
pub use whisper::Whisper;
