//! Command-line arguments and subcommands.

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Command,
}

#[derive(Subcommand)]
pub(crate) enum Command {
    /// Count files and total logical size
    Scan {
        /// Path to scan
        path: PathBuf,
    },
}
