use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct SecurityAnalysis {
    pub hardcoded_secrets: Vec<String>,
    pub input_sanitization: String,
    pub memory_safety: String,
    pub concurrency_safety: String,
}
