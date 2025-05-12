use anyhow::Result;
use clap::Subcommand;
use colored::Colorize;
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
    Reset,
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

            Command::Reset => return handle_reset(manager),
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

            ["reset"] => ParseResult::Cmd(Command::Reset),

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
        println!("{}", "Vault is not initialized!".bright_blue().bold());
        return Ok(false);
    }

    if manager.is_locked() {
        let mut password = prompt_password("Enter master password");
        let result = manager.unlock(&password);
        password.zeroize();

        if let Err(e) = result {
            println!("{} {}", "Failed to unlock vault:".bright_blue().bold(), e);
            return Ok(false);
        }
    }

    Ok(true)
}

fn handle_init(manager: &mut VaultManager) -> Result<()> {
    // check state
    manager.check_vault_file()?;
    if manager.is_init() {
        println!("{}", "Vault is already initialized".yellow().bold());
        return Ok(());
    }

    // initialize vault with new password
    let mut password = prompt_password_with_confirmation();
    manager.init(&password)?;
    password.zeroize();

    println!("{}", "Vault initialized successfully".bright_blue().bold());
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

    if manager.add_entry(&service, &username, &password)? {
        println!("{}", "Entry added successfully".bright_blue().bold());
        return Ok(());
    }
    println!("{}", "Entry already exists".yellow().bold());
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
        println!("{}", "No entries found".yellow().bold());
        return Ok(());
    }

    let label = if entries.len() == 1 {
        "entry"
    } else {
        "entries "
    };
    println!(
        "{} {}",
        "Found".green(),
        format!("{} {}", entries.len(), label).green().bold()
    );
    for (i, entry) in entries.iter().enumerate() {
        println!("{}", "─────────────────────────────".bright_black());
        println!(
            "{}{}",
            "Entry #".bright_blue().bold(),
            (i + 1).to_string().blue().bold()
        );
        println!("{} {}", "Service:".bold(), entry.service().blue());
        println!("{} {}", "Username:".bold(), entry.username().bright_blue());
        println!("{} {}", "Password:".bold(), entry.password().green());
    }
    println!("{}", "─────────────────────────────".bright_black());
    Ok(())
}

fn handle_delete(manager: &mut VaultManager, service: String, username: String) -> Result<()> {
    if !check_vault_state(manager)? {
        return Ok(());
    }

    if manager.delete_entry(&service, &username)? {
        println!("{}", "Entry deleted successfully".green().bold());
        return Ok(());
    }
    println!(
        "{}",
        format!(
            "{} '{}' {} '{}'",
            "No entry found for service", service, "and username", username
        )
        .yellow()
        .bold()
    );
    Ok(())
}

fn handle_reset(manager: &mut VaultManager) -> Result<()> {
    if !check_vault_state(manager)? {
        return Ok(())
    }

    println!("{}", "This action will permanently erase all saved password.".red().bold());
    let input = prompt_input("Do you want to proceed? [yes/no]");
    if input.trim().to_lowercase() != "yes" {
        println!("{}", "Reset aborted. No changes were made.".bright_blue().bold());
        return Ok(())
    }

    if manager.reset_vault()? {
        println!("{}", "Vault has been successfully reset!".green().bold());
    } else {
        println!("{}", "Failed to reset the vault!".yellow().bold());
    }
    Ok(())
}