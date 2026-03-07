//! Collective learning subcommands — usage tracking, promotion, decay.

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_mcp::SessionManager;

// --- Usage ---

#[derive(Args)]
pub struct UsageArgs {
    #[command(subcommand)]
    pub command: UsageCommand,
}

#[derive(Subcommand)]
pub enum UsageCommand {
    /// Update usage stats for a pattern
    Update {
        /// Pattern ID
        id: String,
        /// Domain
        #[arg(long, default_value = "general")]
        domain: String,
        /// Whether the usage was successful
        #[arg(long, default_value = "true")]
        success: bool,
    },
    /// Show most used patterns
    Top {
        /// Number of results
        #[arg(long, default_value = "10")]
        limit: usize,
    },
    /// Show least used patterns
    Bottom {
        /// Number of results
        #[arg(long, default_value = "10")]
        limit: usize,
    },
}

pub fn run_usage(args: UsageArgs, data_dir: &str, json: bool) -> Result<()> {
    let mut session = SessionManager::new(data_dir)?;

    match args.command {
        UsageCommand::Update {
            id,
            domain,
            success,
        } => {
            session.update_usage(&id, &domain, success)?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "status": "updated",
                        "pattern_id": id,
                        "success": success,
                    })
                );
            } else {
                println!("Updated usage for pattern {id} (success={success})");
            }
        }
        UsageCommand::Top { limit } => {
            let patterns = session.list_patterns();
            let mut sorted: Vec<_> = patterns.into_iter().collect();
            sorted.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));
            sorted.truncate(limit);

            if json {
                println!("{}", serde_json::to_string_pretty(&sorted)?);
            } else {
                println!("Top {} patterns by usage:", sorted.len());
                for p in &sorted {
                    println!(
                        "  {} - {} uses (confidence: {:.2})",
                        p.name, p.usage_count, p.confidence
                    );
                }
            }
        }
        UsageCommand::Bottom { limit } => {
            let patterns = session.list_patterns();
            let mut sorted: Vec<_> = patterns.into_iter().collect();
            sorted.sort_by(|a, b| a.usage_count.cmp(&b.usage_count));
            sorted.truncate(limit);

            if json {
                println!("{}", serde_json::to_string_pretty(&sorted)?);
            } else {
                println!("Bottom {} patterns by usage:", sorted.len());
                for p in &sorted {
                    println!(
                        "  {} - {} uses (confidence: {:.2})",
                        p.name, p.usage_count, p.confidence
                    );
                }
            }
        }
    }

    Ok(())
}

// --- Promote ---

#[derive(Args)]
pub struct PromoteArgs;

pub fn run_promote(_args: PromoteArgs, data_dir: &str, json: bool) -> Result<()> {
    let mut session = SessionManager::new(data_dir)?;
    let summary = session.optimize()?;

    if json {
        println!(
            "{}",
            serde_json::json!({
                "promoted": summary.patterns_promoted,
                "demoted": summary.patterns_demoted,
            })
        );
    } else {
        println!("Promotion engine results:");
        println!("  Promoted: {}", summary.patterns_promoted);
        println!("  Demoted:  {}", summary.patterns_demoted);
    }

    Ok(())
}

// --- Decay ---

#[derive(Args)]
pub struct DecayArgs;

pub fn run_decay(_args: DecayArgs, data_dir: &str, json: bool) -> Result<()> {
    let mut session = SessionManager::new(data_dir)?;
    let summary = session.optimize()?;

    if json {
        println!(
            "{}",
            serde_json::json!({
                "decayed": summary.patterns_decayed,
                "healthy": summary.decay_healthy,
                "decaying": summary.decay_decaying,
                "critical": summary.decay_critical,
            })
        );
    } else {
        println!("Decay check results:");
        println!("  Patterns decayed: {}", summary.patterns_decayed);
        println!("  Healthy:          {}", summary.decay_healthy);
        println!("  Decaying:         {}", summary.decay_decaying);
        println!("  Critical:         {}", summary.decay_critical);
    }

    Ok(())
}
