//! CLI command dispatch and scan result output.

mod cli;

use std::io;

use clap::Parser;

use cli::{Cli, Command};

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Scan { path } => {
            let report = gracefulfs::scan(&path)?;
            println!("Files: {}", report.files);
            println!("Logical bytes: {}", report.logical_bytes);
            println!("Elapsed: {:?}", report.elapsed);
        }
    }
    Ok(())
}
