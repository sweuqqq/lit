use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::info;

#[derive(Serialize, Deserialize, clap::Args, Debug, Clone)]
pub struct Config {
    #[arg(long)]
    pub key: Option<String>,

    #[arg(long)]
    pub username: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key: Some("".to_string()),
            username: Some("User".to_string()),
        }
    }
}

impl Config {
    pub fn update_from_args(&mut self, args: &Config) {
        if let Some(ref k) = args.key {
            self.key = Some(k.clone());
            info!("Value of Key has changed")
        }
        if let Some(ref u) = args.username {
            self.username = Some(u.clone());
            info!("Value of Username has changed")
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    let mut path = dirs::config_dir()?;

    path.push("lit");
    fs::create_dir_all(&path).ok()?;
    path.push("config.toml");

    info!("Config path: {}", path.display());
    Some(path)
}

pub fn load_config() -> Config {
    if let Some(path) = get_config_path() {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str::<Config>(&content) {
                    info!("Successfully loaded the config");
                    return config;
                }
            }
        }
    }
    info!("Loading default settings");

    let default_config = Config::default();
    save_config(&default_config);
    default_config
}

pub fn save_config(config: &Config) {
    if let Some(path) = get_config_path() {
        if let Ok(toml_string) = toml::to_string_pretty(config) {
            let _ = fs::write(path, toml_string);
        }
    }
    info!("Successfully saved the config");
}
