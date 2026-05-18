use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "screendiff",
    about = "Screenshot pixel diff engine — detect visual regressions",
    version
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Compare two screenshots
    Compare {
        /// Path to before screenshot
        before: String,
        /// Path to after screenshot
        after: String,
        /// Pixel difference threshold 0.0-1.0 (default: 0.01 = 1%)
        #[arg(long, default_value = "0.01")]
        threshold: f64,
        /// Output as JSON
        #[arg(long)]
        json: bool,
    },
}
