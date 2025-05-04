use anyhow::Result;
use clap::Parser;

mod cli;
mod command;
mod constants;
// mod vault;

use cli::Cli;
use command::CommandHandler;

fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.get_interactive() {
        cli.run_interactive()?;
    } else if let Some(cmd) = cli.get_command() {
        CommandHandler::handle_command(cmd.clone())?;
    } else {
        println!("No command specified.");
        println!("Use -i for interactive mode or specify a command.")
    }

    Ok(())
}