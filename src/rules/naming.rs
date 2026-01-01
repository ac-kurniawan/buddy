use serde::{Serialize, Deserialize};
use regex::Regex;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NamingConvention {
    pub variable_casing: String,
    pub function_casing: String,
    pub class_struct_naming: String,
    pub file_naming: String,
    pub comment_style: String,
}

impl NamingConvention {
    pub fn detect_casing(name: &str) -> &'static str {
        let camel = Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap();
        let pascal = Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
        let snake = Regex::new(r"^[a-z0-9]+(_[a-z0-9]+)*$").unwrap();
        let kebab = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();

        if camel.is_match(name) {
            "camelCase"
        } else if pascal.is_match(name) {
            "PascalCase"
        } else if snake.is_match(name) {
            "snake_case"
        } else if kebab.is_match(name) {
            "kebab-case"
        } else {
            "unknown"
        }
    }
}
