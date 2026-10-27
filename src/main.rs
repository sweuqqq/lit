use clap::Parser;
use lit::{
    commands::init,
    models::{
        cli::{self, Cli},
        config::Model,
    },
};
use tracing::info;

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
        cli::Commands::Init { path } => {
            init::run(path.clone());
            info!("Command Init executed successfully");
        }
        cli::Commands::Push => {
            println!("Push...");
        }
        cli::Commands::Config { command } => {
            match command {
                cli::CommandsConfig::Load => {
                    Model::load();
                }

                cli::CommandsConfig::Change(args) => {
                    let mut current_config = Model::load();

                    current_config.update_args(&args.new_values);

                    Model::save(&current_config);
                }
            }
            info!("Command Config executed successfully");
        }
    }
}
