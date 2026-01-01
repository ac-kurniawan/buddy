pub mod naming;
pub mod dependency_injection;
pub mod testing;
pub mod config;
pub mod security;
pub mod error_handling;

pub mod design_patterns;

use std::collections::HashMap;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct AnalysisResult {
    pub naming: naming::NamingConvention,
    pub di: dependency_injection::DIAnalysis,
    pub testing: testing::TestingAnalysis,
    pub config: config::ConfigAnalysis,
    pub security: security::SecurityAnalysis,
    pub error_handling: error_handling::ErrorHandlingAnalysis,
    pub design_patterns: design_patterns::DesignPatternAnalysis,
    pub language_counts: HashMap<String, usize>,
    pub llm_summary: Option<String>,
}
