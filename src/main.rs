
use std::path::Path;
use clap::Parser;
use buddy::ProjectAnalyzer;
use buddy::report::ReportGenerator;
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the repository or directory to analyze
    #[arg(default_value = ".")]
    path: String,

    /// Output file name
    #[arg(short, long, default_value = "CLAUDE.md")]
    output: String,

    /// Use LLM (Google Gemini) for more accurate analysis
    #[arg(long, default_value_t = false)]
    with_llm: bool,
}

fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let args = Args::parse();
    let path = Path::new(&args.path);

    if !path.exists() {
        anyhow::bail!("Path does not exist: {}", args.path);
    }

    println!("Analyzing repository at: {:?}", path);

    let analyzer = ProjectAnalyzer::new(path);
    let mut result = analyzer.analyze()?;

    if args.with_llm {
        println!("Enhancing analysis with LLM (Google Gemini)...");
        match buddy::llm::GeminiClient::new() {
            Ok(client) => {
                match client.analyze(&result) {
                    Ok(summary) => {
                        result.llm_summary = Some(summary);
                        println!("LLM enhancement completed.");
                    },
                    Err(e) => eprintln!("LLM analysis failed: {}", e),
                }
            },
            Err(e) => eprintln!("Failed to initialize LLM client: {}", e),
        }
    }

    let report = ReportGenerator::generate(&result);
    
    fs::write(&args.output, report)?;
    println!("Guideline generated successfully at: {}", args.output);

    Ok(())
}
