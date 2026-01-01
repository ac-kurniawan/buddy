pub mod go;
pub mod python;
pub mod javascript;

use crate::rules::AnalysisResult;
use crate::parser::SupportedLanguage;

pub trait LanguageAnalyzer {
    fn analyze(&self, content: &str, tree: &tree_sitter::Tree, result: &mut AnalysisResult);
}

pub fn get_analyzer(lang: SupportedLanguage) -> Box<dyn LanguageAnalyzer + Send + Sync> {
    match lang {
        SupportedLanguage::Go => Box::new(go::GoAnalyzer),
        SupportedLanguage::Python => Box::new(python::PythonAnalyzer),
        SupportedLanguage::TypeScript | SupportedLanguage::JavaScript => Box::new(javascript::JavaScriptAnalyzer),
    }
}
