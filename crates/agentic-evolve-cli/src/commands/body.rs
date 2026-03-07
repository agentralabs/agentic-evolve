//! Body subcommands — get function bodies from pattern matches.

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_core::types::match_result::MatchContext;
use agentic_evolve_core::types::pattern::{FunctionSignature, Language};
use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct BodyArgs {
    #[command(subcommand)]
    pub command: BodyCommand,
}

#[derive(Subcommand)]
pub enum BodyCommand {
    /// Get a function body from the best matching pattern
    Get {
        /// Function name
        #[arg(long)]
        name: String,
        /// Language
        #[arg(long, default_value = "rust")]
        language: String,
        /// Domain context
        #[arg(long)]
        domain: Option<String>,
    },
    /// Preview a pattern body by ID
    Preview {
        /// Pattern ID
        id: String,
    },
}

pub fn run(args: BodyArgs, data_dir: &str, json: bool) -> Result<()> {
    let session = SessionManager::new(data_dir)?;

    match args.command {
        BodyCommand::Get {
            name,
            language,
            domain,
        } => {
            let lang = Language::from_name(&language);
            let sig = FunctionSignature::new(&name, lang);
            let mut ctx = MatchContext::new();
            if let Some(d) = domain {
                ctx = ctx.with_domain(&d);
            }

            match session.get_body(&sig, &ctx)? {
                Some((body, pattern_id, score)) => {
                    if json {
                        println!(
                            "{}",
                            serde_json::json!({
                                "body": body,
                                "pattern_id": pattern_id,
                                "score": score,
                            })
                        );
                    } else {
                        println!("Pattern: {pattern_id} (score: {score:.3})");
                        println!("Body:");
                        for line in body.lines() {
                            println!("  {line}");
                        }
                    }
                }
                None => {
                    if json {
                        println!("{}", serde_json::json!({ "body": null }));
                    } else {
                        println!("No matching pattern found.");
                    }
                }
            }
        }
        BodyCommand::Preview { id } => {
            let pattern = session.get_pattern(&id)?;
            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "pattern_id": id,
                        "name": pattern.name,
                        "template": pattern.template,
                    })
                );
            } else {
                println!("Pattern: {} ({})", pattern.name, pattern.id);
                println!("Template:");
                for line in pattern.template.lines() {
                    println!("  {line}");
                }
            }
        }
    }

    Ok(())
}
