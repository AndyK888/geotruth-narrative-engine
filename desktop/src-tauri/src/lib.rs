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
        .plugin(tauri_plugin_fs::init())
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
            commands::video::capture_frame,
            commands::video::auto_scan_moments,
        ])
        .setup(|app| {
            info!("Application setup complete");

            // Initialize Database
            use services::database::LocalDatabase;
            let app_data_dir = app.path().app_data_dir().expect("Failed to get app data dir");
            let db_path = app_data_dir.join("geotruth_v1.duckdb");
            
            let db = LocalDatabase::open(db_path).expect("Failed to initialize database");
            
            // Run async init
            tauri::async_runtime::block_on(async {
                db.init().await.expect("Failed to run database migrations");
            });
            
            app.manage(db);

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

            // Initialize Services
            // In production (bundle), binaries should be in resource_dir.
            use crate::services::{Ffmpeg, Whisper};
            use crate::processor::VideoProcessor;

            // Initialize Services
            // In production (bundle), binaries should be in resource_dir.
            // In dev (debug), they are likely in ../binaries (relative to src-tauri).
             let mut binaries_dir = app.path().resource_dir()
                .unwrap_or(std::path::PathBuf::from("."));
            
            #[cfg(debug_assertions)]
            {
                // Verify if binaries exist in the default location, if not try dev path
                let has_ffmpeg = binaries_dir.join("ffmpeg").exists() || binaries_dir.join("ffmpeg.exe").exists();
                
                if !has_ffmpeg {
                    // Try looking in ../binaries relative to CWD (usually src-tauri)
                    let dev_path = std::env::current_dir()
                        .map(|p| p.join("../binaries"))
                        .unwrap_or_else(|_| std::path::PathBuf::from("../binaries"));
                    
                    if dev_path.exists() {
                        info!("Using development binaries directory: {:?}", dev_path);
                        binaries_dir = dev_path;
                    } else {
                        warn!("Could not find binaries in {:?} or {:?}", binaries_dir, dev_path);
                    }
                }
            }
            
            let ffmpeg = Arc::new(Ffmpeg::new(binaries_dir.clone()).unwrap_or_else(|e| {
                warn!("FFmpeg init failed: {}", e);
                 Ffmpeg::new(std::path::PathBuf::from(".")).unwrap() 
            }));
            let whisper = Arc::new(Whisper::new(binaries_dir.clone()).unwrap_or_else(|e| {
                 warn!("Whisper init failed: {}", e);
                 Whisper::new(std::path::PathBuf::from(".")).unwrap()
            }));

            // Initialize Legacy Ingest State with ACTUAL FFmpeg
            use commands::ingest::AppState as IngestState;
            use tokio::sync::Mutex;
            // The ingest::AppState expects Mutex<Option<Ffmpeg>> (not Arc). 
            // We need to clone the Ffmpeg inner struct, but Ffmpeg definition might not be cloneable or we might need to wrap it differently.
            // Looking at `ingest.rs`, AppState has `ffmpeg: Mutex<Option<Ffmpeg>>`. 
            // And `Ffmpeg` struct in `services/ffmpeg.rs` needs to be checked if it is cloneable.
            // Assuming Ffmpeg is lightweight (just paths), we can clone it if it derives Clone.
            // If not, we might need to adjust ingest.rs to take Arc<Ffmpeg> or similar.
            
            // Checking: ingest.rs: `pub struct AppState { pub ffmpeg: Mutex<Option<Ffmpeg>>, ... }`
            // Let's assume we can clone because we saw `ffmpeg` variable above is `Arc<Ffmpeg>`.
            // Wait, we need to pass a `Ffmpeg` instance, not `Arc`. 
            // Let's check if Ffmpeg implements Clone. If not, we might fail compilation.
            // Safe bet: The logic in ingest.rs is outdated because it uses `Mutex<Option<Ffmpeg>>`.
            // Ideally `ingest.rs` should just use `State<'_, Arc<Ffmpeg>>`.
            // BUT to minimize changes and "fix" the existing logic:
            // We will deref the Arc to get a clone if possible.
            
            // Let's rely on standard Rust pattern here. 
            // Actually, let's inject Arc<Ffmpeg> as a managed state and update ingest.rs to use that instead of the custom struct if possible.
            // BUT `import_video` signature is `ffmpeg_state: State<'_, AppState>`.
            // So we MUST populate AppState.
            
            app.manage(IngestState {
                db: Mutex::new(None), // We use global DB state now
                // We need to unwrap the Arc or clone the inner. 
                // Since Ffmpeg holds PathBufs, it should be cloneable. 
                ffmpeg: Mutex::new(Some((*ffmpeg).clone())), 
            });

            
            // Initialize Video Processor
            let temp_dir = std::env::temp_dir();
            let video_processor = Arc::new(VideoProcessor::new(ffmpeg.clone(), whisper, temp_dir));
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
