use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Add a password
    Add(DefaultArgs),
    
    /// Get a password
    Get(DefaultArgs),

    /// List all stored services
    List(DefaultArgs),

    /// Update a password
    Update(DefaultArgs),

    /// Delete a password
    Delete(DefaultArgs),

    /// View password history
    History(DefaultArgs),

    
}

#[derive(Args)]
struct DefaultArgs {
    /// Website URL/Name
    website: Option<String>,
    /// Username/Email
    username: Option<String>,
    /// Password/Secure Note
    password: Option<String>,
}


fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("hello {}", args.name)
    }
}
