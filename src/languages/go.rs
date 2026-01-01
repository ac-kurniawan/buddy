use crate::languages::LanguageAnalyzer;
use crate::rules::AnalysisResult;
use crate::rules::naming::NamingConvention;
use tree_sitter::Query;
use streaming_iterator::StreamingIterator;

pub struct GoAnalyzer;

impl LanguageAnalyzer for GoAnalyzer {
    fn analyze(&self, content: &str, tree: &tree_sitter::Tree, result: &mut AnalysisResult) {
        let ts_lang = tree_sitter_go::LANGUAGE.into();
        
        self.analyze_naming(content, tree, &ts_lang, result);
        self.analyze_error_handling(content, tree, &ts_lang, result);
        self.analyze_di(content, tree, &ts_lang, result);
        self.analyze_design_patterns(content, tree, &ts_lang, result);
        self.analyze_testing(content, tree, &ts_lang, result);
    }
}

impl GoAnalyzer {
    fn analyze_testing(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (import_spec path: (interpreted_string_literal) @import_path)
            (call_expression
                function: (selector_expression
                    operand: (identifier) @pkg_name
                    field: (field_identifier) @method_name
                )
                (#match? @pkg_name "gomock")
            )
        "#;
        
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let text = &content[node.start_byte()..node.end_byte()];
                
                if text.contains("gomock") {
                    if result.testing.mocking_strategy.is_empty() || result.testing.mocking_strategy == "N/A" {
                        result.testing.mocking_strategy = "gomock".to_string();
                    }
                }
            }
        }
    }

    fn analyze_design_patterns(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (function_declaration 
                name: (identifier) @func_name
                (#match? @func_name "^New[A-Z]")
            ) @factory
            
            (function_declaration
                name: (identifier) @singleton_name
                (#match? @singleton_name "Get(Instance|Config|DB)")
            ) @singleton
            
            (type_spec
                name: (type_identifier) @type_name
                type: (interface_type)
            ) @interface
        "#;

        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                let pattern = match capture_name {
                    "factory" => "Factory Pattern (NewXXX)".to_string(),
                    "singleton" => "Potential Singleton (GetInstance)".to_string(),
                    "interface" => "Strategy Pattern (via Interfaces)".to_string(),
                    _ => continue,
                };
                
                if !result.design_patterns.patterns.contains(&pattern) {
                    result.design_patterns.patterns.push(pattern);
                }
            }
        }
    }
    fn analyze_naming(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (function_declaration name: (identifier) @func_name)
            (method_declaration name: (field_identifier) @method_name)
            (type_spec name: (type_identifier) @type_name)
            (var_spec name: (identifier) @var_name)
            (short_var_declaration left: (expression_list (identifier) @var_name))
        "#;
        
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(match_) = matches.next() {
            for capture in match_.captures {
                let node = capture.node;
                let name = &content[node.start_byte()..node.end_byte()];
                let casing = NamingConvention::detect_casing(name);
                
                let capture_name = query.capture_names()[capture.index as usize];
                match capture_name {
                    "func_name" | "method_name" => {
                        if result.naming.function_casing.is_empty() {
                            result.naming.function_casing = casing.to_string();
                        }
                    },
                    "type_name" => {
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

    fn analyze_error_handling(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (if_statement 
                condition: (binary_expression 
                    left: (identifier) @err_name 
                    operator: "!=" 
                    right: (nil)
                )
            ) @error_check
        "#;
        
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(&query, tree.root_node(), content.as_bytes());

        while let Some(_) = captures.next() {
            let pattern = "if err != nil".to_string();
            if !result.error_handling.failure_patterns.contains(&pattern) {
                result.error_handling.failure_patterns.push(pattern);
            }
        }
    }

    fn analyze_di(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (function_declaration 
                name: (identifier) @func_name
                (#match? @func_name "^New[A-Z]")
            )
        "#;
        
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut captures = cursor.captures(&query, tree.root_node(), content.as_bytes());

        while let Some(_) = captures.next() {
            let pattern = "Constructor Injection (NewXXX)".to_string();
            if !result.di.injection_patterns.contains(&pattern) {
                result.di.injection_patterns.push(pattern);
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
    fn test_go_naming_analysis() {
        let content = r#"
            package main
            type UserInfo struct {
                UserName string
            }
            func (u *UserInfo) GetName() string {
                var localVal = "test"
                return u.UserName
            }
        "#;
        
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_go::LANGUAGE.into()).unwrap();
        let tree = parser.parse(content, None).unwrap();
        
        let analyzer = GoAnalyzer;
        let mut result = AnalysisResult::default();
        analyzer.analyze(content, &tree, &mut result);
        
        assert_eq!(result.naming.function_casing, "PascalCase");
        assert_eq!(result.naming.class_struct_naming, "PascalCase");
        assert_eq!(result.naming.variable_casing, "camelCase");
    }

    #[test]
    fn test_go_error_handling_analysis() {
        let content = r#"
            package main
            func main() {
                err := doSomething()
                if err != nil {
                    panic(err)
                }
            }
        "#;
        
        let mut parser = Parser::new();
        parser.set_language(&tree_sitter_go::LANGUAGE.into()).unwrap();
        let tree = parser.parse(content, None).unwrap();
        
        let analyzer = GoAnalyzer;
        let mut result = AnalysisResult::default();
        analyzer.analyze(content, &tree, &mut result);
        
        assert!(result.error_handling.failure_patterns.contains(&"if err != nil".to_string()));
    }
}
