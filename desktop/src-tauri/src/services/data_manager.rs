#![allow(dead_code)]
//! Data Manager
//!
//! Manages data download, caching, and hybrid online/offline mode.

use std::path::PathBuf;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info};
use tokio::sync::RwLock;

#[derive(Error, Debug)]
pub enum DataError {
    #[error("Region not available offline: {0}")]
    RegionNotAvailable(String),
    
    #[error("Download failed: {0}")]
    DownloadFailed(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Connectivity mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectivityMode {
    Online,
    Offline,
    Hybrid, // Use offline data when available, fallback to online
}

/// Region data availability
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegionInfo {
    pub id: String,
    pub name: String,
    pub size_mb: u64,
    pub downloaded: bool,
    pub last_updated: Option<String>,
    pub poi_count: u32,
    pub bounds: (f64, f64, f64, f64), // min_lat, min_lon, max_lat, max_lon
}

/// Download progress
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadProgress {
    pub region_id: String,
    pub bytes_downloaded: u64,
    pub total_bytes: u64,
    pub progress_percent: f64,
    pub status: String,
}

/// Data Manager for hybrid mode
pub struct DataManager {
    data_dir: PathBuf,
    mode: RwLock<ConnectivityMode>,
    regions: RwLock<HashMap<String, RegionInfo>>,
    download_progress: RwLock<Option<DownloadProgress>>,
}

impl DataManager {
    /// Create new data manager
    pub fn new(data_dir: PathBuf) -> Self {
        Self {
            data_dir,
            mode: RwLock::new(ConnectivityMode::Hybrid),
            regions: RwLock::new(HashMap::new()),
            download_progress: RwLock::new(None),
        }
    }
    
    /// Initialize data manager
    pub async fn init(&self) -> Result<(), DataError> {
        // Create data directories
        let dirs = [
            self.data_dir.join("tiles"),
            self.data_dir.join("pois"),
            self.data_dir.join("cache"),
        ];
        
        for dir in &dirs {
            std::fs::create_dir_all(dir)?;
        }
        
        // Load available regions
        self.load_regions().await?;
        
        info!("Data manager initialized at {:?}", self.data_dir);
        Ok(())
    }
    
    /// Get current connectivity mode
    pub async fn get_mode(&self) -> ConnectivityMode {
        *self.mode.read().await
    }
    
    /// Set connectivity mode
    pub async fn set_mode(&self, mode: ConnectivityMode) {
        *self.mode.write().await = mode;
        info!("Connectivity mode set to {:?}", mode);
    }
    
    /// Check if online services are available
    pub async fn check_connectivity(&self) -> bool {
        // Try to reach API health endpoint
        match reqwest::Client::new()
            .get("http://localhost:8000/v1/health")
            .timeout(std::time::Duration::from_secs(5))
            .send()
            .await
        {
            Ok(response) => response.status().is_success(),
            Err(_) => false,
        }
    }
    
    /// Get available regions
    pub async fn get_regions(&self) -> Vec<RegionInfo> {
        self.regions.read().await.values().cloned().collect()
    }
    
    /// Check if region data is available offline
    pub async fn is_region_available(&self, lat: f64, lon: f64) -> bool {
        let regions = self.regions.read().await;
        
        for region in regions.values() {
            if region.downloaded {
                let (min_lat, min_lon, max_lat, max_lon) = region.bounds;
                if lat >= min_lat && lat <= max_lat && lon >= min_lon && lon <= max_lon {
                    return true;
                }
            }
        }
        
        false
    }
    
    /// Download region data for offline use
    pub async fn download_region(&self, region_id: &str) -> Result<(), DataError> {
        let regions = self.regions.read().await;
        let region = regions.get(region_id)
            .ok_or_else(|| DataError::RegionNotAvailable(region_id.to_string()))?
            .clone();
        drop(regions);
        
        info!("Starting download for region: {}", region.name);
        
        // Initialize progress
        {
            let mut progress = self.download_progress.write().await;
            *progress = Some(DownloadProgress {
                region_id: region_id.to_string(),
                bytes_downloaded: 0,
                total_bytes: region.size_mb * 1024 * 1024,
                progress_percent: 0.0,
                status: "Starting download...".to_string(),
            });
        }
        
        // Download PMTiles
        let tiles_url = format!("http://localhost:8000/v1/tiles/{}.pmtiles", region_id);
        self.download_file(&tiles_url, &self.data_dir.join("tiles").join(format!("{}.pmtiles", region_id))).await?;
        
        // Download POI database
        let pois_url = format!("http://localhost:8000/v1/pois/{}.db", region_id);
        self.download_file(&pois_url, &self.data_dir.join("pois").join(format!("{}.db", region_id))).await?;
        
        // Mark region as downloaded
        {
            let mut regions = self.regions.write().await;
            if let Some(region) = regions.get_mut(region_id) {
                region.downloaded = true;
                region.last_updated = Some(chrono::Utc::now().to_rfc3339());
            }
        }
        
        // Clear progress
        {
            let mut progress = self.download_progress.write().await;
            *progress = None;
        }
        
        info!("Region download complete: {}", region_id);
        Ok(())
    }
    
    /// Get download progress
    pub async fn get_download_progress(&self) -> Option<DownloadProgress> {
        self.download_progress.read().await.clone()
    }
    
    /// Delete region data
    pub async fn delete_region(&self, region_id: &str) -> Result<(), DataError> {
        // Remove files
        let tiles_path = self.data_dir.join("tiles").join(format!("{}.pmtiles", region_id));
        let pois_path = self.data_dir.join("pois").join(format!("{}.db", region_id));
        
        if tiles_path.exists() {
            std::fs::remove_file(&tiles_path)?;
        }
        if pois_path.exists() {
            std::fs::remove_file(&pois_path)?;
        }
        
        // Update region status
        {
            let mut regions = self.regions.write().await;
            if let Some(region) = regions.get_mut(region_id) {
                region.downloaded = false;
                region.last_updated = None;
            }
        }
        
        info!("Region deleted: {}", region_id);
        Ok(())
    }
    
    /// Calculate total offline data size
    pub async fn get_offline_size(&self) -> u64 {
        let regions = self.regions.read().await;
        regions.values()
            .filter(|r| r.downloaded)
            .map(|r| r.size_mb)
            .sum::<u64>() * 1024 * 1024
    }
    
    // Private: Load region definitions
    async fn load_regions(&self) -> Result<(), DataError> {
        // Built-in region definitions
        let built_in_regions = vec![
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
                id: "us-southwest".to_string(),
                name: "US Southwest (Grand Canyon, Utah Parks)".to_string(),
                size_mb: 80,
                downloaded: false,
                last_updated: None,
                poi_count: 25000,
                bounds: (31.0, -120.0, 42.0, -102.0),
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
        ];
        
        let mut regions = self.regions.write().await;
        for region in built_in_regions {
            // Check if already downloaded
            let tiles_path = self.data_dir.join("tiles").join(format!("{}.pmtiles", region.id));
            let mut region = region;
            region.downloaded = tiles_path.exists();
            regions.insert(region.id.clone(), region);
        }
        
        Ok(())
    }
    
    // Private: Download file helper
    async fn download_file(&self, url: &str, path: &PathBuf) -> Result<(), DataError> {
        debug!("Downloading {} to {:?}", url, path);
        
        // For now, just create empty file (actual download would use streaming)
        // This is a placeholder - real implementation would:
        // 1. Send HTTP request with streaming
        // 2. Update progress as chunks arrive
        // 3. Verify checksum
        
        // Simulate download by creating empty file
        std::fs::File::create(path)?;
        
        Ok(())
    }
}
