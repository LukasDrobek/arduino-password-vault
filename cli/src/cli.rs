use clap::Parser;
use anyhow::Result;
use std::io::{self, Write};

use crate::command::{Commands, CommandHandler};
use crate::constants::{APP_NAME, APP_VERSION};

#[derive(Parser, Clone)]
#[command(name="vault-cli")]
#[command(about="CLI application for an Arduino-based password vault", long_about=None)]
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
    
            match command {
                "exit" => break,
                "help" => CommandHandler::show_help(),
                cmd => {
                    if let Some(command) = CommandHandler::parse_command(cmd) {
                        CommandHandler::handle_command(command)?;
                    } else {
                        println!("Unknown command: {}", command);
                        println!("Type 'help' for available commands.");
                    }
                }
            }
        }

        Ok(())
    }
}