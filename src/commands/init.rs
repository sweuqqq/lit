use colored::Colorize;
use std::fs;
use std::path::PathBuf;
use tracing::{info, warn};

pub fn run(path: PathBuf) {
    let lit_path = path.join(".lit");
    info!("LIT-Path: {}", lit_path.display());

    if !lit_path.exists() {
        if let Err(e) = fs::create_dir_all(&lit_path) {
            warn!("{}: {}", "Failed to create directory".red().bold(), e);
            return;
        }
    }

    match std::path::absolute(&lit_path) {
        Ok(abs_path) => {
            info!("Absolute path: {}", abs_path.display());
        }
        Err(e) => {
            warn!("{}: {}", "Could not resolve absolute path".yellow(), e);
            return;
        }
    }

    println!("{}", "Successfully initialized!".green().bold());
}
