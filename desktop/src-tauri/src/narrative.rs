use crate::gemini::GeminiClient;
use crate::types::{NarrateRequest, NarrateResponse, Chapter, ScriptSegment, NarrateScript};
use anyhow::{Context, Result};
use tracing::{info, warn};
use std::collections::HashMap;

pub struct NarrativeEngine {
    gemini: GeminiClient,
}

impl NarrativeEngine {
    pub fn new() -> Self {
        Self {
            gemini: GeminiClient::new(),
        }
    }

    pub async fn generate_narration(&self, request: NarrateRequest) -> Result<NarrateResponse> {
        info!("Generating narration for {} events", request.truth_bundle.events.len());

        let prompt = self.build_narration_prompt(&request);
        
        // Pre-process images (strip data URI prefix if present)
        let images: Vec<String> = request.scene_frames.iter().map(|img| {
            if let Some(idx) = img.find(',') {
                img[idx+1..].to_string()
            } else {
                img.clone()
            }
        }).collect();

        // Call Gemini (Multimodal)
        let response_text = match self.gemini.generate_multimodal(&prompt, images).await {
            Ok(text) => text,
            Err(e) => {
                warn!("Gemini API call failed: {}", e);
                // In a real implementation, we might fallback to offline Llama here
                // For now, return a placeholder or error
                return Err(e.context("Gemini generation failed"));
            }
        };

        // Parse JSON
        // Clean markdown code blocks if present ( ```json ... ``` )
        let clean_json = strip_markdown(&response_text);
        
        let parsed: serde_json::Value = serde_json::from_str(&clean_json)
            .context("Failed to parse Gemini JSON response")?;
        
        // Map to NarrateResponse
        // Using intermediate structure to match JSON output
        #[derive(serde::Deserialize)]
        struct GeminiOutput {
            chapters: Vec<Chapter>,
            script: Vec<ScriptSegment>,
        }
        
        let output: GeminiOutput = serde_json::from_value(parsed)
            .context("Failed to map JSON to output structure")?;

        let mut meta = HashMap::new();
        meta.insert("engine".to_string(), "gemini-3.0-flash".to_string());

        Ok(NarrateResponse {
            chapters: output.chapters,
            script: Some(NarrateScript { segments: output.script }),
            meta,
        })
    }

    fn build_narration_prompt(&self, request: &NarrateRequest) -> String {
        let events = &request.truth_bundle.events;
        
        let event_descriptions: Vec<String> = events.iter().take(20).map(|event| {
            let pois = if event.pois.is_empty() {
                "No landmarks".to_string()
            } else {
                event.pois.iter().take(3).map(|p| p.name.clone()).collect::<Vec<_>>().join(", ")
            };
            
            format!(
                "- At {}: {} (location: {:.4}, {:.4})",
                event.timestamp.format("%H:%M:%S"),
                pois,
                event.location.lat,
                event.location.lon
            )
        }).collect();

        let events_text = if event_descriptions.is_empty() {
            "No events recorded".to_string()
        } else {
            event_descriptions.join("\n")
        };

        let transcript_section = if let Some(transcript) = &request.transcript {
            format!("\n## Existing Audio Transcript\n{}\n", transcript.chars().take(2000).collect::<String>())
        } else {
            String::new()
        };

        format!(
r#"You are a travel documentary narrator creating engaging, fact-checked content.

## Video Context
This is travel footage with verified GPS and location data. Generate narration that:
1. Only mentions facts that can be verified from the provided data
2. Is engaging and suitable for a travel vlog
3. Follows a natural storytelling flow

## Verified Events and Locations
{}
{}
## Output Requirements
Generate a JSON response with this EXACT structure:
{{
  "chapters": [
    {{
      "time_code": "MM:SS",
      "title": "Chapter Title",
      "description": "Brief description"
    }}
  ],
  "script": [
    {{
      "time_code": "MM:SS",
      "narration": "Narration text to speak"
    }}
  ]
}}

Important:
- Each chapter should be 2-5 minutes apart
- Narration should be conversational and engaging
- Only include verifiable facts from the provided data
- Generate 3-5 chapters minimum

Return ONLY valid JSON, no markdown formatting."#,
            events_text,
            transcript_section
        )
    }
}

fn strip_markdown(text: &str) -> String {
    let text = text.trim();
    if text.starts_with("```json") {
        if let Some(end) = text.strip_prefix("```json") {
             if let Some(stripped) = end.strip_suffix("```") {
                 return stripped.trim().to_string();
             }
        }
    }
    if text.starts_with("```") {
         if let Some(end) = text.strip_prefix("```") {
             if let Some(stripped) = end.strip_suffix("```") {
                 return stripped.trim().to_string();
             }
        }
    }
    text.to_string()
}
