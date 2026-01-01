use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct DRYAnalysis {
    pub duplicated_blocks: Vec<String>,
    pub duplication_score: f64,
}
