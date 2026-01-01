use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use rayon::prelude::*;
use crate::parser::CodeParser;
use crate::rules::AnalysisResult;
use crate::rules::naming::Casing;
use std::sync::{Arc, Mutex};

pub struct ProjectAnalyzer {
    root_path: PathBuf,
}

impl ProjectAnalyzer {
    pub fn new(path: &Path) -> Self {
        Self {
            root_path: path.to_path_buf(),
        }
    }

    pub fn analyze(&self) -> anyhow::Result<AnalysisResult> {
        let files = self.collect_files();
        let results = Arc::new(Mutex::new(AnalysisResult::default()));

        files.par_iter().for_each(|file_path| {
            if let Ok(content) = std::fs::read_to_string(file_path) {
                self.analyze_file_pre_parser(file_path, &content, &results);
                
                if let Some(parser) = CodeParser::new(file_path) {
                    if let Some(tree) = parser.parse(&content) {
                        self.analyze_file(file_path, &content, &tree, &parser, &results);
                    }
                }
            }
        });

        let final_result = results.lock().unwrap().clone();
        Ok(final_result)
    }

    fn analyze_file_pre_parser(
        &self,
        path: &Path,
        _content: &str,
        results: &Arc<Mutex<AnalysisResult>>,
    ) {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        let file_name_lower = file_name.to_lowercase();
        
        // Architecture detection based on directory structure
        let mut layers = Vec::new();
        let mut architecture_pattern = String::new();

        if let Some(parent) = path.parent() {
            let parent_str = parent.to_string_lossy();
            if parent_str.contains("domain") || parent_str.contains("usecase") || parent_str.contains("repository") || parent_str.contains("delivery") {
                architecture_pattern = "Clean Architecture".to_string();
                if parent_str.contains("domain") { layers.push("Domain".to_string()); }
                if parent_str.contains("usecase") { layers.push("UseCase".to_string()); }
                if parent_str.contains("repository") { layers.push("Repository".to_string()); }
                if parent_str.contains("delivery") || parent_str.contains("handler") { layers.push("Delivery/Handler".to_string()); }
            } else if parent_str.contains("controller") || parent_str.contains("model") || parent_str.contains("view") {
                architecture_pattern = "MVC".to_string();
                if parent_str.contains("controller") { layers.push("Controller".to_string()); }
                if parent_str.contains("model") { layers.push("Model".to_string()); }
                if parent_str.contains("view") { layers.push("View".to_string()); }
            } else if parent_str.contains("internal") || parent_str.contains("pkg") || parent_str.contains("cmd") {
                architecture_pattern = "Standard Go Layout".to_string();
            }
        }

        // Heuristic for Config Sources
        if path.extension().is_some_and(|ext| ext == "env" || ext == "yaml" || ext == "yml" || ext == "json") 
           || file_name_lower.contains("config") || file_name_lower == "properties.yaml" {
             let mut global_results = results.lock().unwrap();
             let source = if file_name_lower == "properties.yaml" || file_name_lower == "application.yaml" {
                 file_name.to_string()
             } else {
                 path.extension().and_then(|e| e.to_str()).unwrap_or("config").to_string()
             };
             
             if !global_results.config.config_sources.contains(&source) {
                 global_results.config.config_sources.push(source);
             }
             if file_name_lower == "properties.yaml" {
                 global_results.config.type_safety = "Structured (Properties)".to_string();
             }
        }

        // Merge architecture info
        if !architecture_pattern.is_empty() {
            let mut global_results = results.lock().unwrap();
            if global_results.architecture.pattern.is_empty() {
                global_results.architecture.pattern = architecture_pattern;
            }
            for layer in layers {
                if !global_results.architecture.layers.contains(&layer) {
                    global_results.architecture.layers.push(layer);
                }
            }
        }
    }

    fn collect_files(&self) -> Vec<PathBuf> {
        let mut files = Vec::new();
        let walker = WalkBuilder::new(&self.root_path)
            .hidden(false)
            .git_ignore(true)
            .build();

        for entry in walker.flatten() {
            if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
                files.push(entry.path().to_path_buf());
            }
        }
        files
    }

    fn analyze_file(
        &self,
        path: &Path,
        content: &str,
        tree: &tree_sitter::Tree,
        parser: &CodeParser,
        results: &Arc<Mutex<AnalysisResult>>,
    ) {
        let file_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        
        let analyzer = crate::languages::get_analyzer(parser.language);
        let mut local_result = AnalysisResult::default();
        analyzer.analyze(content, tree, &mut local_result);

        // Update naming conventions for file
        if !file_name.is_empty() {
            if let Some(file_stem) = path.file_stem().and_then(|n| n.to_str()) {
                local_result.naming.file_naming = crate::rules::naming::NamingConvention::detect_casing(file_stem);
            }
            
            // Heuristic for Test Location
            if file_name.contains("test") || file_name.contains("_test") || file_name.contains(".spec.") {
                local_result.testing.test_location = "In-project/In-file".to_string();
                local_result.testing.naming_pattern = "test_* or *_test".to_string();
            }

            if file_name.ends_with("_mock.go") {
                local_result.testing.mocking_strategy = "gomock".to_string();
            }
        }

        // Heuristic for Security (Hardcoded Secrets)
        let secret_regex = regex::Regex::new(r#"(?i)(api_key|secret|password|token)\s*[:=]\s*["'][a-zA-Z0-9]{10,}["']"#).unwrap();
        if secret_regex.is_match(content) {
            local_result.security.hardcoded_secrets.push(format!("Potential secret in {:?}", path));
        }

        let mut global_results = results.lock().unwrap();
        
        // Update language counts
        *global_results.language_counts.entry(parser.language.as_str().to_string()).or_insert(0) += 1;
        
        // Merge results
        if global_results.naming.variable_casing == Casing::Unknown {
            global_results.naming.variable_casing = local_result.naming.variable_casing;
        }
        if global_results.naming.function_casing == Casing::Unknown {
            global_results.naming.function_casing = local_result.naming.function_casing;
        }
        if global_results.naming.class_struct_naming == Casing::Unknown {
            global_results.naming.class_struct_naming = local_result.naming.class_struct_naming;
        }
        if global_results.naming.file_naming == Casing::Unknown {
            global_results.naming.file_naming = local_result.naming.file_naming;
        }
        if global_results.naming.interface_prefix.is_none() {
            global_results.naming.interface_prefix = local_result.naming.interface_prefix;
        }

        if global_results.testing.test_location.is_empty() {
            global_results.testing.test_location = local_result.testing.test_location;
        }
        if global_results.testing.naming_pattern.is_empty() {
            global_results.testing.naming_pattern = local_result.testing.naming_pattern;
        }
        if global_results.testing.mocking_strategy.is_empty() || global_results.testing.mocking_strategy == "N/A" {
            global_results.testing.mocking_strategy = local_result.testing.mocking_strategy;
        }
        if global_results.testing.assertion_style.is_empty() {
            global_results.testing.assertion_style = local_result.testing.assertion_style;
        }

        if global_results.config.type_safety.is_empty() || global_results.config.type_safety == "N/A" {
            global_results.config.type_safety = local_result.config.type_safety;
        }
        
        for secret in local_result.security.hardcoded_secrets {
            if !global_results.security.hardcoded_secrets.contains(&secret) {
                global_results.security.hardcoded_secrets.push(secret);
            }
        }
        
        for pattern in local_result.error_handling.failure_patterns {
            if !global_results.error_handling.failure_patterns.contains(&pattern) {
                global_results.error_handling.failure_patterns.push(pattern);
            }
        }

        for pattern in local_result.di.injection_patterns {
            if !global_results.di.injection_patterns.contains(&pattern) {
                global_results.di.injection_patterns.push(pattern);
            }
        }

        for pattern in local_result.design_patterns.patterns {
            if !global_results.design_patterns.patterns.contains(&pattern) {
                global_results.design_patterns.patterns.push(pattern);
            }
        }

        // Merge Tech Stack
        for f in local_result.tech_stack.frameworks {
            if !global_results.tech_stack.frameworks.contains(&f) {
                global_results.tech_stack.frameworks.push(f);
            }
        }
        for l in local_result.tech_stack.libraries {
            if !global_results.tech_stack.libraries.contains(&l) {
                global_results.tech_stack.libraries.push(l);
            }
        }
        for d in local_result.tech_stack.databases {
            if !global_results.tech_stack.databases.contains(&d) {
                global_results.tech_stack.databases.push(d);
            }
        }
    }
}
