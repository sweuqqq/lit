use anyhow::{Context, Result, bail};
use colored::Colorize;
use std::fs;
use std::path::{Component, Path, PathBuf};
use tracing::{info, warn};

use crate::constants::{CONFIG_FILE, LIT_DIR, WATCHLIST_FILE};

pub fn get_config_path() -> Option<PathBuf> {
    let mut path = dirs::config_dir()?;
    path.push(LIT_DIR);
    fs::create_dir_all(&path).ok()?;
    path.push(CONFIG_FILE);

    info!("Config path: {}", path.display());
    Some(path)
}

pub fn resolve_repo_root(start: impl AsRef<Path>) -> Result<PathBuf> {
    let start = fs::canonicalize(start.as_ref()).context("Failed to resolve starting directory")?;
    let target_dir = if start.is_file() {
        start.parent().context("Path has no parent")?.to_path_buf()
    } else {
        start
    };

    let lit_path = target_dir.join(LIT_DIR);
    if !lit_path.is_dir() {
        warn!(
            "{}: .lit not found in {}",
            "Warning".yellow().bold(),
            target_dir.display()
        );
        fs::create_dir_all(&lit_path).context("Failed to create .lit directory")?;
    }

    Ok(target_dir)
}

pub fn get_watchlist_path(root: &Path) -> PathBuf {
    root.join(LIT_DIR).join(WATCHLIST_FILE)
}

pub fn normalize(path: &Path) -> Result<PathBuf> {
    let mut result = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !result.pop() {
                    warn!(
                        "{}: {}",
                        "Cannot normalize path".red().bold(),
                        path.display()
                    );
                }
            }
            other => result.push(other.as_os_str()),
        }
    }
    Ok(result)
}

pub fn to_portable_string(path: &Path, root: &Path) -> Result<String> {
    let relative = path.strip_prefix(root)?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

pub fn resolve_target(root: &Path, target: &Path) -> Result<PathBuf> {
    let absolute = if target.is_absolute() {
        target.to_path_buf()
    } else {
        std::env::current_dir()?.join(target)
    };

    let resolved = if absolute.exists() {
        fs::canonicalize(&absolute)?
    } else {
        normalize(&absolute)?
    };

    if !resolved.starts_with(root) {
        bail!(
            "Target must be inside project root {}: {}",
            root.display(),
            resolved.display()
        );
    }
    Ok(resolved)
}
