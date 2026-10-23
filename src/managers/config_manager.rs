use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub key: String,
    pub username: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            key: "".to_string(),
            username: "User".to_string(),
        }
    }
}

fn get_config_path() -> Option<PathBuf> {
    let mut path = dirs::config_dir()?;
    path.push("lit");
    fs::create_dir_all(&path).ok()?;
    path.push("config.toml");
    Some(path)
}

pub fn load_config() -> Config {
    if let Some(path) = get_config_path() {
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(config) = toml::from_str::<Config>(&content) {
                    return config;
                }
            }
        }
    }

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
}
