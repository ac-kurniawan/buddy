use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DIAnalysis {
    pub injection_patterns: Vec<String>,
    pub abstraction_level: f32, // Ratio of interface vs concrete types
    pub global_state_usage: Vec<String>,
}
