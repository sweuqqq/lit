use clap::{Args, Subcommand};
use std::path::PathBuf;

use crate::managers;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project in currect directory
    Init {
        #[arg(short, long, value_name = "DIR", default_value = ".")]
        path: PathBuf,
    },

    /// Push local changes to the remote instance
    Push,

    /// Working with a configuration file
    Config {
        #[command(subcommand)]
        command: CommandsConfig,
    },
}

#[derive(Subcommand)]
pub enum CommandsConfig {
    /// Loading config or Generating a default config and loading
    Load,

    /// Changing settings in the config file
    Change(ConfigChanges),
}

#[derive(Args, Debug)]
pub struct ConfigChanges {
    #[command(flatten)]
    pub new_values: managers::config::Config,
}
