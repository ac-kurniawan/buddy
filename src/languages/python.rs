use crate::languages::LanguageAnalyzer;
use crate::rules::AnalysisResult;
use crate::rules::naming::NamingConvention;
use tree_sitter::Query;
use streaming_iterator::StreamingIterator;

pub struct PythonAnalyzer;

impl LanguageAnalyzer for PythonAnalyzer {
    fn analyze(&self, content: &str, tree: &tree_sitter::Tree, result: &mut AnalysisResult) {
        let ts_lang = tree_sitter_python::LANGUAGE.into();
        self.analyze_naming(content, tree, &ts_lang, result);
    }
}

impl PythonAnalyzer {
    fn analyze_naming(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (function_definition name: (identifier) @func_name)
            (class_definition name: (identifier) @class_name)
            (assignment left: (identifier) @var_name)
        "#;
        
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;
                let name = &content[node.start_byte()..node.end_byte()];
                let casing = NamingConvention::detect_casing(name);
                
                let capture_name = query.capture_names()[capture.index as usize];
                match capture_name {
                    "func_name" => {
                        if result.naming.function_casing.is_empty() {
                            result.naming.function_casing = casing.to_string();
                        }
                    },
                    "class_name" => {
                        if result.naming.class_struct_naming.is_empty() {
                            result.naming.class_struct_naming = casing.to_string();
                        }
                    },
                    "var_name" => {
                        if result.naming.variable_casing.is_empty() {
                            result.naming.variable_casing = casing.to_string();
                        }
                    },
                    _ => {}
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::AnalysisResult;
    use tree_sitter::Parser;

    #[test]
    fn test_python_naming_analysis() {
        let content = r#"
class UserProfile:
    def get_user_name(self):
        local_var = "test"
        return local_var
"#;
        
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_python::LANGUAGE.into()).unwrap();
        let tree = parser.parse(content, None).unwrap();
        
        let analyzer = PythonAnalyzer;
        let mut result = AnalysisResult::default();
        analyzer.analyze(content, &tree, &mut result);
        
        assert_eq!(result.naming.function_casing, "snake_case");
        assert_eq!(result.naming.class_struct_naming, "PascalCase");
        assert_eq!(result.naming.variable_casing, "snake_case");
    }
}
