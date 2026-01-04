#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

// =============================================================================
// Common Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Coordinates {
    pub lat: f64,
    pub lon: f64,
}

// =============================================================================
// POI Models
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POIFacts {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub established: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub depth_m: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unesco_site: Option<bool>,
    #[serde(flatten)]
    pub extra: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct POI {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name_local: Option<String>,
    pub category: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subcategory: Option<String>,
    pub lat: f64,
    pub lon: f64,
    pub distance_m: f64,
    pub bearing_deg: f64,
    pub in_fov: bool,
    pub confidence: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facts: Option<POIFacts>,
}

// =============================================================================
// Location Context
// =============================================================================

// =============================================================================
// Location Context
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationResult {
    pub lat: f64,
    pub lon: f64,
    // Add matched location if needed later
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocationContext {
    pub country: Option<String>,
    pub city: Option<String>,
    pub road: Option<String>,
    pub region: Option<String>,
    pub population: Option<i64>,
    pub timezone: Option<String>,
    pub elevation_m: Option<f64>,
    pub state: Option<String>,
    pub county: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichRequest {
    pub lat: f64,
    pub lon: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnrichResponse {
    pub location: LocationResult,
    pub context: LocationContext,
    pub pois: Vec<POI>,
}

// =============================================================================
// Truth Bundle
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthEvent {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<f64>,
    pub location: LocationResult,
    #[serde(default)]
    pub pois: Vec<POI>,
    #[serde(default)]
    pub detected_objects: Vec<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TruthBundle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub project_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_id: Option<Uuid>,
    #[serde(default)]
    pub events: Vec<TruthEvent>,
    pub verification_mode: String,
    pub generated_at: DateTime<Utc>,
}

// =============================================================================
// AI Narration
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrateRequest {
    pub truth_bundle: TruthBundle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    #[serde(default)]
    pub options: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub time_code: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptSegment {
    pub time_code: String,
    pub narration: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrateScript {
    pub segments: Vec<ScriptSegment>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrateResponse {
    pub chapters: Vec<Chapter>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub script: Option<NarrateScript>,
    #[serde(default)]
    pub meta: HashMap<String, String>,
}
