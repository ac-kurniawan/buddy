use std::path::Path;
use tree_sitter::{Language, Parser};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SupportedLanguage {
    Go,
    Python,
    TypeScript,
    JavaScript,
    Rust,
}

impl SupportedLanguage {
    pub fn as_str(&self) -> &'static str {
        match self {
            SupportedLanguage::Go => "Go",
            SupportedLanguage::Python => "Python",
            SupportedLanguage::TypeScript => "TypeScript",
            SupportedLanguage::JavaScript => "JavaScript",
            SupportedLanguage::Rust => "Rust",
        }
    }
}

pub struct CodeParser {
    pub language: SupportedLanguage,
    pub ts_language: Language,
}

impl CodeParser {
    pub fn new(path: &Path) -> Option<Self> {
        let extension = path.extension()?.to_str()?;
        let (language, ts_language) = match extension {
            "go" => (SupportedLanguage::Go, tree_sitter_go::LANGUAGE.into()),
            "py" => (SupportedLanguage::Python, tree_sitter_python::LANGUAGE.into()),
            "ts" | "tsx" | "js" | "jsx" => (SupportedLanguage::TypeScript, tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()),
            "rs" => (SupportedLanguage::Rust, tree_sitter_rust::LANGUAGE.into()),
            _ => return None,
        };

        Some(Self {
            language,
            ts_language,
        })
    }

    pub fn parse(&self, source_code: &str) -> Option<tree_sitter::Tree> {
        let mut parser = Parser::new();
        parser.set_language(&self.ts_language).ok()?;
        parser.parse(source_code, None)
    }
}
