// use serialport::SerialPort;
use std::io::{self, Write};

mod test_file;

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

    loop {
        print!("> ");
        io::stdout().flush()?;

        let mut command = String::new();
        io::stdin().read_line(&mut command)?;
        let command = command.trim();

        match command {
            "exit" => {
                println!("Shutting down...");
                break;
            }
            "help" => {
                println!("add <name>");
                println!("list <name>");
                println!("exit");
            }
            "add" => {
                println!("Adding password...");
            }
            "list" => {
                println!("Listing passwords...");
            }
            _ => {
                println!("Invalid command '{}'", command);
            }
        }

    }

    println!("See you soon!");
    Ok(())
}