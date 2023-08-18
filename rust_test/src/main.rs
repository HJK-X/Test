use clap::Error as ClapError;
use clap::Parser;
use clap_repl::ClapEditor;
use console::style;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use serde::{Serialize, Deserialize};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

mod cli;
mod password_manager;

fn main() {
    let mut rl = ClapEditor::<cli::Commands>::new();

    let password = rpassword::prompt_password("Your password: ").unwrap();

    loop {
        let Some(command) = rl.read_command() else {
            continue;
        };

        match command {
            cli::Commands::Add(args) => {
                handle_add_command(&args);
            }
            cli::Commands::Get(args) => {
                handle_get_command(&args);
            }
            cli::Commands::List(args) => {
                handle_list_command(&args);
            }
            cli::Commands::Update(args) => {
                handle_update_command(&args);
            }
            cli::Commands::Delete(args) => {
                handle_delete_command(&args);
            }
            cli::Commands::History(args) => {
                handle_history_command(&args);
            }
        }
    }
}

fn handle_add_command(args: &cli::DefaultArgs) {
    println!("Adding password for {:?}", args.website);

    let mut password_entries: Vec<password_manager::PasswordEntry> = load_password_entries();

    let new_entry = password_manager::PasswordEntry::new(
        args.website.clone().unwrap_or_default(),
        args.username.clone().unwrap_or_default(),
        args.password.clone().unwrap_or_default().into_bytes(),
    );

    password_entries.push(new_entry);
    save_password_entries(&password_entries);

    println!("Added password for {:?}", args.website);
}

fn handle_get_command(args: &cli::DefaultArgs) {
    // Implement the logic to retrieve the password for a specific website using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...

    println!("The password for {:?} is {:?}", args.website, args.password);
}

fn handle_list_command(args: &cli::DefaultArgs) {
    // Implement the logic to list all stored websites/services.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...

    println!("Listing passwords:");
}

fn handle_update_command(args: &cli::DefaultArgs) {
    // Implement the logic to update an existing password entry using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
    
    println!("Updating password for {:?}", args.website);
}

fn handle_delete_command(args: &cli::DefaultArgs) {
    // Implement the logic to delete a password entry using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
    
    println!("Deleting password for {:?}", args.website);
}

fn handle_history_command(args: &cli::DefaultArgs) {
    // Implement the logic to handle the "history" command if needed.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...
    
    println!("Password history for {:?}:", args.website);
}


fn load_password_entries() -> Vec<password_manager::PasswordEntry> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("passwords.json")
        .expect("Failed to open passwords file");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read passwords file");

    serde_json::from_str(&contents).unwrap_or_default()
}

fn save_password_entries(entries: &[password_manager::PasswordEntry]) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("passwords.json")
        .expect("Failed to open passwords file");

    let json_string = serde_json::to_string_pretty(entries)
        .expect("Failed to serialize password entries to JSON");

    file.write_all(json_string.as_bytes())
        .expect("Failed to write password entries to file");
}