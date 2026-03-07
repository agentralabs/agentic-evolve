//! Coverage subcommands — analyze pattern coverage for source files.

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_core::types::pattern::{FunctionSignature, Language};
use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct CoverageArgs {
    #[command(subcommand)]
    pub command: CoverageCommand,
}

#[derive(Subcommand)]
pub enum CoverageCommand {
    /// Get coverage for a file's functions
    File {
        /// Source file path
        file: String,
        /// Language of the source file
        #[arg(long, default_value = "rust")]
        language: String,
        /// Minimum score threshold for coverage
        #[arg(long, default_value = "0.5")]
        threshold: f64,
    },
    /// Get overall coverage summary
    Summary {
        /// Minimum score threshold for coverage
        #[arg(long, default_value = "0.5")]
        threshold: f64,
    },
}

pub fn run(args: CoverageArgs, data_dir: &str, json: bool) -> Result<()> {
    let session = SessionManager::new(data_dir)?;

    match args.command {
        CoverageCommand::File {
            file,
            language,
            threshold,
        } => {
            let content = std::fs::read_to_string(&file)?;
            let lang = Language::from_name(&language);
            let signatures = extract_function_names(&content, &lang);
            let report = session.coverage(&signatures, threshold);

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("Coverage for {file}:");
                println!(
                    "  {}/{} functions covered ({:.1}%)",
                    report.covered,
                    report.total,
                    report.coverage * 100.0
                );
                for d in &report.details {
                    let status = if d.covered { "+" } else { "-" };
                    println!(
                        "  [{status}] {} (score: {:.3})",
                        d.function_name, d.best_match_score
                    );
                }
            }
        }
        CoverageCommand::Summary { threshold } => {
            // Summary across all stored patterns
            let patterns = session.list_patterns();
            let signatures: Vec<FunctionSignature> =
                patterns.iter().map(|p| p.signature.clone()).collect();
            let report = session.coverage(&signatures, threshold);

            if json {
                println!("{}", serde_json::to_string_pretty(&report)?);
            } else {
                println!("Coverage summary:");
                println!(
                    "  {}/{} functions covered ({:.1}%)",
                    report.covered,
                    report.total,
                    report.coverage * 100.0
                );
            }
        }
    }

    Ok(())
}

/// Simple heuristic to extract function names from source code.
fn extract_function_names(content: &str, language: &Language) -> Vec<FunctionSignature> {
    let mut signatures = Vec::new();
    let fn_pattern = match language {
        Language::Rust => "fn ",
        Language::Python => "def ",
        Language::TypeScript | Language::JavaScript => "function ",
        Language::Go => "func ",
        Language::Java | Language::CSharp => "void ",
        _ => "fn ",
    };

    for line in content.lines() {
        let trimmed = line.trim();
        if let Some(after) = trimmed.strip_prefix("pub ").or(Some(trimmed)) {
            if let Some(rest) = after.strip_prefix(fn_pattern) {
                if let Some(name_end) = rest.find('(') {
                    let name = rest[..name_end].trim();
                    if !name.is_empty() {
                        signatures.push(FunctionSignature::new(name, language.clone()));
                    }
                }
            }
        }
    }

    signatures
}
