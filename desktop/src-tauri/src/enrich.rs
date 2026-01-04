use crate::geo::GeoEngine;
use crate::gemini::GeminiClient;
use crate::state::AppState;
use crate::types::{EnrichRequest, EnrichResponse, LocationResult, LocationContext, POI};
use anyhow::Result;
use tracing::{info, debug, warn};
use std::sync::Arc;




pub struct EnrichmentEngine {
    geo: Arc<GeoEngine>,
    state: Arc<AppState>,
    gemini: GeminiClient,
}

impl EnrichmentEngine {
    pub fn new(geo: Arc<GeoEngine>, state: Arc<AppState>) -> Self {
        Self { 
            geo, 
            state,
            gemini: GeminiClient::new(),
        }
    }

    pub async fn enrich_point(&self, request: EnrichRequest) -> Result<EnrichResponse> {
        let _cache_key = format!("enrich:{:.4}:{:.4}", request.lat, request.lon);
        
        debug!("Enriching point: {}, {}", request.lat, request.lon);

        // 1. Try Local GeoEngine (PMTiles)
        let places = self.geo.reverse_geocode(request.lat, request.lon).await?;
        let local_result = places.first().map(|s| s.as_str()).unwrap_or("Unknown");

        // 2. Hybrid Fallback: If unknown, ask Gemini
        let (country, city, road) = if local_result == "Unknown Location" || local_result == "Unknown" {
            debug!("Local geocoding failed, falling back to Gemini...");
            match self.ask_gemini_location(request.lat, request.lon).await {
                Ok(ctx) => ctx,
                Err(e) => {
                    warn!("Gemini fallback failed: {}", e);
                    ("United States".to_string(), "Unknown City".to_string(), None)
                }
            }
        } else {
             ("United States".to_string(), local_result.to_string(), None)
        };

        // Match Context
        let context = LocationContext {
            country: Some(country), 
            timezone: Some("America/Los_Angeles".to_string()), // Placeholder
            elevation_m: None,
            state: None,
            county: None,
            city: Some(city),
            road,
            region: None,
            population: None,
        };

        // Location Result
        let location = LocationResult {
            lat: request.lat,
            lon: request.lon,
             // matched: None
        };

        // Find POIs (Stub)
        let pois: Vec<POI> = Vec::new();

        let response = EnrichResponse {
            location,
            context,
            pois,
        };

        info!("Enrichment complete for {}, {}", request.lat, request.lon);
        
        Ok(response)
    }

    async fn ask_gemini_location(&self, lat: f64, lon: f64) -> Result<(String, String, Option<String>)> {
        let prompt = format!(
            "Identify the location at latitude {} longitude {}. Return a JSON object with 'country', 'city', and 'road' (optional). Return ONLY JSON.",
            lat, lon
        );
        
        let text = self.gemini.generate_content(&prompt).await?;
        
        // Very basic parsing for demo
        // In real app, use serde_json::from_str with specific struct
        // For now, assuming somewhat structured text or just extracting blindly
        // or just return dummy to prove flow
        if text.contains("json") {
             // strip and parse
        }
        
        Ok(("AI Country".to_string(), "AI City".to_string(), None))
    }
}

// Helper for String ownership

