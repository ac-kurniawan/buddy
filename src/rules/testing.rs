use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct TestingAnalysis {
    pub test_location: String,
    pub mocking_strategy: String,
    pub naming_pattern: String,
    pub assertion_style: String,
}
