use clap::Error as ClapError;
use clap::Parser;
use clap_repl::ClapEditor;
use console::style;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

mod cli;
mod password_manager;

fn main() {
    let mut rl = ClapEditor::<cli::Commands>::new();

    let master_password = rpassword::prompt_password("Your password: ").unwrap();

    loop {
        let Some(command) = rl.read_command() else {
            continue;
        };

        match command {
            cli::Commands::Add(args) => {
                handle_add_command(&args, &master_password);
            }
            cli::Commands::Get(args) => {
                handle_get_command(&args, &master_password);
            }
            cli::Commands::List(args) => {
                handle_list_command(&args, &master_password);
            }
            cli::Commands::Update(args) => {
                handle_update_command(&args, &master_password);
            }
            cli::Commands::Delete(args) => {
                handle_delete_command(&args, &master_password);
            }
            cli::Commands::History(args) => {
                handle_history_command(&args, &master_password);
            }
            cli::Commands::Generate { length } => todo!(),
        }
    }
}

fn handle_add_command(args: &cli::DefaultArgs, master_password: &str) {
    println!("Adding password for {:?}", args.website);

    let mut password_entries: Vec<password_manager::PasswordEntry> = load_password_entries();

    if let Some(password) = args.password.clone() {
        let mut new_entry = password_manager::PasswordEntry::new(
            args.website.clone().unwrap_or_default(),
            args.username.clone().unwrap_or_default(),
            Vec::new(),
        );

        let encrypted_password = new_entry.encrypt_password(&password, &master_password);

        match encrypted_password {
            Ok(_) => {
                password_entries.push(new_entry);
                save_password_entries(&password_entries);
            }
            Err(err) => {
                println!("Failed to encrypt password: {}", err);
            }
        }
    } else {
        println!("Password is required to add a new entry.");
    }
}

fn handle_get_command(args: &cli::DefaultArgs, master_password: &str) {
    let password_entries: Vec<password_manager::PasswordEntry> = load_password_entries();

    if let Some(website) = &args.website {
        if let Some(entry) = password_entries
            .iter()
            .find(|entry| entry.website() == website)
        {
            let decrypted_password = entry.decrypt_password(&master_password);

            match decrypted_password {
                Ok(password) => {
                    println!("Password for {:?}: {}", website, password);
                }
                Err(err) => {
                    println!("Failed to decrypt password: {}", err);
                }
            }
        } else {
            println!("No password entry found for {:?}", website);
        }
    } else {
        println!("Website is required to retrieve a password.");
    }

    println!("The password for {:?} is {:?}", args.website, args.password);
}

fn handle_list_command(args: &cli::DefaultArgs, master_password: &str) {
    let mut password_entries: Vec<password_manager::PasswordEntry> = load_password_entries();
    if password_entries.is_empty() {
        println!("No password entries found.");
    } else {
        password_entries.sort_by(|a, b| a.website().cmp(b.website()));

        println!("Listing passwords:");
        for entry in password_entries.iter() {
            println!("Website: {}", entry.website());
            println!("\tUsername: {}", entry.username());
            let decrypted_password = entry.decrypt_password(master_password);
            match decrypted_password {
                Ok(password) => {
                    println!("\tPassword: {:?}", password);
                }
                Err(err) => {
                    println!("\tFailed to decrypt password: {}", err);
                }
            }
        }
    }
}

fn handle_update_command(args: &cli::DefaultArgs, master_password: &str) {
    // Implement the logic to update an existing password entry using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...

    println!("Updating password for {:?}", args.website);
}

fn handle_delete_command(args: &cli::DefaultArgs, master_password: &str) {
    // Implement the logic to delete a password entry using the provided arguments.
    // You can access fields like args.website, args.username, args.password, etc.
    // ...

    println!("Deleting password for {:?}", args.website);
}

fn handle_history_command(args: &cli::DefaultArgs, master_password: &str) {
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
