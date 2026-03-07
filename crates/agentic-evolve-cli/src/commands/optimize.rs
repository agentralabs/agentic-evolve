//! Optimize subcommands.

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct OptimizeArgs {
    #[command(subcommand)]
    pub command: Option<OptimizeCommand>,
}

#[derive(Subcommand)]
pub enum OptimizeCommand {
    /// Find duplicate patterns
    Duplicates,
    /// Prune low-confidence patterns
    Prune {
        /// Minimum confidence threshold
        #[arg(long, default_value = "0.2")]
        threshold: f64,
    },
}

pub fn run(args: OptimizeArgs, data_dir: &str, json: bool) -> Result<()> {
    let mut session = SessionManager::new(data_dir)?;

    match args.command {
        None => {
            let summary = session.optimize()?;

            if json {
                println!("{}", serde_json::to_string_pretty(&summary)?);
            } else {
                println!("Optimization complete:");
                println!("  Total patterns:    {}", summary.patterns_total);
                println!("  Duplicates found:  {}", summary.duplicates_found);
                println!("  Prunable:          {}", summary.prunable);
                println!("  Decay - healthy:   {}", summary.decay_healthy);
                println!("  Decay - decaying:  {}", summary.decay_decaying);
                println!("  Decay - critical:  {}", summary.decay_critical);
                println!("  Patterns decayed:  {}", summary.patterns_decayed);
                println!("  Patterns promoted: {}", summary.patterns_promoted);
                println!("  Patterns demoted:  {}", summary.patterns_demoted);
                println!("  Cache cleared:     {}", summary.cache_cleared);
            }
        }
        Some(OptimizeCommand::Duplicates) => {
            let patterns = session.list_patterns();
            // Group by content_hash to find duplicates
            let mut hash_groups: std::collections::HashMap<String, Vec<String>> =
                std::collections::HashMap::new();
            for p in &patterns {
                hash_groups
                    .entry(p.content_hash.clone())
                    .or_default()
                    .push(format!("{} ({})", p.name, p.id));
            }
            let duplicates: Vec<_> = hash_groups
                .into_iter()
                .filter(|(_, v)| v.len() > 1)
                .collect();

            if json {
                println!("{}", serde_json::to_string_pretty(&duplicates)?);
            } else if duplicates.is_empty() {
                println!("No duplicate patterns found.");
            } else {
                println!("Duplicate groups ({}):", duplicates.len());
                for (hash, names) in &duplicates {
                    println!("  Hash {hash}:");
                    for n in names {
                        println!("    - {n}");
                    }
                }
            }
        }
        Some(OptimizeCommand::Prune { threshold }) => {
            let patterns = session.list_patterns();
            let prunable: Vec<_> = patterns
                .iter()
                .filter(|p| p.confidence < threshold)
                .collect();

            if json {
                println!("{}", serde_json::to_string_pretty(&prunable)?);
            } else if prunable.is_empty() {
                println!("No patterns below confidence threshold {threshold:.2}.");
            } else {
                println!(
                    "Prunable patterns ({}, below {threshold:.2}):",
                    prunable.len()
                );
                for p in &prunable {
                    println!("  {} - {} (confidence: {:.2})", p.id, p.name, p.confidence);
                }
            }
        }
    }

    Ok(())
}
