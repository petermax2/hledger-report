use std::process::exit;

use clap::{Parser, command};
use error::Result;

pub mod error;

/// a template based report generator for the hledger accounting software
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct ReportAppArgs {}

fn run() -> Result<()> {
    let _args = ReportAppArgs::parse();
    Ok(())
}

fn main() {
    if let Err(err) = run() {
        eprintln!("ERROR: {}", err);
        exit(1);
    }
}
