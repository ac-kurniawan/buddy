use serde::{Serialize, Deserialize};
use regex::Regex;
use std::fmt;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Default)]
pub enum Casing {
    #[default]
    Unknown,
    #[serde(rename = "camelCase")]
    CamelCase,
    #[serde(rename = "PascalCase")]
    PascalCase,
    #[serde(rename = "snake_case")]
    SnakeCase,
    #[serde(rename = "kebab-case")]
    KebabCase,
    #[serde(rename = "UPPER_SNAKE_CASE")]
    UpperSnakeCase,
}

impl fmt::Display for Casing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Casing::CamelCase => "camelCase",
            Casing::PascalCase => "PascalCase",
            Casing::SnakeCase => "snake_case",
            Casing::KebabCase => "kebab-case",
            Casing::UpperSnakeCase => "UPPER_SNAKE_CASE",
            Casing::Unknown => "N/A",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct NamingConvention {
    pub variable_casing: Casing,
    pub function_casing: Casing,
    pub class_struct_naming: Casing,
    pub file_naming: Casing,
    pub comment_style: String,
    pub interface_prefix: Option<String>,
    pub struct_suffix: Option<String>,
}

impl NamingConvention {
    pub fn detect_casing(name: &str) -> Casing {
        let camel = Regex::new(r"^[a-z][a-zA-Z0-9]*$").unwrap();
        let pascal = Regex::new(r"^[A-Z][a-zA-Z0-9]*$").unwrap();
        let snake = Regex::new(r"^[a-z0-9]+(_[a-z0-9]+)*$").unwrap();
        let kebab = Regex::new(r"^[a-z0-9]+(-[a-z0-9]+)*$").unwrap();
        let upper_snake = Regex::new(r"^[A-Z0-9]+(_[A-Z0-9]+)*$").unwrap();

        if upper_snake.is_match(name) && name.contains('_') {
            Casing::UpperSnakeCase
        } else if camel.is_match(name) {
            Casing::CamelCase
        } else if pascal.is_match(name) {
            Casing::PascalCase
        } else if snake.is_match(name) {
            Casing::SnakeCase
        } else if kebab.is_match(name) {
            Casing::KebabCase
        } else {
            Casing::Unknown
        }
    }
}
