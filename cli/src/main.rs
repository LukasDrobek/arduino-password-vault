// use serialport::SerialPort;
use std::io::{self, Write};

mod vault;
mod test_file;

use vault::Vault;

const APP_NAME: &str = "arduino-password-vault";
const APP_VERSION: &str = "0.1.0";

fn test(test_num: i32) -> io::Result<()> {
    if test_num == 1 {
        test_file::test_connection()?; 
    }
    if test_num == 2 {
        test_file::test_argon()?;
    }
    if test_num == 3 {
        test_file::test_aes256gcm()?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let test_num = 0;
    test(test_num)?;

    println!("Welcome to {} v{}!", APP_NAME, APP_VERSION);
    println!("Type 'help' to list commands or 'exit' to quit.");

    let mut vault = Vault::new();

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        match parts.as_slice() {
            ["add", service, username, password] => {
                vault.add_entry(
                    service.to_string(),
                    username.to_string(),
                    password.to_string());
            }
            ["add", ..] => {
                println!("Usage: add <service> <username> <password>");
            }

            ["list"] => {
                vault.list_all_entries();
            }

            ["find", service] => {
                vault.find_entries(service.to_string());
            }
            ["find", ..] => {
                println!("Usage: find <service>");
            }

            ["get", service, username] => {
                vault.get_password(
                    service.to_string(),
                    username.to_string());
            }
            ["get", ..] => {
                println!("Usage: get <service> <username>");
            }

            ["help"] => {
                println!("Available commands:");
                println!("\tadd <service> <username> <password>");
                println!("\tlist");
                println!("\tfind <service>");
                println!("\tget <service> <username>");
                println!("\thelp");
                println!("\texit");
            }

            ["exit"] => {
                break;
            }

            _ => {
                println!("Invalid command - type 'help' to see available commands");
            }
        }
    }

    Ok(())
}