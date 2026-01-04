use crate::enrich::EnrichmentEngine;
use crate::types::{EnrichRequest, EnrichResponse};
use tauri::State;

#[tauri::command]
pub async fn enrich(
    request: EnrichRequest,
    engine: State<'_, EnrichmentEngine>,
) -> Result<EnrichResponse, String> {
    engine.enrich_point(request).await.map_err(|e| e.to_string())
}
