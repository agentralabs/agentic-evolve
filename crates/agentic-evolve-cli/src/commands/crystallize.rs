//! Crystallize subcommand.

use anyhow::Result;
use clap::Args;

use agentic_evolve_core::types::pattern::Language;
use agentic_evolve_core::types::skill::SuccessfulExecution;
use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct CrystallizeArgs {
    /// Source file to crystallize
    file: String,

    /// Language of the source file
    #[arg(long, default_value = "rust")]
    language: String,

    /// Domain for the crystallized patterns
    #[arg(long, default_value = "general")]
    domain: String,
}

#[derive(Args)]
pub struct CrystallizeStdinArgs {
    /// Language of the source code
    #[arg(long, default_value = "rust")]
    language: String,

    /// Domain for the crystallized patterns
    #[arg(long, default_value = "general")]
    domain: String,
}

pub fn run(args: CrystallizeArgs, data_dir: &str, json: bool) -> Result<()> {
    let code = std::fs::read_to_string(&args.file)?;
    crystallize_code(&code, &args.language, &args.domain, data_dir, json)
}

pub fn run_stdin(args: CrystallizeStdinArgs, data_dir: &str, json: bool) -> Result<()> {
    let code = std::io::read_to_string(std::io::stdin())?;
    crystallize_code(&code, &args.language, &args.domain, data_dir, json)
}

fn crystallize_code(code: &str, language: &str, domain: &str, data_dir: &str, json: bool) -> Result<()> {
    let mut session = SessionManager::new(data_dir)?;

    let execution = SuccessfulExecution {
        code: code.to_string(),
        language: Language::from_name(language),
        domain: domain.to_string(),
        test_results: Vec::new(),
        execution_time_ms: 0,
    };

    let patterns = session.crystallize(&execution)?;

    if json {
        println!("{}", serde_json::to_string_pretty(&patterns)?);
    } else {
        println!("Crystallized {} pattern(s):", patterns.len());
        for p in &patterns {
            println!("  {} - {} [{}] confidence={:.2}", p.id, p.name, p.domain, p.confidence);
        }
    }

    Ok(())
}
