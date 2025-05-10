use anyhow::Result;

mod cli;
mod command;
mod constants;
mod crypto;
mod manager;
mod serial;
mod vault;

use crate::cli::Cli;

fn main() -> Result<()> {
    Cli::parse().run()
}
