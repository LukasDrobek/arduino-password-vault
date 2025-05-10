use anyhow::Result;
use clap::Subcommand;
use dialoguer::Password;
use zeroize::Zeroize;

use crate::manager::VaultManager;

#[derive(Subcommand, Clone)]
pub enum Command {
    Init,
    Add {
        service: String,
        username: String,
        password: String,
    },
    Get {
        service: Option<String>,
        username: Option<String>,
    },
    Delete {
        service: String,
        username: String,
    },
}

pub struct CommandHandler;

pub enum ParseResult {
    Cmd(Command),
    WrongArgs {
        name: &'static str,
        usage: &'static str,
    },
    Unknown,
}

impl CommandHandler {
    pub fn handle_command(command: Command, manager: &mut VaultManager) -> Result<()> {
        match command {
            Command::Init => {
                // check state
                manager.check_vault_file()?;
                if manager.is_init() {
                    println!("Already initialized");
                    return Ok(());
                }

                // initialize vault with new password
                let mut password = Password::new()
                    .with_prompt("Enter master password")
                    .with_confirmation("Confirm master password", "Passwords don't match")
                    .interact()?;
                manager.init(&password)?;
                password.zeroize();
            }

            Command::Add {
                service,
                username,
                password,
            } => {
                // check state
                manager.check_vault_file()?;
                if !manager.is_init() {
                    println!("Vault is not initialized!");
                    return Ok(());
                }

                // unlock if necessary
                if manager.is_locked() {
                    let mut password = Password::new()
                        .with_prompt("Enter master password")
                        .interact()?;
                    manager.unlock(&password)?;
                    password.zeroize();
                }

                manager.add_entry(&service, &username, &password)?;
            }

            Command::Get { service, username } => {
                // check state
                manager.check_vault_file()?;
                if !manager.is_init() {
                    println!("Vault is not initialized!");
                    return Ok(());
                }

                // unlock if necessary
                if manager.is_locked() {
                    let mut password = Password::new()
                        .with_prompt("Enter master password")
                        .interact()?;
                    manager.unlock(&password)?;
                    password.zeroize();
                }

                manager.get_entry(service, username)?;
            }

            Command::Delete { service, username } => {
                // check state
                manager.check_vault_file()?;
                if !manager.is_init() {
                    println!("Vault is not initialized!");
                    return Ok(());
                }

                // unlock if necessary
                if manager.is_locked() {
                    let mut password = Password::new()
                        .with_prompt("Enter master password")
                        .interact()?;
                    manager.unlock(&password)?;
                    password.zeroize();
                }

                manager.delete_entry(&service, &username)?;
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
            ["init"] => ParseResult::Cmd(Command::Init),

            ["add", service, username, password] => ParseResult::Cmd(Command::Add {
                service: service.to_string(),
                username: username.to_string(),
                password: password.to_string(),
            }),

            ["add", ..] => ParseResult::WrongArgs {
                name: "add",
                usage: ADD_USAGE,
            },

            ["get"] => ParseResult::Cmd(Command::Get {
                service: None,
                username: None,
            }),

            ["get", service] => ParseResult::Cmd(Command::Get {
                service: Some(service.to_string()),
                username: None,
            }),

            ["get", service, username] => ParseResult::Cmd(Command::Get {
                service: Some(service.to_string()),
                username: Some(username.to_string()),
            }),

            ["get", ..] => ParseResult::WrongArgs {
                name: "get",
                usage: GET_USAGE,
            },

            ["delete", service, username] => ParseResult::Cmd(Command::Delete {
                service: service.to_string(),
                username: username.to_string(),
            }),

            ["delete", ..] => ParseResult::WrongArgs {
                name: "delete",
                usage: DELETE_USAGE,
            },

            _ => ParseResult::Unknown,
        }
    }
}
