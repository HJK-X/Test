use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}


#[derive(Subcommand)]
pub enum Commands {
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
pub struct DefaultArgs {
    /// Website URL/Name
    pub website: Option<String>,
    /// Username/Email
    pub username: Option<String>,
    /// Password/Secure Note
    pub password: Option<String>,
}