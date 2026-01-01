use crate::languages::LanguageAnalyzer;
use crate::rules::AnalysisResult;
use crate::rules::naming::{NamingConvention, Casing};
use tree_sitter::Query;
use streaming_iterator::StreamingIterator;

pub struct RustAnalyzer;

impl LanguageAnalyzer for RustAnalyzer {
    fn analyze(&self, content: &str, tree: &tree_sitter::Tree, result: &mut AnalysisResult) {
        let ts_lang = tree_sitter_rust::LANGUAGE.into();
        self.analyze_naming(content, tree, &ts_lang, result);
        self.analyze_tech_stack(content, tree, &ts_lang, result);
        self.analyze_error_handling(content, tree, &ts_lang, result);
    }
}

impl RustAnalyzer {
    fn analyze_tech_stack(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (use_declaration argument: (_) @import_source)
        "#;
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let node = capture.node;
                let text = &content[node.start_byte()..node.end_byte()];
                
                let techs = [
                    ("tokio", "library", "Tokio"),
                    ("serde", "library", "Serde"),
                    ("reqwest", "library", "Reqwest"),
                    ("anyhow", "library", "Anyhow"),
                    ("thiserror", "library", "thiserror"),
                    ("clap", "library", "Clap"),
                    ("axum", "framework", "Axum"),
                    ("actix_web", "framework", "Actix-web"),
                    ("sqlx", "database", "sqlx"),
                    ("diesel", "database", "Diesel"),
                ];

                for (key, category, name) in techs {
                    if text.contains(key) {
                        match category {
                            "framework" => if !result.tech_stack.frameworks.contains(&name.to_string()) { result.tech_stack.frameworks.push(name.to_string()) },
                            "database" => if !result.tech_stack.databases.contains(&name.to_string()) { result.tech_stack.databases.push(name.to_string()) },
                            "library" => if !result.tech_stack.libraries.contains(&name.to_string()) { result.tech_stack.libraries.push(name.to_string()) },
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    fn analyze_naming(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (function_item name: (identifier) @func_name)
            (struct_item name: (type_identifier) @class_name)
            (enum_item name: (type_identifier) @class_name)
            (let_declaration pattern: (identifier) @var_name)
            (const_item name: (identifier) @const_name)
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

    fn analyze_error_handling(&self, content: &str, tree: &tree_sitter::Tree, lang: &tree_sitter::Language, result: &mut AnalysisResult) {
        let query_str = r#"
            (enum_variant name: (identifier) @variant_name)
            (call_expression
                function: (identifier) @func_name
                (#match? @func_name "panic")
            ) @panic_call
        "#;
        let query = Query::new(lang, query_str).unwrap();
        let mut cursor = tree_sitter::QueryCursor::new();
        let mut matches = cursor.matches(&query, tree.root_node(), content.as_bytes());

        while let Some(m) = matches.next() {
            for capture in m.captures {
                let capture_name = query.capture_names()[capture.index as usize];
                match capture_name {
                    "panic_call" => {
                        let pattern = "panic!()".to_string();
                        if !result.error_handling.failure_patterns.contains(&pattern) {
                            result.error_handling.failure_patterns.push(pattern);
                        }
                    },
                    _ => {
                        let name = &content[capture.node.start_byte()..capture.node.end_byte()];
                        if name == "Result" || name == "Option" {
                             let pattern = format!("Monadic ({})", name);
                             if !result.error_handling.failure_patterns.contains(&pattern) {
                                 result.error_handling.failure_patterns.push(pattern);
                             }
                        }
                    }
                }
            }
        }
    }
}
