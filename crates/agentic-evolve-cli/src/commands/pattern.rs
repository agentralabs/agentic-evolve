//! Pattern management subcommands.

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_core::types::pattern::{FunctionSignature, Language, PatternVariable};
use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct PatternArgs {
    #[command(subcommand)]
    pub command: PatternCommand,
}

#[derive(Subcommand)]
pub enum PatternCommand {
    /// Store a pattern from JSON (reads from stdin)
    Store,
    /// Get a pattern by ID
    Get {
        /// Pattern ID
        id: String,
    },
    /// Search patterns by query
    Search {
        /// Search query
        query: String,
    },
    /// List all patterns
    List {
        /// Filter by domain
        #[arg(long)]
        domain: Option<String>,
        /// Filter by language
        #[arg(long)]
        language: Option<String>,
    },
    /// Delete a pattern by ID
    Delete {
        /// Pattern ID
        id: String,
    },
    /// Export a pattern to JSON
    Export {
        /// Pattern ID
        id: String,
    },
    /// Import a pattern from a JSON file
    Import {
        /// Path to JSON file
        file: String,
    },
    /// Count total patterns
    Count,
    /// List all tags
    Tags,
    /// List all domains
    Domains,
}

pub fn run(args: PatternArgs, data_dir: &str, json: bool) -> Result<()> {
    let mut session = SessionManager::new(data_dir)?;

    match args.command {
        PatternCommand::Store => {
            let input = std::io::read_to_string(std::io::stdin())?;
            let v: serde_json::Value = serde_json::from_str(&input)?;

            let name = v["name"].as_str().unwrap_or("unnamed");
            let domain = v["domain"].as_str().unwrap_or("general");
            let lang = Language::from_name(v["language"].as_str().unwrap_or("rust"));
            let template = v["template"].as_str().unwrap_or("");
            let confidence = v["confidence"].as_f64().unwrap_or(0.5);
            let tags: Vec<String> = v["tags"]
                .as_array()
                .map(|a| a.iter().filter_map(|t| t.as_str().map(String::from)).collect())
                .unwrap_or_default();

            let sig_name = v["signature"]["name"].as_str().unwrap_or(name);
            let signature = FunctionSignature::new(sig_name, lang.clone());

            let variables: Vec<PatternVariable> = v["variables"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .collect()
                })
                .unwrap_or_default();

            let pattern =
                session.store_pattern(name, domain, lang, signature, template, variables, confidence, tags)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&pattern)?);
            } else {
                println!("Stored pattern: {} ({})", pattern.name, pattern.id);
            }
        }
        PatternCommand::Get { id } => {
            let pattern = session.get_pattern(&id)?;
            if json {
                println!("{}", serde_json::to_string_pretty(pattern)?);
            } else {
                print_pattern(pattern);
            }
        }
        PatternCommand::Search { query } => {
            let results = session.search_patterns(&query);
            if json {
                println!("{}", serde_json::to_string_pretty(&results)?);
            } else {
                println!("Found {} pattern(s):", results.len());
                for p in &results {
                    println!("  {} - {} [{}] (confidence: {:.2})", p.id, p.name, p.domain, p.confidence);
                }
            }
        }
        PatternCommand::List { domain, language } => {
            let patterns = session.list_patterns();
            let filtered: Vec<_> = patterns
                .into_iter()
                .filter(|p| {
                    domain.as_ref().is_none_or(|d| &p.domain == d)
                        && language
                            .as_ref()
                            .is_none_or(|l| p.language.as_str() == l.as_str())
                })
                .collect();

            if json {
                println!("{}", serde_json::to_string_pretty(&filtered)?);
            } else {
                println!("Patterns ({}):", filtered.len());
                for p in &filtered {
                    println!(
                        "  {} - {} [{}] lang={} confidence={:.2}",
                        p.id, p.name, p.domain, p.language, p.confidence
                    );
                }
            }
        }
        PatternCommand::Delete { id } => {
            let pattern = session.delete_pattern(&id)?;
            if json {
                println!("{}", serde_json::to_string_pretty(&pattern)?);
            } else {
                println!("Deleted pattern: {} ({})", pattern.name, pattern.id);
            }
        }
        PatternCommand::Export { id } => {
            let pattern = session.get_pattern(&id)?;
            println!("{}", serde_json::to_string_pretty(pattern)?);
        }
        PatternCommand::Import { file } => {
            let content = std::fs::read_to_string(&file)?;
            let v: serde_json::Value = serde_json::from_str(&content)?;

            let name = v["name"].as_str().unwrap_or("unnamed");
            let domain = v["domain"].as_str().unwrap_or("general");
            let lang = Language::from_name(v["language"].as_str().unwrap_or("rust"));
            let template = v["template"].as_str().unwrap_or("");
            let confidence = v["confidence"].as_f64().unwrap_or(0.5);
            let tags: Vec<String> = v["tags"]
                .as_array()
                .map(|a| a.iter().filter_map(|t| t.as_str().map(String::from)).collect())
                .unwrap_or_default();

            let sig_name = v["signature"]["name"].as_str().unwrap_or(name);
            let signature = FunctionSignature::new(sig_name, lang.clone());

            let variables: Vec<PatternVariable> = v["variables"]
                .as_array()
                .map(|a| {
                    a.iter()
                        .filter_map(|v| serde_json::from_value(v.clone()).ok())
                        .collect()
                })
                .unwrap_or_default();

            let pattern =
                session.store_pattern(name, domain, lang, signature, template, variables, confidence, tags)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&pattern)?);
            } else {
                println!("Imported pattern: {} ({}) from {}", pattern.name, pattern.id, file);
            }
        }
        PatternCommand::Count => {
            let count = session.pattern_count();
            if json {
                println!("{}", serde_json::json!({ "count": count }));
            } else {
                println!("Total patterns: {count}");
            }
        }
        PatternCommand::Tags => {
            let patterns = session.list_patterns();
            let mut tags: Vec<String> = patterns
                .iter()
                .flat_map(|p| p.tags.clone())
                .collect();
            tags.sort();
            tags.dedup();

            if json {
                println!("{}", serde_json::to_string_pretty(&tags)?);
            } else {
                println!("Tags ({}):", tags.len());
                for tag in &tags {
                    println!("  {tag}");
                }
            }
        }
        PatternCommand::Domains => {
            let patterns = session.list_patterns();
            let mut domains: Vec<String> = patterns.iter().map(|p| p.domain.clone()).collect();
            domains.sort();
            domains.dedup();

            if json {
                println!("{}", serde_json::to_string_pretty(&domains)?);
            } else {
                println!("Domains ({}):", domains.len());
                for d in &domains {
                    println!("  {d}");
                }
            }
        }
    }

    Ok(())
}

fn print_pattern(p: &agentic_evolve_core::types::pattern::Pattern) {
    println!("Pattern: {}", p.name);
    println!("  ID:         {}", p.id);
    println!("  Domain:     {}", p.domain);
    println!("  Language:   {}", p.language);
    println!("  Confidence: {:.2}", p.confidence);
    println!("  Usage:      {} (success: {})", p.usage_count, p.success_count);
    println!("  Version:    {}", p.version);
    println!("  Tags:       {}", p.tags.join(", "));
    println!("  Template:");
    for line in p.template.lines() {
        println!("    {line}");
    }
}
