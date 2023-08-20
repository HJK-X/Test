
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Editor, history::FileHistory};
use serde_json;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

mod password_manager;

const MASTER_SALT: &[u8; 12] = b"uniquemaster";
const MASTER_HASH_FILE: &str = "master_password_hash.bin";

fn main() -> Result<(), String> {
    if master_hash_file_exists() {
        let master_password: String =
            rpassword::prompt_password("Input your master password: ").unwrap();
        let master_password_confirmation =
            rpassword::prompt_password("Retype your master password: ").unwrap();
        if master_password == master_password_confirmation {
            save_master_password_hash(&master_password)?;
        } else {
            panic!("Master password does not match")
        }
    }

    let master_password_hash: [u8; 32] = load_master_password_hash()?;

    let master_password = rpassword::prompt_password("Your password: ").unwrap();

    if password_manager::derive_key_from_master_password(&master_password, MASTER_SALT)? == master_password_hash {
        println!("Logged in");
    } else {
        panic!("Master password was incorrect")
    }

    let mut rl = DefaultEditor::new().map_err(|_| "Could not start rustyline")?;
    let mut password_entries: Vec<password_manager::PasswordEntry> = load_password_entries();
    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                println!("Line: {}", line);
                println!("{}", master_password);
                let trimmed = line.trim();

                match trimmed {
                    "add" => {
                        let args: DefaultArgs = handle_default_args(&rl)?;
                        handle_add_command(args, &master_password, password_entries);
                    },
                    "get" => {
                        let args: DefaultArgs = handle_default_args(&rl)?;
                        handle_get_command(args, &master_password);
                    },
                    "update" => {
                        let args: DefaultArgs = handle_default_args(&rl)?;
                        handle_get_command(args, &master_password);
                    },
                    "delete" => {
                        let args: DefaultArgs = handle_default_args(&rl)?;
                        handle_get_command(args, &master_password);
                    },
                    "list" => {
                        let args: DefaultArgs = handle_default_args(&rl)?;
                        handle_get_command(args, &master_password);
                    },
                    "quit" => {
                        println!("bye");
                        save_password_entries(&password_entries);
                        break;
                    }
                    _ => println!("Unknown command"),
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("interrupted");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    Ok(())

    // let mut rl = ClapEditor::<cli::Commands>::new();

    // let master_password = rpassword::prompt_password("Your password: ").unwrap();

    // loop {
    //     let Some(command) = rl.read_command() else {
    //         continue;
    //     };

    //     match command {
    //         cli::Commands::Add(args) => {
    //             handle_add_command(&args, &master_password);
    //         }
    //         cli::Commands::Get(args) => {
    //             handle_get_command(&args, &master_password);
    //         }
    //         cli::Commands::List(args) => {
    //             handle_list_command(&args, &master_password);
    //         }
    //         cli::Commands::Update(args) => {
    //             handle_update_command(&args, &master_password);
    //         }
    //         cli::Commands::Delete(args) => {
    //             handle_delete_command(&args, &master_password);
    //         }
    //         cli::Commands::Generate { length } => todo!(),
    //     }
    // }
}

fn master_hash_file_exists() -> bool {
    Path::new(MASTER_HASH_FILE).exists()
}

fn save_master_password_hash(master_password: &str) -> Result<(), String> {
    let hash: [u8; 32] =
        password_manager::derive_key_from_master_password(master_password, MASTER_SALT)
            .map_err(|_| "Failed to derive key from master password")?;

    fs::write(MASTER_HASH_FILE, &hash).map_err(|_| "Failed to save hash to file")?;

    println!("Master password hash saved successfully.");
    Ok(())
}

fn load_master_password_hash() -> Result<[u8; 32], String> {
    if master_hash_file_exists() {
        return Err("Hash file does not exist".into());
    }
    let hash = fs::read(MASTER_HASH_FILE).map_err(|_| "Failed to load hash from file")?;

    if hash.len() != 32 {
        return Err("Invalid hash length".into());
    }

    let mut hash_array: [u8; 32] = [0u8; 32];
    hash_array.copy_from_slice(&hash);

    Ok(hash_array)
}

fn handle_default_args(rl: &Editor<(), FileHistory>) -> Result<DefaultArgs, String> {
    let website: String = rl
        .readline("Website: ")
        .map(|line| line.trim().to_string())
        .map_err(|_| "Readline failed")?;
    let username: String = rl
        .readline("Website: ")
        .map(|line| line.trim().to_string())
        .map_err(|_| "Readline failed")?;
    let password: String = rl
        .readline("Website: ")
        .map(|line| line.trim().to_string())
        .map_err(|_| "Readline failed")?;

    Ok(DefaultArgs {
        website,
        username,
        password,
    })
}

fn handle_add_command(args: DefaultArgs, master_password: &str, password_entries: Vec<password_manager::PasswordEntry>) {
    println!("Adding password for {:?}", args.website);

    let mut new_entry: password_manager::PasswordEntry = password_manager::PasswordEntry::new(
        args.website.clone(),
        args.username.clone(),
        Vec::new(),
    );

    let encrypted_password = new_entry.encrypt_password(&args.password, &master_password);

    match encrypted_password {
        Ok(_) => {
            password_entries.push(new_entry);
            save_password_entries(&password_entries);
        }
        Err(err) => {
            println!("Failed to encrypt password: {}", err);
        }
    }
}

fn handle_get_command(args: DefaultArgs, master_password: &str,password_entries: Vec<password_manager::PasswordEntry>) {
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

fn handle_list_command(args: DefaultArgs, master_password: &str,password_entries: Vec<password_manager::PasswordEntry>) {
    let mut password_entries: Vec<password_manager::PasswordEntry> = load_password_entries();
    if password_entries.is_empty() {
        println!("No password entries found.");
    } else {
        password_entries.sort_by(|a, b| a.website().cmp(b.website()));

        println!("Listing passwords:");
        for entry in password_entries.iter() {
            if let Some(website) = &args.website {
                if entry.website() != website {
                    continue;
                }
            }
            if let Some(username) = &args.username {
                if entry.username() != username {
                    continue;
                }
            }
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

fn handle_update_command(args: DefaultArgs, master_password: &str,password_entries: Vec<password_manager::PasswordEntry>) {
    if password_entries.is_empty() {
        println!("No password entries found.");
    } else {
        for entry in password_entries.iter_mut() {
            if let Some(website) = &args.website {
                if entry.website() == website {
                    entry.update_fields(
                        args.username.clone(),
                        args.password.clone(),
                        master_password,
                    );
                }
            }
        }

        save_password_entries(&password_entries);

        println!("Password entry updated successfully.");
    }
}

fn load_password_entries() -> Vec<password_manager::PasswordEntry> {
    let mut file = fs::OpenOptions::new()
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
    let mut file = fs::OpenOptions::new()
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

struct DefaultArgs {
    /// Website URL/Name
    website: String,
    /// Username/Email
    username: String,
    /// Password/Secure Note
    password: String,
}
