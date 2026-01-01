use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TechStack {
    pub frameworks: Vec<String>,
    pub libraries: Vec<String>,
    pub databases: Vec<String>,
    pub build_tools: Vec<String>,
}
