//! Match subcommands.

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_core::types::match_result::MatchContext;
use agentic_evolve_core::types::pattern::{FunctionSignature, Language};
use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct MatchArgs {
    #[command(subcommand)]
    pub command: MatchCommand,
}

#[derive(Subcommand)]
pub enum MatchCommand {
    /// Match patterns by function signature
    Signature {
        /// Function name
        #[arg(long)]
        name: String,
        /// Language
        #[arg(long, default_value = "rust")]
        language: String,
        /// Maximum results
        #[arg(long, default_value = "5")]
        limit: usize,
    },
    /// Match patterns with context
    Context {
        /// Function name
        #[arg(long)]
        name: String,
        /// Language
        #[arg(long, default_value = "rust")]
        language: String,
        /// Domain context
        #[arg(long)]
        domain: Option<String>,
        /// Surrounding code
        #[arg(long)]
        surrounding: Option<String>,
        /// Maximum results
        #[arg(long, default_value = "5")]
        limit: usize,
    },
    /// Semantic search for patterns
    Semantic {
        /// Search query
        query: String,
        /// Maximum results
        #[arg(long, default_value = "5")]
        limit: usize,
    },
    /// Fuzzy search for patterns
    Fuzzy {
        /// Search query
        query: String,
        /// Maximum results
        #[arg(long, default_value = "5")]
        limit: usize,
    },
}

pub fn run(args: MatchArgs, data_dir: &str, json: bool) -> Result<()> {
    let session = SessionManager::new(data_dir)?;

    match args.command {
        MatchCommand::Signature {
            name,
            language,
            limit,
        } => {
            let lang = Language::from_name(&language);
            let sig = FunctionSignature::new(&name, lang);
            let ctx = MatchContext::new().with_max_results(limit);
            let results = session.match_signature(&sig, &ctx, limit)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("Signature matches ({}):", results.len());
                for r in &results {
                    println!(
                        "  {} - score: {:.3} (sig={:.2} ctx={:.2} sem={:.2} conf={:.2})",
                        r.pattern_id,
                        r.score.combined,
                        r.score.signature_score,
                        r.score.context_score,
                        r.score.semantic_score,
                        r.score.confidence_score,
                    );
                }
            }
        }
        MatchCommand::Context {
            name,
            language,
            domain,
            surrounding,
            limit,
        } => {
            let lang = Language::from_name(&language);
            let sig = FunctionSignature::new(&name, lang);
            let mut ctx = MatchContext::new().with_max_results(limit);
            if let Some(d) = domain {
                ctx = ctx.with_domain(&d);
            }
            if let Some(s) = surrounding {
                ctx = ctx.with_surrounding_code(&s);
            }
            let results = session.match_context(&sig, &ctx, limit)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("Context matches ({}):", results.len());
                for r in &results {
                    println!(
                        "  {} - score: {:.3} (sig={:.2} ctx={:.2} sem={:.2} conf={:.2})",
                        r.pattern_id,
                        r.score.combined,
                        r.score.signature_score,
                        r.score.context_score,
                        r.score.semantic_score,
                        r.score.confidence_score,
                    );
                }
            }
        }
        MatchCommand::Semantic { query, limit } => {
            let results = session.search_patterns(&query);
            let limited: Vec<_> = results.into_iter().take(limit).collect();

            if json {
                println!("{}", serde_json::to_string_pretty(&limited)?);
            } else {
                println!("Semantic matches ({}):", limited.len());
                for p in &limited {
                    println!(
                        "  {} - {} [{}] confidence={:.2}",
                        p.id, p.name, p.domain, p.confidence
                    );
                }
            }
        }
        MatchCommand::Fuzzy { query, limit } => {
            let results = session.search_patterns(&query);
            let limited: Vec<_> = results.into_iter().take(limit).collect();

            if json {
                println!("{}", serde_json::to_string_pretty(&limited)?);
            } else {
                println!("Fuzzy matches ({}):", limited.len());
                for p in &limited {
                    println!(
                        "  {} - {} [{}] confidence={:.2}",
                        p.id, p.name, p.domain, p.confidence
                    );
                }
            }
        }
    }

    Ok(())
}
