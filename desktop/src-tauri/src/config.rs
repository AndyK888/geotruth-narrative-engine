//! Application Configuration
//!
//! Handles environment-based configuration for the GeoTruth desktop app.

use std::env;

/// Default API URL for local Docker backend
const DEFAULT_API_URL: &str = "http://localhost:8000";

/// Get the API URL from environment or use default
pub fn get_api_url() -> String {
    env::var("GEOTRUTH_API_URL").unwrap_or_else(|_| DEFAULT_API_URL.to_string())
}

/// Check if running in development mode
#[allow(dead_code)]
pub fn is_development() -> bool {
    cfg!(debug_assertions)
}
