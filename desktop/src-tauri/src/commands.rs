//! Tauri Commands
//!
//! Commands exposed to the frontend via Tauri's invoke system.

use tracing::{debug, info, warn};

use crate::config;

/// Get the application version
#[tauri::command]
pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION").to_string();
    debug!(version = %version, "Version requested");
    version
}

/// Check if the API backend is reachable
#[tauri::command]
pub async fn check_api_connection() -> bool {
    let api_url = config::get_api_url();
    let health_url = format!("{}/v1/health", api_url);

    debug!(url = %health_url, "Checking API connection");

    match reqwest::get(&health_url).await {
        Ok(response) => {
            if response.status().is_success() {
                info!(url = %health_url, "API connection successful");
                true
            } else {
                warn!(
                    url = %health_url,
                    status = %response.status(),
                    "API returned non-success status"
                );
                false
            }
        }
        Err(e) => {
            warn!(
                url = %health_url,
                error = %e,
                "Failed to connect to API"
            );
            false
        }
    }
}

/// Get system information
#[tauri::command]
pub fn get_system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    }
}

/// System information structure
#[derive(serde::Serialize)]
pub struct SystemInfo {
    pub os: String,
    pub arch: String,
    pub app_version: String,
}
