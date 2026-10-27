use std::fs;
use tracing::info;

use crate::{models::config::Model, utils::path};

impl Model {
    pub fn load() -> Model {
        if let Some(path) = path::get_config_path() {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(&path) {
                    if let Ok(config) = toml::from_str::<Model>(&content) {
                        info!("Successfully loaded the config");
                        return config;
                    }
                }
            }
        }
        info!("Loading default settings");

        let default_config = Model::default();
        Self::save(&default_config);
        default_config
    }

    pub fn save(config: &Model) {
        if let Some(path) = path::get_config_path() {
            if let Ok(toml_string) = toml::to_string_pretty(config) {
                let _ = fs::write(path, toml_string);
            }
        }
        info!("Successfully saved the config");
    }

    pub fn update_args(&mut self, args: &Model) {
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
