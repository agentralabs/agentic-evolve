//! Info and version subcommands.

use anyhow::Result;

use agentic_evolve_mcp::SessionManager;

pub fn run_info(data_dir: &str, json: bool) -> Result<()> {
    let session = SessionManager::new(data_dir)?;
    let count = session.pattern_count();

    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");

    if json {
        println!(
            "{}",
            serde_json::json!({
                "name": name,
                "version": version,
                "data_dir": data_dir,
                "pattern_count": count,
                "engine": "AgenticEvolve",
            })
        );
    } else {
        println!("AgenticEvolve");
        println!("  Version:       {version}");
        println!("  Package:       {name}");
        println!("  Data dir:      {data_dir}");
        println!("  Patterns:      {count}");
    }

    Ok(())
}

pub fn run_version(json: bool) -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");

    if json {
        println!("{}", serde_json::json!({ "version": version }));
    } else {
        println!("evolve {version}");
    }

    Ok(())
}
