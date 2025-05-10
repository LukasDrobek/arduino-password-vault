use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};

use crate::command::{CommandHandler, ParseResult};
use crate::constants::{APP_DESCRIPTION, APP_NAME, APP_VERSION};
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
    raw_args: Vec<String>,
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
        manager.update_vault_file()?;

        Ok(())
    }

    fn run_interactive(&self) -> Result<()> {
        let mut manager = VaultManager::new()?;
        self.show_welcome()?;

        loop {
            if let Some(cmd) = self.prompt_for_command()? {
                match cmd.as_str() {
                    "exit" | "quit" => break,
                    "-h" | "help" | "--help" => self.show_help()?,
                    "-v" | "version" | "--version" => self.show_version()?,
                    _ => self.dispatch_command(&cmd, &mut manager)?
                }
            }
        }
        manager.update_vault_file()?;
        println!("Goodbye!");

        Ok(())
    }

    fn dispatch_command(&self, command: &str, manager: &mut VaultManager) -> Result<()> {
        match CommandHandler::parse_command(command) {
            ParseResult::Cmd(command) => {
                CommandHandler::handle_command(command, manager)?;
            }
            ParseResult::WrongArgs { name, usage } => {
                println!("Error: Incorrect usage of '{}'.", name);
                println!("Usage: {}", usage);
            }
            ParseResult::Unknown => {
                println!("Error: Unknown command: '{}'.", command);
                println!("Type 'help' for available commands.");
            }
        }
        Ok(())
    }

    fn prompt_for_command(&self) -> Result<Option<String>> {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        
        let command = line.trim().to_string();
        if command.is_empty() {
            return Ok(None);
        }
        
        Ok(Some(command))
    }

    fn show_welcome(&self) -> Result<()> {
        println!("┌─────────────────────────────────────────┐");
        println!("│ Welcome to {} v{}", APP_NAME, APP_VERSION);
        println!("│ Type 'help' for commands or 'exit' to quit");
        println!("└─────────────────────────────────────────┘");
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
        println!("{} v{} - {}", APP_NAME, APP_VERSION, APP_DESCRIPTION);
        println!();

        println!("USAGE:");
        println!("    {0} [OPTIONS] [COMMAND]", APP_NAME);
        println!();

        println!("OPTIONS:");
        println!("    -h, --help         Display this help message");
        println!("    -i, --interactive  Start in interactive mode");
        println!("    -v, --version      Display version information");
        println!();

        println!("COMMANDS:");
        let commands = [
            ("init", "Initialize an empty vault"),
            ("add", "<service> <username> <password> - Add a new entry"),
            ("get", "[service] [username] - Retrieve entries"),
            ("delete", "<service> <username> - Delete an entry"),
            ("help", "Show this help information"),
            ("exit", "Exit interactive mode"),
        ];

        let width = commands.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
        for (cmd, desc) in &commands {
            println!("    {:<width$}  {}", cmd, desc, width = width);
        }
        Ok(())
    }
}
