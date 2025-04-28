use std::process::exit;

use clap::{Parser, command};
use config::Config;
use error::Result;

pub mod config;
pub mod error;
pub mod hledger;

/// A template based report generator for the hledger accounting software.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ReportAppArgs {}

/// A wrapper function for the application logic to make error handling in `main()` easier.
fn run() -> Result<()> {
    let _args = ReportAppArgs::parse();
    let _config = Config::load()?;

    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("ERROR: {}", err);
        exit(1);
    }
}
