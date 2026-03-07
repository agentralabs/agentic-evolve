//! AgenticEvolve CLI — pattern library management tool.

use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "evolve", about = "AgenticEvolve - Pattern Library CLI")]
#[command(version)]
struct Cli {
    /// Directory for persistent pattern storage
    #[arg(long, default_value = ".agentic/evolve")]
    data_dir: String,

    /// Output JSON instead of formatted text
    #[arg(long, default_value = "false")]
    json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage stored patterns
    Pattern(commands::pattern::PatternArgs),
    /// Match patterns against signatures and queries
    Match(commands::match_cmd::MatchArgs),
    /// Crystallize code into patterns
    Crystallize(commands::crystallize::CrystallizeArgs),
    /// Crystallize code from stdin
    CrystallizeStdin(commands::crystallize::CrystallizeStdinArgs),
    /// Get function bodies from pattern matches
    Body(commands::body::BodyArgs),
    /// Compose multiple patterns together
    Compose(commands::compose::ComposeArgs),
    /// Analyze pattern coverage for source files
    Coverage(commands::coverage::CoverageArgs),
    /// Show statistics
    Stats(commands::stats::StatsArgs),
    /// Track and query usage
    Usage(commands::collective::UsageArgs),
    /// Run promotion engine
    Promote(commands::collective::PromoteArgs),
    /// Run decay check
    Decay(commands::collective::DecayArgs),
    /// Run optimization passes
    Optimize(commands::optimize::OptimizeArgs),
    /// Start MCP server
    Serve(commands::server::ServeArgs),
    /// Show system info
    Info,
    /// Show version
    Version,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Pattern(args) => commands::pattern::run(args, &cli.data_dir, cli.json),
        Commands::Match(args) => commands::match_cmd::run(args, &cli.data_dir, cli.json),
        Commands::Crystallize(args) => commands::crystallize::run(args, &cli.data_dir, cli.json),
        Commands::CrystallizeStdin(args) => {
            commands::crystallize::run_stdin(args, &cli.data_dir, cli.json)
        }
        Commands::Body(args) => commands::body::run(args, &cli.data_dir, cli.json),
        Commands::Compose(args) => commands::compose::run(args, &cli.data_dir, cli.json),
        Commands::Coverage(args) => commands::coverage::run(args, &cli.data_dir, cli.json),
        Commands::Stats(args) => commands::stats::run(args, &cli.data_dir, cli.json),
        Commands::Usage(args) => commands::collective::run_usage(args, &cli.data_dir, cli.json),
        Commands::Promote(args) => commands::collective::run_promote(args, &cli.data_dir, cli.json),
        Commands::Decay(args) => commands::collective::run_decay(args, &cli.data_dir, cli.json),
        Commands::Optimize(args) => commands::optimize::run(args, &cli.data_dir, cli.json),
        Commands::Serve(args) => commands::server::run(args, &cli.data_dir),
        Commands::Info => commands::info::run_info(&cli.data_dir, cli.json),
        Commands::Version => commands::info::run_version(cli.json),
    }
}
