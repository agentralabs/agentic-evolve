//! Stats subcommands.

use anyhow::Result;
use clap::{Args, Subcommand};

use agentic_evolve_mcp::SessionManager;

#[derive(Args)]
pub struct StatsArgs {
    #[command(subcommand)]
    pub command: Option<StatsCommand>,
}

#[derive(Subcommand)]
pub enum StatsCommand {
    /// Pattern statistics
    Patterns,
    /// Usage statistics
    Usage,
    /// Success rates
    Success,
    /// Decay report
    Decay,
}

pub fn run(args: StatsArgs, data_dir: &str, json: bool) -> Result<()> {
    let session = SessionManager::new(data_dir)?;

    match args.command {
        None => {
            // Overall statistics
            let patterns = session.list_patterns();
            let total = patterns.len();
            let total_usage: u64 = patterns.iter().map(|p| p.usage_count).sum();
            let total_success: u64 = patterns.iter().map(|p| p.success_count).sum();
            let avg_confidence: f64 = if total == 0 {
                0.0
            } else {
                patterns.iter().map(|p| p.confidence).sum::<f64>() / total as f64
            };
            let mut domains: Vec<String> = patterns.iter().map(|p| p.domain.clone()).collect();
            domains.sort();
            domains.dedup();
            let mut languages: Vec<String> =
                patterns.iter().map(|p| p.language.as_str().to_string()).collect();
            languages.sort();
            languages.dedup();

            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "total_patterns": total,
                        "total_usage": total_usage,
                        "total_success": total_success,
                        "avg_confidence": avg_confidence,
                        "domains": domains,
                        "languages": languages,
                    })
                );
            } else {
                println!("AgenticEvolve Statistics");
                println!("  Patterns:       {total}");
                println!("  Total usage:    {total_usage}");
                println!("  Total success:  {total_success}");
                println!("  Avg confidence: {avg_confidence:.2}");
                println!("  Domains:        {}", domains.join(", "));
                println!("  Languages:      {}", languages.join(", "));
            }
        }
        Some(StatsCommand::Patterns) => {
            let patterns = session.list_patterns();
            let by_domain = group_by(&patterns, |p| p.domain.clone());
            let by_lang = group_by(&patterns, |p| p.language.as_str().to_string());

            if json {
                println!(
                    "{}",
                    serde_json::json!({
                        "total": patterns.len(),
                        "by_domain": by_domain,
                        "by_language": by_lang,
                    })
                );
            } else {
                println!("Pattern statistics ({} total):", patterns.len());
                println!("  By domain:");
                for (d, c) in &by_domain {
                    println!("    {d}: {c}");
                }
                println!("  By language:");
                for (l, c) in &by_lang {
                    println!("    {l}: {c}");
                }
            }
        }
        Some(StatsCommand::Usage) => {
            let patterns = session.list_patterns();
            let mut sorted: Vec<_> = patterns.iter().collect();
            sorted.sort_by(|a, b| b.usage_count.cmp(&a.usage_count));

            if json {
                let items: Vec<_> = sorted
                    .iter()
                    .map(|p| {
                        serde_json::json!({
                            "id": p.id.as_str(),
                            "name": &p.name,
                            "usage_count": p.usage_count,
                            "success_count": p.success_count,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&items)?);
            } else {
                println!("Usage statistics:");
                for p in &sorted {
                    println!(
                        "  {} - {} uses ({} success)",
                        p.name, p.usage_count, p.success_count
                    );
                }
            }
        }
        Some(StatsCommand::Success) => {
            let patterns = session.list_patterns();
            let mut sorted: Vec<_> = patterns.iter().collect();
            sorted.sort_by(|a, b| {
                b.success_rate()
                    .partial_cmp(&a.success_rate())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            if json {
                let items: Vec<_> = sorted
                    .iter()
                    .map(|p| {
                        serde_json::json!({
                            "id": p.id.as_str(),
                            "name": &p.name,
                            "success_rate": p.success_rate(),
                            "usage_count": p.usage_count,
                            "success_count": p.success_count,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&items)?);
            } else {
                println!("Success rates:");
                for p in &sorted {
                    println!(
                        "  {} - {:.1}% ({}/{})",
                        p.name,
                        p.success_rate() * 100.0,
                        p.success_count,
                        p.usage_count
                    );
                }
            }
        }
        Some(StatsCommand::Decay) => {
            let patterns = session.list_patterns();
            let now = chrono::Utc::now().timestamp();

            if json {
                let items: Vec<_> = patterns
                    .iter()
                    .map(|p| {
                        let age_days = (now - p.last_used) / 86400;
                        serde_json::json!({
                            "id": p.id.as_str(),
                            "name": &p.name,
                            "confidence": p.confidence,
                            "days_since_use": age_days,
                        })
                    })
                    .collect();
                println!("{}", serde_json::to_string_pretty(&items)?);
            } else {
                println!("Decay report:");
                for p in &patterns {
                    let age_days = (now - p.last_used) / 86400;
                    let status = if p.confidence >= 0.7 {
                        "healthy"
                    } else if p.confidence >= 0.3 {
                        "decaying"
                    } else {
                        "critical"
                    };
                    println!(
                        "  {} - confidence={:.2} last_used={}d ago [{}]",
                        p.name, p.confidence, age_days, status
                    );
                }
            }
        }
    }

    Ok(())
}

fn group_by<F>(
    patterns: &[&agentic_evolve_core::types::pattern::Pattern],
    key_fn: F,
) -> Vec<(String, usize)>
where
    F: Fn(&&agentic_evolve_core::types::pattern::Pattern) -> String,
{
    let mut map = std::collections::HashMap::new();
    for p in patterns {
        *map.entry(key_fn(p)).or_insert(0usize) += 1;
    }
    let mut pairs: Vec<_> = map.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    pairs
}
