use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ErrorHandlingAnalysis {
    pub failure_patterns: Vec<String>,
    pub logging_consistency: String,
}
