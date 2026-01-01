use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ConfigAnalysis {
    pub config_sources: Vec<String>,
    pub type_safety: String,
    pub secret_handling: String,
}
