use crate::narrative::NarrativeEngine;
use crate::types::{NarrateRequest, NarrateResponse};
use tauri::State;

#[tauri::command]
pub async fn narrate(
    request: NarrateRequest,
    engine: State<'_, NarrativeEngine>,
) -> Result<NarrateResponse, String> {
    engine.generate_narration(request).await.map_err(|e| e.to_string())
}
