use clap::{Parser, Subcommand};
use lit::{commands::init_command, managers::config_manager};
use std::path::PathBuf;

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
    Init {
        #[arg(short, long, value_name = "DIR", default_value = ".")]
        path: PathBuf,
    },

    /// Push local changes to the remote instance
    Push,

    /// Config
    Config {
        #[command(subcommand)]
        command: CommandsConfig,
    },
}

#[derive(Subcommand)]
enum CommandsConfig {
    /// Loading config or Generating a default config and loading
    Load,

    /// Changing settings in the config file
    Change,
}

fn main() {
    let cli = Cli::parse();

    if cli.debug {
        println!("debug");
    }

    match &cli.command {
        Commands::Init { path } => {
            init_command::run(path.clone());
        }
        Commands::Push => {
            println!("Push...");
        }
        Commands::Config { command } => {
            match command {
                CommandsConfig::Load => {
                    config_manager::load_config();
                }

                CommandsConfig::Change => {}
            }
            println!("Config...")
        }
    }
}
