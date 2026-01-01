use crate::rules::AnalysisResult;

pub struct ReportGenerator;

impl ReportGenerator {
    pub fn generate(result: &AnalysisResult) -> String {
        let mut report = String::new();
        let dominant_lang = Self::get_dominant_language(result);

        report.push_str("# Project Guideline\n\n");
        report.push_str(&format!("> **Dominant Language**: {}\n\n", dominant_lang));

        // 1. Naming & Syntax Conventions
        report.push_str("## 1. Naming & Syntax Conventions\n");
        report.push_str(&format!("- **Variable Casing**: {}\n", Self::format_val(&result.naming.variable_casing)));
        report.push_str(&format!("- **Function Casing**: {}\n", Self::format_val(&result.naming.function_casing)));
        report.push_str(&format!("- **Class/Struct Naming**: {}\n", Self::format_val(&result.naming.class_struct_naming)));
        report.push_str(&format!("- **File Naming**: {}\n", Self::format_val(&result.naming.file_naming)));
        report.push_str(&format!("- **Comment Style**: {}\n", Self::format_val(&result.naming.comment_style)));
        let naming_pattern = format!("{} {} {}", result.naming.variable_casing, result.naming.function_casing, result.naming.class_struct_naming);
        Self::append_context(&mut report, &dominant_lang, "naming", &naming_pattern);
        report.push_str("\n");

        // 2. Dependency Injection (DI) & Coupling
        report.push_str("## 2. Dependency Injection (DI) & Coupling\n");
        let di_pattern = Self::format_list(&result.di.injection_patterns);
        report.push_str(&format!("- **Injection Pattern**: {}\n", di_pattern));
        report.push_str(&format!("- **Abstraction Level**: {:.2}\n", result.di.abstraction_level));
        report.push_str(&format!("- **Global State Dependency**: {}\n", Self::format_list(&result.di.global_state_usage)));
        Self::append_context(&mut report, &dominant_lang, "di", &di_pattern);
        report.push_str("\n");

        // 3. Testing Culture & Style
        report.push_str("## 3. Testing Culture & Style\n");
        report.push_str(&format!("- **Test Location**: {}\n", Self::format_val(&result.testing.test_location)));
        report.push_str(&format!("- **Mocking Strategy**: {}\n", Self::format_val(&result.testing.mocking_strategy)));
        report.push_str(&format!("- **Naming Pattern**: {}\n", Self::format_val(&result.testing.naming_pattern)));
        report.push_str(&format!("- **Assertion Style**: {}\n", Self::format_val(&result.testing.assertion_style)));
        Self::append_context(&mut report, &dominant_lang, "testing", &result.testing.test_location);
        report.push_str("\n");

        // 4. Configuration & Environment Management
        report.push_str("## 4. Configuration & Environment Management\n");
        report.push_str(&format!("- **Config Source**: {}\n", Self::format_list(&result.config.config_sources)));
        report.push_str(&format!("- **Type Safety**: {}\n", Self::format_val(&result.config.type_safety)));
        report.push_str(&format!("- **Secret Handling**: {}\n", Self::format_val(&result.config.secret_handling)));
        Self::append_context(&mut report, &dominant_lang, "config", &result.config.type_safety);
        report.push_str("\n");

        // 5. Security & Safety Baseline
        report.push_str("## 5. Security & Safety Baseline\n");
        let secrets_found = if result.security.hardcoded_secrets.is_empty() { "None detected" } else { "Potential secrets found" };
        report.push_str(&format!("- **Hardcoded Secrets**: {}\n", secrets_found));
        report.push_str(&format!("- **Input Sanitization**: {}\n", Self::format_val(&result.security.input_sanitization)));
        report.push_str(&format!("- **Memory Safety**: {}\n", Self::format_val(&result.security.memory_safety)));
        report.push_str(&format!("- **Concurrency Safety**: {}\n", Self::format_val(&result.security.concurrency_safety)));
        Self::append_context(&mut report, &dominant_lang, "security", secrets_found);
        report.push_str("\n");

        // 6. Error Handling Strategy
        report.push_str("## 6. Error Handling Strategy\n");
        let error_pattern = Self::format_list(&result.error_handling.failure_patterns);
        report.push_str(&format!("- **Failure Pattern**: {}\n", error_pattern));
        report.push_str(&format!("- **Logging Consistency**: {}\n", Self::format_val(&result.error_handling.logging_consistency)));
        Self::append_context(&mut report, &dominant_lang, "error_handling", &error_pattern);
        report.push_str("\n");

        // 7. Design Patterns
        report.push_str("## 7. Design Patterns\n");
        let patterns = Self::format_list(&result.design_patterns.patterns);
        report.push_str(&format!("- **Detected Patterns**: {}\n", patterns));
        Self::append_context(&mut report, &dominant_lang, "design_patterns", &patterns);
        report.push_str("\n");

        if let Some(llm_summary) = &result.llm_summary {
            report.push_str("## 8. LLM Analysis Insights\n");
            report.push_str(llm_summary);
            report.push_str("\n");
        }

        report
    }

    fn get_dominant_language(result: &AnalysisResult) -> String {
        result.language_counts.iter()
            .max_by_key(|&(_, count)| count)
            .map(|(lang, _)| lang.clone())
            .unwrap_or_else(|| "Unknown".to_string())
    }

    fn format_val(val: &str) -> String {
        if val.trim().is_empty() || val == "unknown" {
            "N/A".to_string()
        } else {
            val.to_string()
        }
    }

    fn format_list(list: &[String]) -> String {
        if list.is_empty() {
            "N/A".to_string()
        } else {
            list.join(", ")
        }
    }

    fn append_context(report: &mut String, lang: &str, aspect: &str, found_pattern: &str) {
        let (context, recommendation): (Option<String>, Option<String>) = match (lang, aspect) {
            ("Go", "error_handling") => {
                if found_pattern.contains("if err != nil") {
                    (
                        Some("Pola ini adalah standar idiomatis dalam bahasa Go untuk memastikan kegagalan ditangani secara eksplisit.".to_string()),
                        Some("Pastikan untuk membungkus error (*error wrapping*) menggunakan `%w` pada `fmt.Errorf` untuk mempertahankan *stack trace* atau konteks error saat dikembalikan ke pemanggil.".to_string())
                    )
                } else { (None, None) }
            },
            ("Go", "di") => {
                if found_pattern.contains("NewXXX") {
                    (
                        Some("Menggunakan factory function (`NewXXX`) untuk inisialisasi struct adalah pola umum di Go untuk mendukung dependency injection.".to_string()),
                        Some("Pertimbangkan untuk menerima interface daripada struct konkret dalam constructor untuk meningkatkan *testability* dan modularitas.".to_string())
                    )
                } else { (None, None) }
            },
            ("Go", "testing") => {
                if found_pattern.contains("gomock") {
                    (
                        Some("Penggunaan `gomock` menunjukkan budaya testing yang matang dengan penggunaan mock objects yang tergenerasi.".to_string()),
                        Some("Pastikan mock diupdate setiap kali ada perubahan pada interface menggunakan `mockgen`.".to_string())
                    )
                } else { (None, None) }
            },
            ("Go", "config") => {
                if found_pattern.contains("properties.yaml") {
                    (
                        Some("Penggunaan `properties.yaml` menunjukkan adaptasi pola konfigurasi terstruktur mirip Spring Boot.".to_string()),
                        Some("Pastikan konfigurasi di-load ke dalam struct yang ter-validate untuk menjamin type safety saat runtime.".to_string())
                    )
                } else { (None, None) }
            },
            ("Go", "design_patterns") => {
                let mut ctx = Vec::new();
                let mut rec = Vec::new();
                if found_pattern.contains("Factory") {
                    ctx.push("Factory pattern digunakan untuk enkapsulasi inisialisasi objek kompleks.");
                }
                if found_pattern.contains("Singleton") {
                    ctx.push("Singleton digunakan untuk akses global ke resource tunggal (seperti DB connection).");
                    rec.push("Hati-hati dengan Singleton dalam pengujian paralel; pertimbangkan dependency injection sebagai alternatif.");
                }
                if found_pattern.contains("Strategy") {
                    ctx.push("Strategy pattern diimplementasikan melalui interfaces untuk fleksibilitas algoritma.");
                }
                
                (
                    if ctx.is_empty() { None } else { Some(ctx.join(" ")) },
                    if rec.is_empty() { None } else { Some(rec.join(" ")) }
                )
            },
            ("Go", "naming") => {
                if found_pattern.contains("PascalCase") {
                    (
                        Some("PascalCase di Go digunakan untuk mengekspor (export) simbol agar bisa diakses dari package lain.".to_string()),
                        Some("Gunakan `camelCase` untuk internal (unexported) variabel dan fungsi guna menjaga enkapsulasi package.".to_string())
                    )
                } else { (None, None) }
            },
            ("Python", "naming") => {
                if found_pattern.contains("snake_case") {
                    (
                        Some("Python mengikuti PEP 8 yang merekomendasikan `snake_case` untuk variabel dan fungsi.".to_string()),
                        Some("Pastikan untuk konsisten menggunakan `PascalCase` hanya untuk nama Class.".to_string())
                    )
                } else { (None, None) }
            },
            ("TypeScript", "naming") | ("JavaScript", "naming") => {
                if found_pattern.contains("camelCase") {
                    (
                        Some("JavaScript/TypeScript standar menggunakan `camelCase` untuk variabel dan fungsi.".to_string()),
                        Some("Gunakan `PascalCase` untuk Class dan Interfaces, serta `UPPER_SNAKE_CASE` untuk konstanta global.".to_string())
                    )
                } else { (None, None) }
            },
            ("Python", "error_handling") => {
                (
                    Some("Python mengandalkan EAFP (*Easier to Ask for Forgiveness than Permission*) menggunakan blok `try-except`.".to_string()),
                    Some("Gunakan exception yang spesifik daripada menangkap `Exception` umum untuk menghindari penanganan error yang tidak disengaja.".to_string())
                )
            },
            ("TypeScript", "di") | ("JavaScript", "di") => {
                (
                    Some("Di ekosistem JS/TS, Constructor Injection sering digunakan terutama dengan framework seperti NestJS atau Inversify.".to_string()),
                    Some("Manfaatkan TypeScript Interfaces untuk decoupling antara consumer dan provider agar lebih mudah di-mock saat unit testing.".to_string())
                )
            },
            _ => (None, None)
        };

        if let Some(ctx) = context {
            report.push_str(&format!("- **Context**: {}\n", ctx));
        }
        if let Some(rec) = recommendation {
            report.push_str(&format!("- **Best Practice Recommendation**: {}\n", rec));
        }
    }
}
