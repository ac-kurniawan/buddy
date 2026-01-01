# Buddy - Repository Analyzer for AI Agents

Buddy is a tool built in Rust to analyze code repositories and generate a `guideline.md` file. This file serves as a reference for AI agents to understand the conventions, patterns, and best practices used in the codebase.

## Features

- **Multi-language Support**: Currently supports Go, Python, and JavaScript/TypeScript.
- **Fast Analysis**: Built with Rust and uses `rayon` for parallel processing, making it efficient even for large repositories.
- **AST-based Parsing**: Uses `tree-sitter` for accurate code analysis rather than just regex.
- **Git-aware**: Respects `.gitignore` rules automatically.

## Analysis Aspects

1. **Naming & Syntax Conventions**: Detects casing for variables, functions, and classes.
2. **Dependency Injection & Coupling**: Identifies DI patterns and abstraction levels.
3. **Testing Culture**: Analyzes test locations, naming patterns, and styles.
4. **Configuration Management**: Detects how configurations and secrets are handled.
5. **Security & Safety**: Identifies hardcoded secrets and basic safety patterns.
6. **Error Handling**: Analyzes failure patterns and logging consistency.

## Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)

## Installation

Clone the repository and build the project:

```bash
git clone <repository-url>
cd buddy
cargo build --release
```

## Usage

You can run Buddy directly using `cargo run`:

```bash
cargo run -- [PATH] [OPTIONS]
```

### Arguments

- `[PATH]`: Path to the repository or directory to analyze. Defaults to the current directory (`.`).

### Options

- `-o, --output <OUTPUT>`: Output file name. Defaults to `guideline.md`.
- `--with-llm`: Enable LLM-enhanced analysis using Google Gemini. Requires `GEMINI_API_KEY` environment variable.
- `-h, --help`: Print help information.
- `-V, --version`: Print version information.

### Examples

Analyze the current directory:
```bash
cargo run
```

Analyze a specific directory and save to a custom file:
```bash
cargo run -- /path/to/repo --output my-guidelines.md
```

Analyze with LLM enhancement:
```bash
# Set your Gemini API key first
# Windows (PowerShell): $env:GEMINI_API_KEY="your_api_key"
# Linux/macOS: export GEMINI_API_KEY="your_api_key"

cargo run -- . --with-llm
```

## License

The MIT License (MIT)

Copyright (c) 2026 Ardhi C Kurniawan

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.