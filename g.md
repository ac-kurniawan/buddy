# Project Guideline

> **Dominant Language**: Rust

## 0. Tech Stack & Architecture
- **Architecture Pattern**: N/A
- **Frameworks**: N/A
- **Databases**: N/A
- **Libraries**: 
  - Serde
  - Clap
  - Anyhow

## 1. Naming & Syntax Conventions
- **Variable Casing**: camelCase
- **Function Casing**: snake_case
- **Class/Struct Naming**: PascalCase
- **File Naming**: camelCase
- **Comment Style**: N/A

## 2. Dependency Injection (DI) & Coupling
- **Injection Pattern**: N/A
- **Abstraction Level**: 0.00
- **Global State Dependency**: N/A

## 3. Testing Culture & Style
- **Test Location**: In-project/In-file
- **Mocking Strategy**: N/A
- **Naming Pattern**: test_* or *_test
- **Assertion Style**: N/A

## 4. Configuration & Environment Management
- **Config Source**: 
  - config
  - rs
- **Type Safety**: N/A
- **Secret Handling**: N/A

## 5. Security & Safety Baseline
- **Hardcoded Secrets**: None detected
- **Input Sanitization**: N/A
- **Memory Safety**: N/A
- **Concurrency Safety**: N/A

## 6. Error Handling Strategy
- **Failure Pattern**: N/A
- **Logging Consistency**: N/A

## 7. Design Patterns
- **Detected Patterns**: N/A

## 8. LLM Analysis Insights
Berikut adalah analisis arsitektur perangkat lunak dan panduan (`guideline.md`) yang disusun berdasarkan hasil analisis repositori Anda. Panduan ini dirancang untuk membantu AI agent bekerja secara konsisten di dalam proyek ini.

---

# 📘 Software Architecture Guidelines & Analysis

## 1. Executive Summary
Repositori ini adalah proyek berbasis **Rust** (21 file) yang tampaknya berfokus pada aplikasi Command Line Interface (CLI) atau layanan backend ringan. Penggunaan pustaka seperti `Clap` memperkuat indikasi aplikasi CLI, sementara `Serde` menunjukkan adanya pemrosesan data (JSON/YAML/dll). Penggunaan `Anyhow` menandakan strategi penanganan error yang fleksibel pada level aplikasi.

Secara keseluruhan, proyek ini mengikuti konvensi Rust yang cukup standar namun memiliki keunikan pada *naming convention* variabel dan file yang menggunakan `camelCase`, yang berbeda dari standar `snake_case` Rust pada umumnya.

---

## 2. Detailed Analysis Insights

### 2.1 Konvensi Penamaan (Naming)
*   **Variabel & File:** Menggunakan `camelCase`. Ini adalah poin penting karena berbeda dari standar idiomatik Rust (`snake_case`). AI harus mengikuti pola `camelCase` agar konsisten dengan kode yang ada.
*   **Fungsi:** Menggunakan `snake_case`, sesuai dengan standar Rust.
*   **Struct & Enum:** Menggunakan `PascalCase`.
*   **Insight:** Ada perpaduan antara standar Rust (fungsi/struct) dan gaya kustom (variabel/file).

### 2.2 Dependency Injection (DI) & Abstraksi
*   **Status:** Belum ditemukan framework DI formal atau pola abstraksi tingkat tinggi.
*   **Insight:** Proyek kemungkinan besar menggunakan *manual injection* (mewatkan dependency melalui konstruktor struct) atau bersifat fungsional murni.

### 2.3 Pengujian (Testing)
*   **Lokasi:** "In-project/In-file". Ini mengikuti pola idiomatik Rust di mana unit test diletakkan di file yang sama dengan kode sumber menggunakan modul `mod tests`.
*   **Naming:** `test_*` atau `*_test`.

### 2.4 Konfigurasi & Stack Teknologi
*   **Tech Stack:** 
    *   `Serde`: Untuk serialisasi/deserialisasi data.
    *   `Clap`: Untuk parsing argumen CLI.
    *   `Anyhow`: Untuk penanganan error yang mudah di sisi aplikasi.
*   **Config:** Menggunakan file `config.rs`, kemungkinan besar memuat struct konfigurasi yang diisi via variabel lingkungan atau file eksternal.

### 2.5 Penanganan Error (Error Handling)
*   **Insight:** Meskipun data JSON kosong, keberadaan `Anyhow` menunjukkan bahwa aplikasi menggunakan `anyhow::Result<T>` untuk mengalirkan error ke level atas tanpa perlu mendefinisikan tipe error kustom yang kompleks untuk setiap fungsi.

---

## 3. Strategic Recommendations & AI Guidelines

Sebagai pakar arsitektur, berikut adalah pedoman yang harus diikuti oleh AI agent saat berkontribusi pada repositori ini:

### 3.1 Pedoman Coding (Coding Guidelines)
AI harus mengikuti standar penamaan yang terdeteksi meskipun tidak standar bagi Rust:

*   **Gunakan `camelCase`** untuk nama file dan nama variabel di dalam fungsi.
*   **Gunakan `snake_case`** untuk nama fungsi.
*   **Gunakan `PascalCase`** untuk nama Struct dan Enum.

**Contoh Kode:**
```rust
// Nama file: dataProcessor.rs

pub struct UserProfile { // PascalCase untuk struct
    pub userId: i32,     // camelCase untuk variabel (mengikuti pola repo)
}

fn process_user_data(user_id: i32) -> anyhow::Result<()> { // snake_case untuk fungsi
    let currentData = String::from("Sample"); // camelCase untuk variabel lokal
    println!("Processing: {}", currentData);
    Ok(())
}
```

### 3.2 Strategi Error Handling
Gunakan `anyhow` untuk fungsi-fungsi di level aplikasi/bisnis logika agar kode tetap bersih. Gunakan operator `?` untuk propagasi error.

```rust
use anyhow::{Context, Result};

fn load_config() -> Result<Config> {
    let content = std::fs::read_to_string("config.json")
        .with_context(|| "Gagal membaca file konfigurasi")?;
    let config: Config = serde_json::from_str(&content)?;
    Ok(config)
}
```

### 3.3 Struktur Pengujian
Tuliskan test di dalam file yang sama untuk unit testing. Gunakan atribut `#[cfg(test)]`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_logic() {
        // Logika pengujian di sini
        assert_eq!(2 + 2, 4);
    }
}
```

### 3.4 Implementasi CLI (Clap)
Pastikan setiap penambahan fitur CLI baru terintegrasi dengan `Clap` menggunakan gaya derivatif (jika versi yang digunakan mendukung).

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    inputFile: String, // Tetap gunakan camelCase sesuai pola repo
}
```

### 3.5 Pengamanan State (Security & Safety)
*   **Memory Safety:** Karena ini Rust, manfaatkan *ownership model* secara maksimal. Hindari penggunaan `unsafe` block kecuali benar-benar diperlukan.
*   **Input Sanitization:** Karena menggunakan `Clap`, pastikan validasi input dilakukan pada level parsing argumen.

---

## 4. Summary Table for AI Agent

| Aspek | Instruksi |
| :--- | :--- |
| **Bahasa** | Rust |
| **Nama File** | `camelCase.rs` |
| **Variabel** | `camelCase` |
| **Fungsi** | `snake_case` |
| **Error Handling** | Gunakan `anyhow` |
| **Unit Test** | Di dalam file (mod tests) |
| **Library Utama** | Serde, Clap, Anyhow |

Dengan mengikuti panduan ini, AI agent akan menghasilkan kode yang sejalan dengan struktur dan gaya yang sudah ada di dalam proyek, menjaga integritas kode, dan mempercepat proses review.
