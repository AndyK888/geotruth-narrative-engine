//! GeoTruth Narrative Engine - Desktop Library
//!
//! This module contains the core Tauri application logic and commands
//! that bridge the React frontend with the Rust backend.

use tauri::Manager;
use tracing::{info, warn, Level};
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

mod commands;
mod config;

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
        .invoke_handler(tauri::generate_handler![
            commands::get_version,
            commands::check_api_connection,
            commands::get_system_info,
        ])
        .setup(|app| {
            info!("Application setup complete");

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
