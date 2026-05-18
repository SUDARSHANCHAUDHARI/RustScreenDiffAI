mod cli;
mod output;

use anyhow::Result;
use clap::Parser;
use cli::{Cli, Commands};
use screendiff::{diff, report};

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Compare {
            before,
            after,
            threshold,
            json,
        } => {
            let result = diff::compare(&before, &after, threshold)?;
            let report = report::build(&before, &after, &result);
            if json {
                output::json::print(&report)?;
            } else {
                output::terminal::print(&report);
            }
        }
    }
    Ok(())
}
