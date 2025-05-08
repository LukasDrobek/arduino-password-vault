use anyhow::Result;
use clap::Subcommand;

use crate::constants::{APP_NAME, APP_VERSION, APP_DESCRIPTION};

#[derive(Subcommand, Clone)]
pub enum Command {
    Init,
    Add {
        service: String,
        username: String,
        password: String
    },
    Get {
        service: Option<String>,
        username: Option<String>
    },
    Delete {
        service: String,
        username: String
    }
}

pub struct CommandHandler;

pub enum ParseResult {
    Cmd(Command),
    WrongArgs { name: &'static str, usage: &'static str },
    Unknown
}

impl CommandHandler {
    pub fn handle_command(command: Command) -> Result<()> {
        match command {
            Command::Init => {
                println!("handle_command(init)");
            }
            Command::Add { service, username, password } => {
                println!("handle_command(add <{}> <{}> <{}>)", service, username, password);
            }
            Command::Get { service, username } => {
                match (service, username) {
                    (None, _) => {
                        println!("handle_command(get)");
                    }
                    (Some(service), None) => {
                        println!("handle_command(get [{}]", service);
                    }
                    (Some(service), Some(username)) => {
                        println!("handle_command(get [{}] [{}]", service, username);
                    }
                }
            }
            Command::Delete { service, username } => {
                println!("handle_command(delete <{}> <{}>)", service, username);
            }
        }

        Ok(())
    }

    pub fn parse_command(command_str: &str) -> ParseResult {
        let parts: Vec<&str> = command_str.split_whitespace().collect();

        const ADD_USAGE: &str = "add <service> <username> <password>";
        const GET_USAGE: &str = "get [service] [username]";
        const DELETE_USAGE: &str = "delete <service> <username>";

        match parts.as_slice() {
            ["init"] => {
                ParseResult::Cmd(Command::Init)
            }
            
            ["add", service, username, password] => {
                ParseResult::Cmd(Command::Add {
                    service: service.to_string(),
                    username: username.to_string(),
                    password: password.to_string()
                })
            }
            
            ["add", ..] => {
                ParseResult::WrongArgs { name: "add", usage: ADD_USAGE }
            }

            ["get"] => {
                ParseResult::Cmd(Command::Get {
                    service: None,
                    username: None
                })
            }

            ["get", service] => {
                ParseResult::Cmd(Command::Get {
                    service: Some(service.to_string()),
                    username: None
                })
            }

            ["get", service, username] => {
                ParseResult::Cmd(Command::Get {
                    service: Some(service.to_string()),
                    username: Some(username.to_string())
                })
            }

            ["get", ..] => {
                ParseResult::WrongArgs { name: "get", usage: GET_USAGE }
            }

            ["delete", service, username] => {
                ParseResult::Cmd(Command::Delete {
                    service: service.to_string(),
                    username: username.to_string()
                })
            }

            ["delete", ..] => {
                ParseResult::WrongArgs { name: "delete", usage: DELETE_USAGE }
            }

            _ => {
                ParseResult::Unknown
            }
        }
    }

    pub fn show_help() -> Result<()> {
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

    pub fn show_version() -> Result<()> {
        println!("{} v{}", APP_NAME, APP_VERSION);
        Ok(())
    }

    pub fn show_no_command() -> Result<()> {
        println!("No command specified.");
        println!("Use -i for interactive mode or specify a command.");
        Ok(())
    }
}
