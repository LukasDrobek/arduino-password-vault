use anyhow::Result;
use clap::Parser;
use std::io::{self, Write};
use colored::Colorize;

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
        println!("{}", "Goodbye!".bright_blue().bold());

        Ok(())
    }

    fn dispatch_command(&self, command: &str, manager: &mut VaultManager) -> Result<()> {
        match CommandHandler::parse_command(command) {
            ParseResult::Cmd(command) => {
                CommandHandler::handle_command(command, manager)?;
            }
            ParseResult::WrongArgs { name, usage } => {
                println!("{} {}", "Error:".red().bold(), format!("Incorrect usage of '{}'", name).red());
                println!("{} {}", "Usage:".yellow().bold(), usage);
            }
            ParseResult::Unknown => {
                println!("{} {}", "Error:".red().bold(), format!("Unknown command '{}'", command).red());
                println!("{} {}", "Type".yellow(), "'help' for available commands".yellow().bold());
            }
        }
        Ok(())
    }

    fn prompt_for_command(&self) -> Result<Option<String>> {
        print!("{}", "> ".bright_blue().bold());
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
        println!("{}", "┌────────────────────────────────────────────┐".bright_blue());
        println!("{} {} {}", "│".bright_blue(), format!("Welcome to {} {}{}", APP_NAME.bright_blue().bold(), "v".bright_blue().bold(), APP_VERSION.bright_blue().bold()).bold(), "               │".bright_blue());
        println!("{} {} {}", "│".bright_blue(), "Type 'help' for commands or 'exit' to quit".italic(), "│".bright_blue());
        println!("{}", "└────────────────────────────────────────────┘".bright_blue());
        Ok(())
    }

    fn show_version(&self) -> Result<()> {
        println!("{} {}", APP_NAME.bright_green().bold(), format!("v{}", APP_VERSION.bright_cyan()).bold());
        Ok(())
    }

    fn show_no_command(&self) -> Result<()> {
        println!("{}", "No command specified".bright_yellow().bold());
        println!("Use {} for interactive mode or specify a command", "-i".green().bold());
        self.show_help()
    }

    fn show_help(&self) -> Result<()> {
        println!("{} {}", APP_NAME.bright_green().bold(), format!("v{}", APP_VERSION.bright_cyan()).bold());
        println!("- {}", APP_DESCRIPTION.italic());
        println!();

        println!("{}", "USAGE:".bold());
        println!("    {} [OPTIONS] [COMMAND]", APP_NAME.green());
        println!();

        println!("{}", "OPTIONS:".bold());
        println!("            {:<12}  {}", "-h, --help".bright_blue().bold(), "Display this help message");
        println!("            {:<12}  {}", "-i, --interactive".bright_blue().bold(), "Start in interactive mode");
        println!("            {:<12}  {}", "-v, --version".bright_blue().bold(), "Display version information");
        println!();

        println!("{}", "COMMANDS:".bold());
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
            println!("            {:<width$}  {}", cmd.bright_blue().bold(), desc, width = width);
        }

        Ok(())
    }
}
