use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Name of person
    #[arg(short, long)]
    name: String,

    /// Number of times
    #[arg(short, long, default_value_t = 1)]
    count:u8,
}

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
