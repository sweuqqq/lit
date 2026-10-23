use colored::Colorize;
use std::fs;
use std::path::PathBuf;

pub fn run(path: PathBuf) {
    let lit_path = path.join(".lit");

    if !lit_path.exists() {
        if let Err(e) = fs::create_dir_all(&lit_path) {
            eprintln!("{}: {}", "Failed to create directory".red().bold(), e);
            return;
        }
    }

    match std::path::absolute(&lit_path) {
        Ok(abs_path) => {
            println!("{}", abs_path.display());
        }
        Err(e) => {
            eprintln!("{}: {}", "Could not resolve absolute path".yellow(), e);
            return;
        }
    }

    println!("{}", "Successfully initialized!".green().bold());
}
