//! CM CLI - Command-line interface for CM game.

use clap::{Parser, Subcommand};
use tracing_subscriber::EnvFilter;

use cm_telemetry::tracing::init_tracing;

mod commands;
mod errors;
mod output;

#[derive(Parser, Debug)]
#[command(name = "cm", version, about = "CM01/02-style manager sim (Rust)")]
struct Cli {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Create a new game
    NewGame(commands::new_game::NewGameArgs),
    /// Advance simulation by days
    AdvanceDay(commands::advance_day::AdvanceDayArgs),
    /// Simulate a single match
    SimulateMatch(commands::simulate_match::SimulateMatchArgs),
}

fn main() -> anyhow::Result<()> {
    init_tracing(EnvFilter::from_default_env().add_directive("info".parse()?));
    let cli = Cli::parse();

    match cli.cmd {
        Command::NewGame(args) => commands::new_game::run(args),
        Command::AdvanceDay(args) => commands::advance_day::run(args),
        Command::SimulateMatch(args) => commands::simulate_match::run(args),
    }
}
