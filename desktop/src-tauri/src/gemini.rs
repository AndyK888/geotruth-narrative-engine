use crate::config;
use anyhow::{bail, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{debug, error, info};

const GEMINI_API_BASE: &str = "https://generativelanguage.googleapis.com/v1beta/models";

pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
}

impl GeminiClient {
    pub fn new() -> Self {
        let api_key = config::get_gemini_api_key();
        Self {
            client: Client::new(),
            api_key,
            model: "gemini-3.0-flash".to_string(),
        }
    }

    pub async fn generate_content(&self, prompt: &str) -> Result<String> {
        self.generate_multimodal(prompt, vec![]).await
    }

    pub async fn generate_multimodal(&self, prompt: &str, images_base64: Vec<String>) -> Result<String> {
        if self.api_key.is_empty() {
             bail!("Gemini API Key is missing. Please configure it.");
        }

        let url = format!("{}/{}:generateContent?key={}", GEMINI_API_BASE, self.model, self.api_key);
        
        // Build parts
        let mut parts = vec![Part {
            text: Some(prompt.to_string()),
            inline_data: None,
        }];

        // Add images
        for img in images_base64 {
            parts.push(Part {
                text: None,
                inline_data: Some(InlineData {
                    mime_type: "image/jpeg".to_string(), // Assuming JPEG for now
                    data: img,
                }),
            });
        }
        
        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts,
            }],
        };

        debug!("Sending request to Gemini API...");
        let response = self.client.post(&url)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            error!("Gemini API Error: {}", error_text);
            bail!("Gemini API request failed: {}", error_text);
        }

        let result: GenerateContentResponse = response.json().await?;
        
        if let Some(candidate) = result.candidates.first() {
            if let Some(part) = candidate.content.parts.first() {
                // Return text content from response
                if let Some(text) = &part.text {
                     info!("Gemini response received successfully");
                     return Ok(text.clone());
                }
            }
        }

        bail!("No content generated from Gemini API");
    }
}

#[derive(Serialize)]
struct GenerateContentRequest {
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Part {
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    inline_data: Option<InlineData>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InlineData {
    mime_type: String,
    data: String,
}

#[derive(Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}
