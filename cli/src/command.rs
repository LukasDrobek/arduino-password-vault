use anyhow::Result;
use clap::Subcommand;
use dialoguer::{Input, Password};
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
            Command::Init => return handle_init(manager),

            Command::Add {
                service,
                username,
                password,
            } => return handle_add(manager, service, username, password),

            Command::Get { service, username } => return handle_get(manager, service, username),

            Command::Delete { service, username } => {
                return handle_delete(manager, service, username);
            }
        }
    }

    pub fn parse_command(command_str: &str) -> ParseResult {
        let parts: Vec<&str> = command_str.split_whitespace().collect();

        const ADD_USAGE: &str = "add <service> <username> <password>";
        const GET_USAGE: &str = "get [service] [username]";
        const DELETE_USAGE: &str = "delete <service> <username>";

        match parts.as_slice() {
            ["init"] => ParseResult::Cmd(Command::Init),

            ["add"] => {
                let service: String = prompt_input("Serivce");
                let username: String = prompt_input("Username");
                let password: String = prompt_input("Password");
                ParseResult::Cmd(Command::Add {
                    service,
                    username,
                    password,
                })
            }

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

            ["delete"] => {
                let service: String = prompt_input("Service");
                let username: String = prompt_input("Username");
                ParseResult::Cmd(Command::Delete { service, username })
            }

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

fn prompt_input(prompt: &str) -> String {
    Input::new()
        .with_prompt(prompt)
        .interact()
        .expect(&format!("Failed to read {}", prompt.to_lowercase()))
}

fn prompt_password(prompt: &str) -> String {
    Password::new()
        .with_prompt(prompt)
        .interact()
        .expect(&format!("Failed to read {}", prompt.to_lowercase()))
}

fn prompt_password_with_confirmation() -> String {
    Password::new()
        .with_prompt("Create master password")
        .with_confirmation("Confirm master password", "Passwords don't match")
        .interact()
        .expect("Failed to read password")
}

fn check_vault_state(manager: &mut VaultManager) -> Result<bool> {
    manager.check_vault_file()?;
    if !manager.is_init() {
        println!("Vault is not initialized!");
        return Ok(false);
    }

    if manager.is_locked() {
        let mut password = prompt_password("Enter master password");
        let result = manager.unlock(&password);
        password.zeroize();

        if let Err(e) = result {
            println!("Failed to unlock vault: {}", e);
            return Ok(false);
        }
    }

    Ok(true)
}

fn handle_init(manager: &mut VaultManager) -> Result<()> {
    // check state
    manager.check_vault_file()?;
    if manager.is_init() {
        println!("Vault already initialized.");
        return Ok(());
    }

    // initialize vault with new password
    let mut password = prompt_password_with_confirmation();
    manager.init(&password)?;
    password.zeroize();

    println!("Vault initialized successfully!");
    Ok(())
}

fn handle_add(
    manager: &mut VaultManager,
    service: String,
    username: String,
    password: String,
) -> Result<()> {
    if !check_vault_state(manager)? {
        return Ok(());
    }

    manager.add_entry(&service, &username, &password)?;
    println!("Entry added successfully!");
    Ok(())
}

fn handle_get(
    manager: &mut VaultManager,
    service: Option<String>,
    username: Option<String>,
) -> Result<()> {
    if !check_vault_state(manager)? {
        return Ok(());
    }

    let entries = manager.get_entries(service, username)?;
    if entries.is_empty() {
        println!("No entries found");
        return Ok(());
    }

    println!(
        "Found {} entr{}:",
        entries.len(),
        if entries.len() == 1 { "y" } else { "ies" }
    );
    for (i, entry) in entries.iter().enumerate() {
        println!("─────────────────────────────");
        println!("Entry #{}", i + 1);
        println!("Service: {}", entry.service());
        println!("Username: {}", entry.username());
        println!("Password: {}", entry.password());
    }
    println!("─────────────────────────────");
    Ok(())
}

fn handle_delete(manager: &mut VaultManager, service: String, username: String) -> Result<()> {
    if !check_vault_state(manager)? {
        return Ok(());
    }

    manager.delete_entry(&service, &username)?;
    println!("Entry deleted successfully");

    Ok(())
}
