use clap::{Args, Parser};

#[derive(Parser, Debug)]
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

    /// Generate a new password
    Generate {
        length: Option<u16>
    },

}

#[derive(Args, Debug)]
pub struct DefaultArgs {
    /// Website URL/Name
    pub website: Option<String>,
    /// Username/Email
    pub username: Option<String>,
    /// Password/Secure Note
    pub password: Option<String>,
}