use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DesignPatternAnalysis {
    pub patterns: Vec<String>,
}
