use clap::Parser;
use uuid::Uuid;
use std::time::{Duration, SystemTime};


mod cli;
mod password_manager;

fn main() {
    let cli: cli::Cli = cli::Cli::parse();

    let mut session_token: Option<Uuid> = None;

    match &cli.command {
        cli::Commands::Add(args) => {
            handle_add_command(args);
        }
        cli::Commands::Get(args) => {
            handle_get_command(args);
        }
        cli::Commands::List(args) => {
            handle_list_command(args);
        }
        cli::Commands::Update(args) => {
            handle_update_command(args);
        }
        cli::Commands::Delete(args) => {
            handle_delete_command(args);
        }
        cli::Commands::History(args) => {
            handle_history_command(args);
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