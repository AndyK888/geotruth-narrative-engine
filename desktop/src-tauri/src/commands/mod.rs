//! Tauri Commands
//!
//! All Tauri command modules for the desktop application.

use tracing::{debug, info, warn};

use crate::config;

pub mod ingest;
pub mod narrate;
pub mod enrich;
pub mod process;
pub mod video;



use std::sync::Arc;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

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
/// Global available regions catalog (Hardcoded for now - exhaustive list)
static AVAILABLE_REGIONS: Lazy<Vec<RegionInfo>> = Lazy::new(|| {
    vec![
        // USA
        RegionInfo { id: "us/alabama".to_string(), name: "Alabama (US)".to_string(), size_mb: 250, downloaded: false, last_updated: None, poi_count: 50000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/alaska".to_string(), name: "Alaska (US)".to_string(), size_mb: 150, downloaded: false, last_updated: None, poi_count: 50000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/arizona".to_string(), name: "Arizona (US)".to_string(), size_mb: 200, downloaded: false, last_updated: None, poi_count: 80000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/arkansas".to_string(), name: "Arkansas (US)".to_string(), size_mb: 180, downloaded: false, last_updated: None, poi_count: 60000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/california".to_string(), name: "California (US)".to_string(), size_mb: 1100, downloaded: false, last_updated: None, poi_count: 450000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/colorado".to_string(), name: "Colorado (US)".to_string(), size_mb: 220, downloaded: false, last_updated: None, poi_count: 100000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/connecticut".to_string(), name: "Connecticut (US)".to_string(), size_mb: 80, downloaded: false, last_updated: None, poi_count: 40000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/delaware".to_string(), name: "Delaware (US)".to_string(), size_mb: 40, downloaded: false, last_updated: None, poi_count: 20000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/district-of-columbia".to_string(), name: "District of Columbia (US)".to_string(), size_mb: 30, downloaded: false, last_updated: None, poi_count: 15000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/florida".to_string(), name: "Florida (US)".to_string(), size_mb: 450, downloaded: false, last_updated: None, poi_count: 200000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/georgia".to_string(), name: "Georgia (US)".to_string(), size_mb: 300, downloaded: false, last_updated: None, poi_count: 120000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/hawaii".to_string(), name: "Hawaii (US)".to_string(), size_mb: 50, downloaded: false, last_updated: None, poi_count: 25000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/idaho".to_string(), name: "Idaho (US)".to_string(), size_mb: 150, downloaded: false, last_updated: None, poi_count: 40000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/illinois".to_string(), name: "Illinois (US)".to_string(), size_mb: 350, downloaded: false, last_updated: None, poi_count: 150000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/indiana".to_string(), name: "Indiana (US)".to_string(), size_mb: 200, downloaded: false, last_updated: None, poi_count: 80000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/iowa".to_string(), name: "Iowa (US)".to_string(), size_mb: 180, downloaded: false, last_updated: None, poi_count: 60000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/kansas".to_string(), name: "Kansas (US)".to_string(), size_mb: 160, downloaded: false, last_updated: None, poi_count: 50000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/kentucky".to_string(), name: "Kentucky (US)".to_string(), size_mb: 200, downloaded: false, last_updated: None, poi_count: 70000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/louisiana".to_string(), name: "Louisiana (US)".to_string(), size_mb: 220, downloaded: false, last_updated: None, poi_count: 80000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/maine".to_string(), name: "Maine (US)".to_string(), size_mb: 120, downloaded: false, last_updated: None, poi_count: 40000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/maryland".to_string(), name: "Maryland (US)".to_string(), size_mb: 150, downloaded: false, last_updated: None, poi_count: 60000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/massachusetts".to_string(), name: "Massachusetts (US)".to_string(), size_mb: 200, downloaded: false, last_updated: None, poi_count: 90000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/michigan".to_string(), name: "Michigan (US)".to_string(), size_mb: 350, downloaded: false, last_updated: None, poi_count: 140000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/minnesota".to_string(), name: "Minnesota (US)".to_string(), size_mb: 250, downloaded: false, last_updated: None, poi_count: 90000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/mississippi".to_string(), name: "Mississippi (US)".to_string(), size_mb: 160, downloaded: false, last_updated: None, poi_count: 50000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/missouri".to_string(), name: "Missouri (US)".to_string(), size_mb: 250, downloaded: false, last_updated: None, poi_count: 90000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/montana".to_string(), name: "Montana (US)".to_string(), size_mb: 180, downloaded: false, last_updated: None, poi_count: 40000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/nebraska".to_string(), name: "Nebraska (US)".to_string(), size_mb: 160, downloaded: false, last_updated: None, poi_count: 40000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/nevada".to_string(), name: "Nevada (US)".to_string(), size_mb: 120, downloaded: false, last_updated: None, poi_count: 30000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/new-hampshire".to_string(), name: "New Hampshire (US)".to_string(), size_mb: 80, downloaded: false, last_updated: None, poi_count: 30000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/new-jersey".to_string(), name: "New Jersey (US)".to_string(), size_mb: 180, downloaded: false, last_updated: None, poi_count: 80000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/new-mexico".to_string(), name: "New Mexico (US)".to_string(), size_mb: 150, downloaded: false, last_updated: None, poi_count: 40000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/new-york".to_string(), name: "New York (US)".to_string(), size_mb: 450, downloaded: false, last_updated: None, poi_count: 200000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/north-carolina".to_string(), name: "North Carolina (US)".to_string(), size_mb: 300, downloaded: false, last_updated: None, poi_count: 120000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/north-dakota".to_string(), name: "North Dakota (US)".to_string(), size_mb: 100, downloaded: false, last_updated: None, poi_count: 20000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/ohio".to_string(), name: "Ohio (US)".to_string(), size_mb: 350, downloaded: false, last_updated: None, poi_count: 140000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/oklahoma".to_string(), name: "Oklahoma (US)".to_string(), size_mb: 200, downloaded: false, last_updated: None, poi_count: 70000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/oregon".to_string(), name: "Oregon (US)".to_string(), size_mb: 250, downloaded: false, last_updated: None, poi_count: 90000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/pennsylvania".to_string(), name: "Pennsylvania (US)".to_string(), size_mb: 350, downloaded: false, last_updated: None, poi_count: 140000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/rhode-island".to_string(), name: "Rhode Island (US)".to_string(), size_mb: 40, downloaded: false, last_updated: None, poi_count: 15000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/south-carolina".to_string(), name: "South Carolina (US)".to_string(), size_mb: 200, downloaded: false, last_updated: None, poi_count: 70000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/south-dakota".to_string(), name: "South Dakota (US)".to_string(), size_mb: 120, downloaded: false, last_updated: None, poi_count: 30000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/tennessee".to_string(), name: "Tennessee (US)".to_string(), size_mb: 220, downloaded: false, last_updated: None, poi_count: 80000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/texas".to_string(), name: "Texas (US)".to_string(), size_mb: 850, downloaded: false, last_updated: None, poi_count: 350000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/utah".to_string(), name: "Utah (US)".to_string(), size_mb: 150, downloaded: false, last_updated: None, poi_count: 50000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/vermont".to_string(), name: "Vermont (US)".to_string(), size_mb: 80, downloaded: false, last_updated: None, poi_count: 20000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/virginia".to_string(), name: "Virginia (US)".to_string(), size_mb: 250, downloaded: false, last_updated: None, poi_count: 90000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/washington".to_string(), name: "Washington (US)".to_string(), size_mb: 300, downloaded: false, last_updated: None, poi_count: 120000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/west-virginia".to_string(), name: "West Virginia (US)".to_string(), size_mb: 120, downloaded: false, last_updated: None, poi_count: 40000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/wisconsin".to_string(), name: "Wisconsin (US)".to_string(), size_mb: 250, downloaded: false, last_updated: None, poi_count: 90000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "us/wyoming".to_string(), name: "Wyoming (US)".to_string(), size_mb: 120, downloaded: false, last_updated: None, poi_count: 30000, bounds: (0.0, 0.0, 0.0, 0.0) },
        // Europe Examples
        RegionInfo { id: "europe/monaco".to_string(), name: "Monaco".to_string(), size_mb: 1, downloaded: false, last_updated: None, poi_count: 500, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "europe/france".to_string(), name: "France".to_string(), size_mb: 3500, downloaded: false, last_updated: None, poi_count: 1500000, bounds: (0.0, 0.0, 0.0, 0.0) },
        RegionInfo { id: "europe/germany".to_string(), name: "Germany".to_string(), size_mb: 3200, downloaded: false, last_updated: None, poi_count: 1400000, bounds: (0.0, 0.0, 0.0, 0.0) },
    ]
});

/// Global map regions state (User added regions)
static MAP_REGIONS: Lazy<Arc<RwLock<Vec<RegionInfo>>>> = Lazy::new(|| {
    let regions = load_regions_from_disk().unwrap_or_else(|| {
        vec![
            // Defaults if no file exists
            AVAILABLE_REGIONS.iter().find(|r| r.id == "europe/monaco").unwrap().clone(),
            AVAILABLE_REGIONS.iter().find(|r| r.id == "us/california").unwrap().clone(),
        ]
    });
    Arc::new(RwLock::new(regions))
});

/// Helper to get persistence file path
fn get_regions_file_path() -> std::path::PathBuf {
    dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.geotruth.app")
        .join("regions.json")
}

/// Helper to save regions to disk
fn save_regions_to_disk(regions: &Vec<RegionInfo>) {
    let path = get_regions_file_path();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).ok();
    }
    
    if let Ok(json) = serde_json::to_string_pretty(regions) {
        if let Err(e) = std::fs::write(&path, json) {
            warn!("Failed to save regions: {}", e);
        } else {
            info!("Saved regions to {:?}", path);
        }
    }
}

/// Helper to load regions from disk
fn load_regions_from_disk() -> Option<Vec<RegionInfo>> {
    let path = get_regions_file_path();
    if !path.exists() {
        return None;
    }
    
    match std::fs::read_to_string(&path) {
        Ok(json) => {
            match serde_json::from_str(&json) {
                Ok(regions) => Some(regions),
                Err(e) => {
                    warn!("Failed to parse regions file: {}", e);
                    None
                }
            }
        },
        Err(e) => {
            warn!("Failed to read regions file: {}", e);
            None
        }
    }
}

/// Global download progress state
static DOWNLOAD_PROGRESS: Lazy<Arc<RwLock<Option<DownloadProgress>>>> = Lazy::new(|| {
    Arc::new(RwLock::new(None))
});

/// Get all available map regions from catalog
#[tauri::command]
pub async fn get_available_regions() -> Vec<RegionInfo> {
    AVAILABLE_REGIONS.clone()
}

/// Add a region to my map packs
#[tauri::command]
pub async fn add_region(region_id: String) -> Result<(), String> {
    let mut regions = MAP_REGIONS.write().await;
    
    // Check if already added
    if regions.iter().any(|r| r.id == region_id) {
        return Ok(());
    }

    // Find in catalog
    if let Some(region) = AVAILABLE_REGIONS.iter().find(|r| r.id == region_id) {
        regions.push(region.clone());
        // Save using current list
        save_regions_to_disk(&regions);
        Ok(())
    } else {
        Err(format!("Region not found in catalog: {}", region_id))
    }
}

/// Get my map regions
#[tauri::command]
pub async fn get_map_regions() -> Vec<RegionInfo> {
    let regions = MAP_REGIONS.read().await;
    
    let data_dir = dirs::data_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("com.geotruth.app")
        .join("tiles");
    
    regions.iter().map(|r| {
        let mut region = r.clone();
        // sanitize id for filename (replace / with _)
        let filename = r.id.replace("/", "_");
        let path = data_dir.join(format!("{}.osm.pbf", filename));
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
    
    let file_path = data_dir.join(format!("{}.osm.pbf", region_id.replace("/", "_")));
    
    // Get download URL based on region
    // Dynamic Geofabrik URL construction
    let url = if region_id.starts_with("us/") {
        let state = region_id.strip_prefix("us/").unwrap();
        format!("https://download.geofabrik.de/north-america/us/{}-latest.osm.pbf", state)
    } else if region_id.starts_with("europe/") {
        let country = region_id.strip_prefix("europe/").unwrap();
        format!("https://download.geofabrik.de/europe/{}-latest.osm.pbf", country)
    } else {
        match region_id.as_str() {
            "monaco" => "https://download.geofabrik.de/europe/monaco-latest.osm.pbf".to_string(),
            "california" => "https://download.geofabrik.de/north-america/us/california-latest.osm.pbf".to_string(), // Legacy fallback
            _ => return Err(format!("Download logic not implemented for: {}", region_id)),
        }
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
    
    // Download file with streaming for progress
    use futures_util::StreamExt;
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
    
    let mut file = std::fs::File::create(&file_path).map_err(|e| format!("Failed to create file: {}", e))?;
    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();
    
    while let Some(item) = stream.next().await {
        let chunk = item.map_err(|e| format!("Error while downloading: {}", e))?;
        std::io::Write::write_all(&mut file, &chunk).map_err(|e| format!("Error while writing to file: {}", e))?;
        downloaded += chunk.len() as u64;
        
        {
            let mut progress = DOWNLOAD_PROGRESS.write().await;
            if let Some(p) = progress.as_mut() {
                p.bytes_downloaded = downloaded;
                p.progress_percent = (downloaded as f64 / total_size as f64) * 100.0;
            }
        }
    }
    
    {
        let mut progress = DOWNLOAD_PROGRESS.write().await;
        if let Some(p) = progress.as_mut() {
            p.bytes_downloaded = downloaded;
            p.progress_percent = 100.0;
            p.status = "Saving...".to_string();
        }
    }
    
    info!("Download complete: {:?} ({} bytes)", file_path, downloaded);
    
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
    
    let file_path = data_dir.join(format!("{}.osm.pbf", region_id.replace("/", "_")));
    
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
