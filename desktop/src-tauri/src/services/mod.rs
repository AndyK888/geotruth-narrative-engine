//! Desktop Application Services
//!
//! This module contains services for file processing, transcription, etc.

pub mod ffmpeg;
pub mod whisper;
pub mod database;
pub mod gps;
pub mod sync;

pub use ffmpeg::Ffmpeg;
pub use whisper::Whisper;
pub use database::LocalDatabase;
pub use gps::{parse_gps_file, GpsTrack, GpsPoint};
pub use sync::TimeSyncEngine;
