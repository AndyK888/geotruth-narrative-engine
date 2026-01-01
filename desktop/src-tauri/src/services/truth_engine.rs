//! Local Truth Engine
//!
//! Offline geospatial verification using PMTiles and local data.

use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use tracing::{debug, info, warn};

use super::gps::GpsPoint;

#[derive(Error, Debug)]
pub enum TruthEngineError {
    #[error("Map tiles not found at {0}")]
    TilesNotFound(PathBuf),
    
    #[error("Verification failed: {0}")]
    VerificationFailed(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

/// Verification confidence level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum VerificationConfidence {
    High,      // 90%+ confidence
    Medium,    // 60-90% confidence
    Low,       // 30-60% confidence
    Unverified // < 30% confidence
}

impl VerificationConfidence {
    pub fn as_f64(&self) -> f64 {
        match self {
            VerificationConfidence::High => 0.95,
            VerificationConfidence::Medium => 0.75,
            VerificationConfidence::Low => 0.45,
            VerificationConfidence::Unverified => 0.15,
        }
    }
    
    pub fn from_f64(v: f64) -> Self {
        if v >= 0.9 { VerificationConfidence::High }
        else if v >= 0.6 { VerificationConfidence::Medium }
        else if v >= 0.3 { VerificationConfidence::Low }
        else { VerificationConfidence::Unverified }
    }
}

/// A verified location fact
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedFact {
    pub fact_type: String,
    pub name: String,
    pub value: String,
    pub confidence: VerificationConfidence,
    pub source: String,
}

/// A verified POI from local data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalPOI {
    pub id: String,
    pub name: String,
    pub category: String,
    pub lat: f64,
    pub lon: f64,
    pub distance_m: f64,
    pub bearing_deg: f64,
    pub in_fov: bool,
    pub facts: Vec<VerifiedFact>,
}

/// Truth Bundle for a location
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthBundle {
    pub location: VerifiedLocation,
    pub pois: Vec<LocalPOI>,
    pub facts: Vec<VerifiedFact>,
    pub verification_mode: String,
    pub confidence: VerificationConfidence,
}

/// Verified location context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedLocation {
    pub lat: f64,
    pub lon: f64,
    pub matched_lat: Option<f64>,
    pub matched_lon: Option<f64>,
    pub road_name: Option<String>,
    pub country: Option<String>,
    pub state: Option<String>,
    pub timezone: Option<String>,
}

/// Local Truth Engine for offline verification
pub struct LocalTruthEngine {
    tiles_path: Option<PathBuf>,
    poi_db_path: Option<PathBuf>,
    initialized: bool,
}

impl LocalTruthEngine {
    /// Create new offline truth engine
    pub fn new() -> Self {
        Self {
            tiles_path: None,
            poi_db_path: None,
            initialized: false,
        }
    }
    
    /// Initialize with map tiles
    pub fn with_tiles(mut self, tiles_path: PathBuf) -> Self {
        if tiles_path.exists() {
            self.tiles_path = Some(tiles_path);
            info!("Map tiles configured");
        } else {
            warn!("Map tiles not found: {:?}", tiles_path);
        }
        self
    }
    
    /// Initialize with local POI database
    pub fn with_poi_db(mut self, db_path: PathBuf) -> Self {
        if db_path.exists() {
            self.poi_db_path = Some(db_path);
            info!("POI database configured");
        } else {
            warn!("POI database not found: {:?}", db_path);
        }
        self
    }
    
    /// Check if engine is available for offline use
    pub fn is_available(&self) -> bool {
        self.tiles_path.is_some() || self.poi_db_path.is_some()
    }
    
    /// Verify a GPS point and return Truth Bundle
    pub async fn verify_point(
        &self,
        point: &GpsPoint,
        fov_deg: f64,
    ) -> Result<TruthBundle, TruthEngineError> {
        debug!("Verifying point: ({}, {})", point.lat, point.lon);
        
        // Build verified location
        let location = VerifiedLocation {
            lat: point.lat,
            lon: point.lon,
            matched_lat: None, // Would need PMTiles road network
            matched_lon: None,
            road_name: None,
            country: self.estimate_country(point.lat, point.lon),
            state: None,
            timezone: self.estimate_timezone(point.lat, point.lon),
        };
        
        // Query local POIs (simplified - would use spatial index)
        let pois = self.query_nearby_pois(point.lat, point.lon, 500.0, point.heading_deg, fov_deg)
            .await?;
        
        // Build facts from location
        let mut facts = Vec::new();
        
        if let Some(ref country) = location.country {
            facts.push(VerifiedFact {
                fact_type: "country".to_string(),
                name: "Country".to_string(),
                value: country.clone(),
                confidence: VerificationConfidence::Medium,
                source: "local".to_string(),
            });
        }
        
        if let Some(ref tz) = location.timezone {
            facts.push(VerifiedFact {
                fact_type: "timezone".to_string(),
                name: "Timezone".to_string(),
                value: tz.clone(),
                confidence: VerificationConfidence::High,
                source: "local".to_string(),
            });
        }
        
        // Calculate overall confidence
        let confidence = if pois.is_empty() && facts.is_empty() {
            VerificationConfidence::Low
        } else if pois.len() > 2 {
            VerificationConfidence::High
        } else {
            VerificationConfidence::Medium
        };
        
        Ok(TruthBundle {
            location,
            pois,
            facts,
            verification_mode: "offline".to_string(),
            confidence,
        })
    }
    
    /// Query nearby POIs from local database
    async fn query_nearby_pois(
        &self,
        lat: f64,
        lon: f64,
        radius_m: f64,
        heading_deg: Option<f64>,
        fov_deg: f64,
    ) -> Result<Vec<LocalPOI>, TruthEngineError> {
        // Placeholder - would query local SQLite/DuckDB POI database
        // with spatial index for efficient radius queries
        
        // For now, return empty list (POIs would come from downloaded data)
        Ok(vec![])
    }
    
    /// Estimate country from coordinates (simplified)
    fn estimate_country(&self, lat: f64, lon: f64) -> Option<String> {
        // Very simplified - just check rough bounds
        // Real implementation would use reverse geocoding tiles
        
        if lat >= 24.0 && lat <= 50.0 && lon >= -125.0 && lon <= -66.0 {
            Some("United States".to_string())
        } else if lat >= 41.0 && lat <= 84.0 && lon >= -141.0 && lon <= -52.0 {
            Some("Canada".to_string())
        } else if lat >= 14.0 && lat <= 33.0 && lon >= -118.0 && lon <= -86.0 {
            Some("Mexico".to_string())
        } else {
            None
        }
    }
    
    /// Estimate timezone from coordinates (simplified)
    fn estimate_timezone(&self, lat: f64, lon: f64) -> Option<String> {
        // Simplified timezone estimation based on longitude
        // Real implementation would use timezone boundary tiles
        
        if lon >= -125.0 && lon < -115.0 {
            Some("America/Los_Angeles".to_string())
        } else if lon >= -115.0 && lon < -100.0 {
            Some("America/Denver".to_string())
        } else if lon >= -100.0 && lon < -85.0 {
            Some("America/Chicago".to_string())
        } else if lon >= -85.0 && lon < -66.0 {
            Some("America/New_York".to_string())
        } else {
            None
        }
    }
}

impl Default for LocalTruthEngine {
    fn default() -> Self {
        Self::new()
    }
}
