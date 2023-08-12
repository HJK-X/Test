use clap::Parser;
use rustyline::{DefaultEditor, Result};
use uuid::Uuid;
use std::time::{Duration, SystemTime};


mod cli;
mod password_manager;

fn main() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    loop {
        let readline = rl.readline(">> ");

        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                
                if trimmed == "quit" {
                    println!("Goodbye!");
                }
                rl.add_history_entry(line.as_str());
                
                if let Ok(cli) = cli::Cli::parse_from(trimmed.split_whitespace()) {
                    match &cli.command {
                        cli::Commands::Add(args) => {
                            handle_add_command(&args);
                        }
                        cli::Commands::Get(args) => todo!(),
                        cli::Commands::List(_) => todo!(),
                        cli::Commands::Update(_) => todo!(),
                        cli::Commands::Delete(_) => todo!(),
                        cli::Commands::History(_) => todo!(),
                    }
                }

            }
            Err(_) => {
                println!("Error reading input.");
                ()
            }
        }
    }
}

fn handle_add_command(args: &cli::DefaultArgs) {
    // Implement the logic to add a new password entry using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...

}

fn handle_get_command(args: &cli::DefaultArgs) {
    // Implement the logic to retrieve the password for a specific website using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
}

fn handle_list_command(args: &cli::DefaultArgs) {
    // Implement the logic to list all stored websites/services.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
}

fn handle_update_command(args: &cli::DefaultArgs) {
    // Implement the logic to update an existing password entry using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
}

fn handle_delete_command(args: &cli::DefaultArgs) {
    // Implement the logic to delete a password entry using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
}

fn handle_history_command(args: &cli::DefaultArgs) {
    // Implement the logic to handle the "history" command if needed.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
}