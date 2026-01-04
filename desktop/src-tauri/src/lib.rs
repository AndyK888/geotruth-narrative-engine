//! GeoTruth Narrative Engine - Desktop Library
//!
//! This module contains the core Tauri application logic and commands
//! that bridge the React frontend with the Rust backend.

use tauri::Manager;
use tracing::{info, warn};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod commands;
mod config;
mod services;
mod db;
mod state;
mod geo;
mod gemini;
mod types;
mod narrative;
mod enrich;
mod processor;

use db::DbState;
use state::AppState;
use geo::GeoEngine;
// use gemini::GeminiClient; // Removed unused
use narrative::NarrativeEngine;
use enrich::EnrichmentEngine;
use std::sync::Arc;

/// Initialize structured logging with JSON output in production
fn init_logging() {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,geotruth_lib=debug"));

    #[cfg(debug_assertions)]
    {
        // Pretty output for development
        tracing_subscriber::registry()
            .with(filter)
            .with(
                fmt::layer()
                    .with_target(true)
                    .with_thread_ids(false)
                    .with_file(true)
                    .with_line_number(true),
            )
            .init();
    }

    #[cfg(not(debug_assertions))]
    {
        // JSON output for production
        tracing_subscriber::registry()
            .with(filter)
            .with(fmt::layer().json())
            .init();
    }
}

/// Run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_logging();

    info!(
        version = env!("CARGO_PKG_VERSION"),
        "Starting GeoTruth Desktop Application"
    );

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::get_version,
            commands::check_api_connection,
            commands::get_system_info,
            commands::get_map_regions,
            commands::get_available_regions,
            commands::add_region,
            commands::download_map_region,
            commands::delete_map_region,
            commands::get_download_progress,
            commands::ingest::import_video,
            commands::ingest::get_project_videos,
            commands::ingest::create_project,
            commands::ingest::get_projects,
            commands::narrate::narrate,
            commands::enrich::enrich,
            commands::process::process_video,
        ])
        .setup(|app| {
            info!("Application setup complete");

            // Initialize Database
            let db_state = DbState::new(app.handle())
                .expect("Failed to initialize database");
            app.manage(db_state);

            // Initialize Legacy Ingest State
            use commands::ingest::AppState as IngestState;
            use tokio::sync::Mutex;
            app.manage(IngestState {
                db: Mutex::new(None),
                ffmpeg: Mutex::new(None),
            });

            // Initialize Global App State
            let app_state = Arc::new(AppState::new());
            app.manage(app_state.clone());

            // Initialize Geo Engine
            let geo_engine = Arc::new(GeoEngine::new());
            app.manage(geo_engine.clone());
            
            // Initialize Narrative Engine
            let narrative_engine = NarrativeEngine::new();
            app.manage(narrative_engine);
            
            // Initialize Enrichment Engine
            let enrichment_engine = EnrichmentEngine::new(geo_engine, app_state);
            app.manage(enrichment_engine);

            use crate::services::{Ffmpeg, Whisper};
            use crate::processor::VideoProcessor;
            
            // Initialize Services
            // Accessing app handle to get resource path if needed, or assume standard paths
            // For now assuming binaries are in PATH or sidecar
             let binaries_dir = app.path().resource_dir()
                .unwrap_or(std::path::PathBuf::from("."));
            
            let ffmpeg = Arc::new(Ffmpeg::new(binaries_dir.clone()).unwrap_or_else(|e| {
                warn!("FFmpeg init failed: {}", e);
                // Return a dummy or panic? 
                // For now, construct even if binary missing, or handle error better.
                // Re-creating Ffmpeg without checking new() error since it just checks paths.
                 // Actually Ffmpeg::new checks existence and warns.
                 Ffmpeg::new(std::path::PathBuf::from(".")).unwrap() // Fallback
            }));
            let whisper = Arc::new(Whisper::new(binaries_dir.clone()).unwrap_or_else(|e| {
                 warn!("Whisper init failed: {}", e);
                 Whisper::new(std::path::PathBuf::from(".")).unwrap()
            }));
            
            // Initialize Video Processor
            let temp_dir = std::env::temp_dir();
            let video_processor = Arc::new(VideoProcessor::new(ffmpeg, whisper, temp_dir));
            app.manage(video_processor);

            // Log window info
            if let Some(window) = app.get_webview_window("main") {
                info!(
                    window_label = %window.label(),
                    "Main window created"
                );
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
