//! Tauri Commands
//!
//! All Tauri command modules for the desktop application.

use tracing::{debug, info, warn};

use crate::config;

pub mod ingest;
pub mod narrate;
pub mod enrich;
pub mod process;

// Re-export commonly used types
// pub use ingest::{AppState, import_video, get_project_videos, create_project, get_projects};
// pub use narrate::narrate;
// pub use enrich::enrich;
// pub use process::process_video;

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

// =============================================================================
// Map Region Commands
// =============================================================================

use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

/// Region data structure for frontend
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct RegionInfo {
    pub id: String,
    pub name: String,
    pub size_mb: u64,
    pub downloaded: bool,
    pub last_updated: Option<String>,
    pub poi_count: u32,
    pub bounds: (f64, f64, f64, f64),
}

/// Download progress structure
#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct DownloadProgress {
    pub region_id: String,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub progress_percent: f64,
    pub status: String,
}

/// Global map regions state
static MAP_REGIONS: Lazy<Arc<RwLock<Vec<RegionInfo>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(vec![
        RegionInfo {
            id: "monaco".to_string(),
            name: "Monaco (Test - 1MB)".to_string(),
            size_mb: 1,
            downloaded: false,
            last_updated: None,
            poi_count: 500,
            bounds: (43.72, 7.41, 43.75, 7.44),
        },
        RegionInfo {
            id: "us-southwest".to_string(),
            name: "US Southwest (Grand Canyon, Utah Parks)".to_string(),
            size_mb: 80,
            downloaded: false,
            last_updated: None,
            poi_count: 25000,
            bounds: (31.0, -120.0, 42.0, -102.0),
        },
        RegionInfo {
            id: "us-west".to_string(),
            name: "US West Coast".to_string(),
            size_mb: 150,
            downloaded: false,
            last_updated: None,
            poi_count: 50000,
            bounds: (32.0, -125.0, 49.0, -110.0),
        },
        RegionInfo {
            id: "us-east".to_string(),
            name: "US East Coast".to_string(),
            size_mb: 200,
            downloaded: false,
            last_updated: None,
            poi_count: 75000,
            bounds: (25.0, -85.0, 45.0, -66.0),
        },
    ]))
});

/// Global download progress state
static DOWNLOAD_PROGRESS: Lazy<Arc<RwLock<Option<DownloadProgress>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(None))
});

/// Get all available map regions
#[tauri::command]
pub async fn get_map_regions() -> Vec<RegionInfo> {
    let regions = MAP_REGIONS.read().await;
    
    // Check which regions are already downloaded
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.geotruth.app")
        .join("tiles");
    
    regions.iter().map(|r| {
        let mut region = r.clone();
        let path = data_dir.join(format!("{}.osm.pbf", r.id));
        region.downloaded = path.exists();
        region
    }).collect()
}

/// Download a map region
#[tauri::command]
pub async fn download_map_region(region_id: String) -> Result<(), String> {
    let regions = MAP_REGIONS.read().await;
    let region = regions.iter()
        .find(|r| r.id == region_id)
        .ok_or_else(|| format!("Region not found: {}", region_id))?
        .clone();
    drop(regions);
    
    info!("Starting download for region: {} ({})", region.name, region.id);
    
    // Create data directory
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.geotruth.app")
        .join("tiles");
    std::fs::create_dir_all(&data_dir).map_err(|e| e.to_string())?;
    
    let file_path = data_dir.join(format!("{}.osm.pbf", region_id));
    
    // Get download URL based on region
    let url = match region_id.as_str() {
        "monaco" => "https://download.geofabrik.de/europe/monaco-latest.osm.pbf",
        _ => return Err(format!("Download not yet available for: {}", region_id)),
    };
    
    // Initialize progress
    {
        let mut progress = DOWNLOAD_PROGRESS.write().await;
        *progress = Some(DownloadProgress {
            region_id: region_id.clone(),
            bytes_downloaded: 0,
            total_bytes: region.size_mb * 1024 * 1024,
            progress_percent: 0.0,
            status: "Connecting...".to_string(),
        });
    }
    
    // Download file
    let client = reqwest::Client::new();
    let response = client.get(url)
        .send()
        .await
        .map_err(|e| format!("Download failed: {}", e))?;
    
    let total_size = response.content_length().unwrap_or(region.size_mb * 1024 * 1024);
    
    {
        let mut progress = DOWNLOAD_PROGRESS.write().await;
        if let Some(p) = progress.as_mut() {
            p.total_bytes = total_size;
            p.status = "Downloading...".to_string();
        }
    }
    
    let bytes = response.bytes()
        .await
        .map_err(|e| format!("Failed to read response: {}", e))?;
    
    {
        let mut progress = DOWNLOAD_PROGRESS.write().await;
        if let Some(p) = progress.as_mut() {
            p.bytes_downloaded = bytes.len() as u64;
            p.progress_percent = 100.0;
            p.status = "Saving...".to_string();
        }
    }
    
    // Save file
    std::fs::write(&file_path, &bytes).map_err(|e| format!("Failed to save: {}", e))?;
    
    info!("Download complete: {:?} ({} bytes)", file_path, bytes.len());
    
    // Clear progress
    {
        let mut progress = DOWNLOAD_PROGRESS.write().await;
        *progress = None;
    }
    
    Ok(())
}

/// Delete a downloaded map region
#[tauri::command]
pub async fn delete_map_region(region_id: String) -> Result<(), String> {
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.geotruth.app")
        .join("tiles");
    
    let file_path = data_dir.join(format!("{}.osm.pbf", region_id));
    
    if file_path.exists() {
        std::fs::remove_file(&file_path).map_err(|e| format!("Failed to delete: {}", e))?;
        info!("Deleted map region: {}", region_id);
    }
    
    Ok(())
}

/// Get current download progress
#[tauri::command]
pub async fn get_download_progress() -> Option<DownloadProgress> {
    DOWNLOAD_PROGRESS.read().await.clone()
}
