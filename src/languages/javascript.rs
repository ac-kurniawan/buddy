use crate::languages::LanguageAnalyzer;
use crate::rules::AnalysisResult;
use crate::rules::naming::NamingConvention;
use tree_sitter::Query;
use streaming_iterator::StreamingIterator;

pub struct JavaScriptAnalyzer;

impl LanguageAnalyzer for JavaScriptAnalyzer {
    fn analyze(&self, content: &str, tree: &tree_sitter::Tree, result: &mut AnalysisResult) {
        // Gunakan TS grammar untuk JS/TS
        let ts_lang = tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into();
        self.analyze_naming(content, tree, &ts_lang, result);
    }
}

impl JavaScriptAnalyzer {
    fn analyze_naming(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (function_declaration (identifier) @func_name)
            (method_definition (property_identifier) @func_name)
            (class_declaration (type_identifier) @class_name)
            (variable_declarator (identifier) @var_name)
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
    fn test_js_naming_analysis() {
        let content = r#"
class UserProfile {
    getUserName() {
        const localVal = "test";
        return localVal;
    }
}
"#;
        
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into()).unwrap();
        let tree = parser.parse(content, None).unwrap();
        
        let analyzer = JavaScriptAnalyzer;
        let mut result = AnalysisResult::default();
        analyzer.analyze(content, &tree, &mut result);
        
        assert_eq!(result.naming.function_casing, "camelCase");
        assert_eq!(result.naming.class_struct_naming, "PascalCase");
        assert_eq!(result.naming.variable_casing, "camelCase");
    }
}
