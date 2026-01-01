//! Tauri Commands
//!
//! All Tauri command modules for the desktop application.

pub mod ingest;

// Re-export commonly used types
pub use ingest::{AppState, import_video, get_project_videos, create_project, get_projects};
