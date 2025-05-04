use anyhow::Result;
use clap::Parser;

mod cli;
mod command;
mod constants;
mod vault;

use cli::Cli;
use command::{ParseResult, CommandHandler};

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.get_help() {
        CommandHandler::show_help();
        return Ok(());
    }

    if cli.get_version() {
        CommandHandler::show_version();
        return Ok(());
    }

    if cli.get_interactive() {
        cli.run_interactive()?;
    } else if cli.get_raw().is_empty() {
        CommandHandler::show_no_command();
    } else {
        let line = cli.get_raw().join(" ");
        let command = line.trim();

        match CommandHandler::parse_command(command) {
            ParseResult::Cmd(command) => {
                CommandHandler::handle_command(command)?;
            }
            ParseResult::WrongArgs { name, usage } => {
                println!("Incorrect usage of '{}'.", name);
                println!("Usage: {}", usage);
            }
            ParseResult::Unknown => {
                println!("Unknown command: '{}'.", command);
                println!("Type 'help' for available commands.");
            }
        }
    }

    Ok(())
}