use crate::languages::LanguageAnalyzer;
use crate::rules::AnalysisResult;
use crate::rules::naming::{NamingConvention, Casing};
use tree_sitter::Query;
use streaming_iterator::StreamingIterator;

pub struct PythonAnalyzer;

impl LanguageAnalyzer for PythonAnalyzer {
    fn analyze(&self, content: &str, tree: &tree_sitter::Tree, result: &mut AnalysisResult) {
        let ts_lang = tree_sitter_python::LANGUAGE.into();
        self.analyze_naming(content, tree, &ts_lang, result);
        self.analyze_tech_stack(content, tree, &ts_lang, result);
        self.analyze_dry(content, tree, &ts_lang, result);
    }
}

impl PythonAnalyzer {
    fn analyze_tech_stack(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (import_from_statement (dotted_name) @module_name)
            (import_statement (dotted_name) @module_name)
        "#;
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;
                let name = &content[node.start_byte()..node.end_byte()];
                
                let (category, tech) = match name {
                    "django" => (Some("framework"), "Django"),
                    "flask" => (Some("framework"), "Flask"),
                    "fastapi" => (Some("framework"), "FastAPI"),
                    "sqlalchemy" => (Some("database"), "SQLAlchemy"),
                    "pandas" => (Some("library"), "Pandas"),
                    "numpy" => (Some("library"), "NumPy"),
                    "pytest" => (Some("library"), "pytest"),
                    _ => (None, ""),
                };

                if let Some(cat) = category {
                    match cat {
                        "framework" => if !result.tech_stack.frameworks.contains(&tech.to_string()) { result.tech_stack.frameworks.push(tech.to_string()) },
                        "database" => if !result.tech_stack.databases.contains(&tech.to_string()) { result.tech_stack.databases.push(tech.to_string()) },
                        "library" => if !result.tech_stack.libraries.contains(&tech.to_string()) { result.tech_stack.libraries.push(tech.to_string()) },
                        _ => {}
                    }
                }
            }
        }
    }

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
                        if result.naming.function_casing == Casing::Unknown {
                            result.naming.function_casing = casing;
                        }
                    },
                    "class_name" => {
                        if result.naming.class_struct_naming == Casing::Unknown {
                            result.naming.class_struct_naming = casing;
                        }
                    },
                    "var_name" => {
                        if result.naming.variable_casing == Casing::Unknown {
                            result.naming.variable_casing = casing;
                        }
                    },
                    _ => {}
                }
            }
        }
    }

    fn analyze_dry(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (string) @string
        "#;
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        let mut strings = std::collections::HashMap::new();
        while let Some(m) = matches.next() {
            for capture in m.captures {
                let text = &content[capture.node.start_byte()..capture.node.end_byte()];
                if text.len() > 10 {
                    *strings.entry(text).or_insert(0) += 1;
                }
            }
        }

        for (text, count) in strings {
            if count > 1 {
                result.dry.duplicated_blocks.push(format!("String literal repeated {} times: {}", count, text));
                result.dry.duplication_score += (count - 1) as f64 * 0.1;
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
        
        assert_eq!(result.naming.function_casing, Casing::SnakeCase);
        assert_eq!(result.naming.class_struct_naming, Casing::PascalCase);
        assert_eq!(result.naming.variable_casing, Casing::SnakeCase);
    }
}
