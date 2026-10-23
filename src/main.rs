use clap::Parser;
use lit::{command_enum, commands::init, managers::config};
use tracing::info;

/// Local Information Tracker
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// Debug mode
    #[arg(short, long)]
    debug: bool,

    #[command(subcommand)]
    command: command_enum::Commands,
}

fn init_logging(debug_mode: bool) {
    let level = if debug_mode {
        tracing::Level::DEBUG
    } else {
        tracing::Level::WARN
    };

    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_target(false)
        .init();
}

fn main() {
    let cli = Cli::parse();

    init_logging(cli.debug);

    match &cli.command {
        command_enum::Commands::Init { path } => {
            init::run(path.clone());
            info!("Command Init executed successfully");
        }
        command_enum::Commands::Push => {
            println!("Push...");
        }
        command_enum::Commands::Config { command } => {
            match command {
                command_enum::CommandsConfig::Load => {
                    config::load_config();
                }

                command_enum::CommandsConfig::Change(args) => {
                    let mut current_config = config::load_config();

                    current_config.update_from_args(&args.new_values);

                    config::save_config(&current_config);
                }
            }
            info!("Command Config executed successfully");
        }
    }
}
