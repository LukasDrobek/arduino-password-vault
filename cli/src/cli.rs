use clap::Parser;
use anyhow::Result;
use std::io::{self, Write};

use crate::command::{CommandHandler, ParseResult};
use crate::constants::{APP_NAME, APP_VERSION, APP_DESCRIPTION};
use crate::manager::VaultManager;

#[derive(Parser, Clone)]
#[command(
    name = APP_NAME,
    version = APP_VERSION,
    about = APP_DESCRIPTION,
    long_about = None,
    disable_help_flag = true,
    disable_version_flag = true
)]
pub struct Cli {
    #[arg(short, long)]
    help: bool,

    #[arg(short, long)]
    interactive: bool,

    #[arg(short, long)]
    version: bool,

    #[arg(num_args = 0..)]
    raw_args: Vec<String>
}

impl Cli {
    pub fn parse() -> Self {
        Parser::parse()
    }

    pub fn run(&self) -> Result<()> {
        if self.help {
            return self.show_help();
        }
        if self.version {
            return self.show_version();
        }
        if self.interactive {
            return self.run_interactive();
        }

        let raw_args = self.raw_args.join(" ");
        let command = raw_args.trim();
        if command.is_empty() {
            return self.show_no_command();
        }

        let mut manager = VaultManager::new()?;
        self.dispatch_command(command, &mut manager)?;
        Ok(())
    }

    fn run_interactive(&self) -> Result<()> {
        let mut manager = VaultManager::new()?;
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
                "-h" | "help" | "--help" => {
                    self.show_help()?;
                    continue;
                }
                "-v" | "version" | "--version" => {
                    self.show_version()?;
                }
                _ => self.dispatch_command(command, &mut manager)?
            }
        }
        Ok(())
    }

    fn dispatch_command(&self, command: &str, manager: &mut VaultManager) -> Result<()> {
        match CommandHandler::parse_command(command) {
            ParseResult::Cmd(command) => {
                CommandHandler::handle_command(command, manager)?;
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
        Ok(())
    }

    fn show_version(&self) -> Result<()> {
        println!("{} v{}", APP_NAME, APP_VERSION);
        Ok(())
    }

    fn show_no_command(&self) -> Result<()> {
        println!("No command specified.");
        println!("Use -i for interactive mode or specify a command.");
        Ok(())
    }

    fn show_help(&self) -> Result<()> {
        println!("{} v{}", APP_NAME, APP_VERSION);
        println!("{}", APP_DESCRIPTION);
        println!();

        println!("Usage:");
        println!("    {0} [options]              # single-command mode", APP_NAME);
        println!("    {0} -i, --interactive      # start in interactive (REPL) mode", APP_NAME);
        println!();

        println!("Commands:");
        let commands = [
            ("help",                                "Show this help information"),
            ("version",                             "Print version information"),
            ("init",                                "Initialize an empty vault"),
            ("add <service> <username> <passowrd>", "Add a new password entry"),
            ("get [service] [username]",            "Retrieve entries, optionally for a specific service and username"),
            ("delete <service> <username>",         "Delete a password entry"),
            ("exit",                                "Exit interactive mode"),
        ];
    
        let width = commands.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
        for (cmd, desc) in &commands {
            println!("    {:width$}    {}", cmd, desc, width = width);
        }

        Ok(())
    }
}