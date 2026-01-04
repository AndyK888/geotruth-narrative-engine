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
            model: "gemini-2.0-flash".to_string(),
        }
    }

    pub async fn generate_content(&self, prompt: &str) -> Result<String> {
        if self.api_key.is_empty() {
             bail!("Gemini API Key is missing. Please configure it.");
        }

        let url = format!("{}/{}:generateContent?key={}", GEMINI_API_BASE, self.model, self.api_key);
        
        let request = GenerateContentRequest {
            contents: vec![Content {
                role: "user".to_string(),
                parts: vec![Part {
                    text: prompt.to_string(),
                }],
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
                info!("Gemini response received successfully");
                return Ok(part.text.clone());
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
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GenerateContentResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: Content,
}
