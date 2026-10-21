use clap::{Parser, Subcommand};

/// Local Information Tracker
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Debug mode
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new project in currect directory
    Init,

    /// Push local changes to the remote instance
    Push,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("debug");
    }

    match &cli.command {
        Commands::Init => {
            println!("Initializing...");
        }
        Commands::Push => {
            println!("Push...");
        }
    }
}
