//! Desktop Application Services
//!
//! This module contains services for file processing, transcription, etc.

pub mod ffmpeg;
pub mod whisper;
pub mod database;
pub mod gps;
pub mod sync;
pub mod truth_engine;
pub mod data_manager;

pub use ffmpeg::Ffmpeg;
pub use whisper::{Whisper, WhisperModel};
pub use database::LocalDatabase;
pub use gps::{parse_gps_file, GpsTrack};
