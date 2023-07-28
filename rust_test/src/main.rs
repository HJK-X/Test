use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}


#[derive(Subcommand)]
enum Commands {
    /// Adds fiels to app
    Add {name: Option<String>},
    
}


fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("hello {}", args.name)
    }
}
