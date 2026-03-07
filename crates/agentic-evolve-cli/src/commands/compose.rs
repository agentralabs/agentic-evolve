//! Compose subcommands — combine multiple patterns.

use std::collections::HashMap;

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct ComposeArgs {
    #[command(subcommand)]
    pub command: ComposeCommand,
}

#[derive(Subcommand)]
pub enum ComposeCommand {
    /// Compose patterns by their IDs
    Run {
        /// Pattern IDs to compose
        ids: Vec<String>,
        /// Variable bindings as key=value pairs
        #[arg(long, value_delimiter = ',')]
        bindings: Vec<String>,
    },
    /// Preview composition without saving
    Preview {
        /// Pattern IDs to compose
        ids: Vec<String>,
        /// Variable bindings as key=value pairs
        #[arg(long, value_delimiter = ',')]
        bindings: Vec<String>,
    },
}

pub fn run(args: ComposeArgs, data_dir: &str, json: bool) -> Result<()> {
    let session = SessionManager::new(data_dir)?;

    match args.command {
        ComposeCommand::Run { ids, bindings } | ComposeCommand::Preview { ids, bindings } => {
            let binding_map = parse_bindings(&bindings);
            let result = session.compose(&ids, &binding_map)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&result)?);
            } else {
                println!("Composed {} pattern(s):", result.patterns_used.len());
                println!("  Coverage: {:.1}%", result.coverage * 100.0);
                if !result.gaps.is_empty() {
                    println!("  Gaps:");
                    for gap in &result.gaps {
                        println!("    - {gap}");
                    }
                }
                println!("Code:");
                for line in result.code.lines() {
                    println!("  {line}");
                }
            }
        }
    }

    Ok(())
}

fn parse_bindings(bindings: &[String]) -> HashMap<String, String> {
    let mut map = HashMap::new();
    for b in bindings {
        if let Some((key, value)) = b.split_once('=') {
            map.insert(key.to_string(), value.to_string());
        }
    }
    map
}
