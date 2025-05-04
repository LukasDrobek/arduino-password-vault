use anyhow::Result;

mod cli;
mod command;
mod constants;
mod vault;

use crate::cli::Cli;

fn main() -> Result<()> {
    Cli::parse().run()
}