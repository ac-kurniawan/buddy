use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ArchitectureAnalysis {
    pub pattern: String, // e.g., "Clean Architecture", "MVC", "Hexagonal"
    pub layers: Vec<String>,
    pub modules: Vec<String>,
}
