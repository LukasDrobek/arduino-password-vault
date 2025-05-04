use clap::Parser;
use anyhow::Result;
use std::io::{self, Write};

use crate::command::{CommandHandler, Commands, ParseResult};
use crate::constants::{APP_NAME, APP_VERSION, APP_DESCRIPTION};

#[derive(Parser, Clone)]
#[command(
    name = APP_NAME,
    version = APP_VERSION,
    about = APP_DESCRIPTION,
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long)]
    interactive: bool
}

impl Cli {
    pub fn get_interactive(&self) -> bool {
        self.interactive
    }

    pub fn get_command(&self) -> Option<&Commands> {
        self.command.as_ref()
    }

    pub fn run_interactive(&self) -> Result<()> {
        println!("Welcome to {} v{}", APP_NAME, APP_VERSION);
        println!("Type 'help' to list commands or 'exit' to quit.");

        loop {
            print!("> ");
            io::stdout().flush()?;
    
            let mut line = String::new();
            io::stdin().read_line(&mut line)?;
            let command = line.trim();
            if command.is_empty() {
                continue;
            }

            match command {
                "exit" => {
                    break;
                }
                "help" => {
                    CommandHandler::show_help();
                    continue;
                }
                _ => {}
            }

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
}