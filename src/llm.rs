use serde::{Serialize, Deserialize};
use crate::rules::AnalysisResult;
use anyhow::{Context, Result};

#[derive(Serialize)]
struct GeminiRequest {
    contents: Vec<Content>,
}

#[derive(Serialize)]
struct Content {
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Deserialize)]
struct GeminiResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

pub struct GeminiClient {
    api_key: String,
}

impl GeminiClient {
    pub fn new() -> Result<Self> {
        let api_key = std::env::var("GEMINI_API_KEY")
            .context("GEMINI_API_KEY environment variable not set")?;
        Ok(Self { api_key })
    }

    pub fn analyze(&self, result: &AnalysisResult) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-3-flash-preview:generateContent?key={}",
            self.api_key
        );

        let prompt = self.build_prompt(result);

        let request = GeminiRequest {
            contents: vec![Content {
                parts: vec![Part { text: prompt }],
            }],
        };

        let response = client
            .post(&url)
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().unwrap_or_else(|_| "Could not read error body".to_string());
            anyhow::bail!("Gemini API error ({}): {}", status, error_text);
        }

        let gemini_response: GeminiResponse = response.json()?;

        let text = gemini_response
            .candidates
            .first()
            .and_then(|c| c.content.parts.first())
            .map(|p| p.text.clone())
            .context("Failed to get response text from Gemini")?;

        Ok(text)
    }

    fn build_prompt(&self, result: &AnalysisResult) -> String {
        let result_json = serde_json::to_string_pretty(result).unwrap_or_default();
        format!(
            "You are an expert software architect. Analyze the following repository analysis results and provide professional insights, context, and best practice recommendations for each aspect. 
            The goal is to create a high-quality guideline.md for an AI agent.
            
            Analysis Results (JSON):
            ```json
            {}
            ```
            
            Please provide the output in Markdown format with the following sections if applicable:
            - Executive Summary
            - Detailed Analysis Insights (Naming, DI, Testing, Config, Security, Error Handling, Design Patterns)
            - Strategic Recommendations, you can add sample code snippets if applicable.
            
            Use Indonesian language for the analysis and recommendations.",
            result_json
        )
    }
}
