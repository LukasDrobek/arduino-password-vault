use anyhow::Result;
use clap::Subcommand;

use crate::constants::{APP_NAME, APP_VERSION, APP_DESCRIPTION};

#[derive(Subcommand)]
pub enum Commands {
    Init,
    Add {
        service: String,
        username: String,
        password: String
    },
    Get {
        service: String,
        username: Option<String>
    },
    List,
    Delete {
        service: String,
        username: String
    }
}

pub struct CommandHandler;

impl CommandHandler {
    pub fn handle_command(command: Commands) -> Result<()> {
        match command {
            Commands::Init => {
                println!("handle_command(init)");
            }
            Commands::Add { service, username, password } => {
                println!("handle_command(add <{}> <{}> <{}>)", service, username, password);
            }
            Commands::Get { service, username } => {
                match username {
                    Some(username) => {
                        println!("handle_command(get <{}> <{}>)", service, username);
                    }
                    None => {
                        println!("handle_command(get <{}>", service);
                    }
                }
            }
            Commands::List => {
                println!("handle_command(list)");
            }
            Commands::Delete { service, username } => {
                println!("handle_command(delete <{}> <{}>)", service, username);
            }
        }

        Ok(())
    }

    pub fn parse_command(command_str: &str) -> Option<Commands> {
        let parts: Vec<&str> = command_str.split_whitespace().collect();

        match parts.as_slice() {
            ["init"] => {
                return Some(Commands::Init);
            }
            ["add", service, username, password] => {
                return Some(Commands::Add {
                    service: service.to_string(),
                    username: username.to_string(),
                    password: password.to_string() 
                });
            }
            ["get", service] => {
                return Some(Commands::Get {
                    service: service.to_string(),
                    username: None
                })
            }
            ["get", service, username] => {
                return Some(Commands::Get {
                    service: service.to_string(),
                    username: Some(username.to_string())
                })
            }
            ["list"] => {
                return Some(Commands::List);
            }
            ["delete", service, username] => {
                return Some(Commands::Delete {
                    service: service.to_string(),
                    username: username.to_string()
                })
            }
            _ => {
                return None;
            }
        }
    }

    pub fn show_help() {
        println!("{} v{}", APP_NAME, APP_VERSION);
        println!("{}", APP_DESCRIPTION);
        println!();

        println!("Usage:");
        println!("    {0} [options]              # single-command mode", APP_NAME);
        println!("    {0} -i, --interactive      # start in interactive (REPL) mode", APP_NAME);
        println!();

        println!("Commands:");
        let commands = [
            ("help",                          "Show this help information"),
            ("version",                       "Print version information"),
            ("init",                          "Initialize an empty vault"),
            ("add <service> <username> <pw>", "Add a new password entry"),
            ("get <service> [username]",      "Retrieve entries for a service"),
            ("list",                          "List all saved entries"),
            ("delete <service> <username>",   "Delete a password entry"),
            ("exit",                          "Exit interactive mode"),
        ];
    
        let width = commands.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
        for (cmd, desc) in &commands {
            println!("    {:width$}    {}", cmd, desc, width = width);
        }
    }
}
