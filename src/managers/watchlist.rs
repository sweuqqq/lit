use anyhow::{Context, Result, bail};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::{Component, PathBuf};
use walkdir::WalkDir;

use crate::constants::{LIT_DIR, WATCHLIST_FILE};
use crate::models::watchlist::Watchlist;
use crate::utils::path::{get_watchlist_path, resolve_target, to_portable_string};

pub struct WatchlistManager;

impl WatchlistManager {
    /// Загружает все отслеживаемые файлы
    pub fn load(watchlist: &Watchlist) -> Result<Vec<PathBuf>> {
        let path = get_watchlist_path(&watchlist.root);
        if !path.exists() {
            return Ok(Vec::new());
        }

        let file =
            File::open(&path).with_context(|| format!("Failed to open {}", path.display()))?;
        let mut entries = Vec::new();

        for line in BufReader::new(file).lines() {
            let line = line?;
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let entry = PathBuf::from(trimmed);
            if entry.is_absolute()
                || entry
                    .components()
                    .any(|c| matches!(c, Component::ParentDir | Component::RootDir))
            {
                bail!("Invalid path in watchlist: {trimmed}");
            }
            entries.push(entry);
        }

        entries.sort();
        entries.dedup();
        Ok(entries)
    }

    /// Сохраняет записи в файл ватчлиста
    pub fn save(watchlist: &Watchlist, mut entries: Vec<PathBuf>) -> Result<()> {
        let path = get_watchlist_path(&watchlist.root);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        entries.sort();
        entries.dedup();

        let file =
            File::create(&path).with_context(|| format!("Failed to create {}", path.display()))?;
        let mut writer = BufWriter::new(file);

        for entry in entries {
            writeln!(writer, "{}", entry.display())?;
        }
        writer.flush()?;
        Ok(())
    }

    /// Рекурсивно добавляет файлы из директории
    pub fn add(watchlist: &Watchlist, target: impl AsRef<std::path::Path>) -> Result<usize> {
        let target = resolve_target(&watchlist.root, target.as_ref())?;
        if !target.is_dir() {
            bail!("Target is not a directory: {}", target.display());
        }

        let mut entries = Self::load(watchlist)?;
        let initial_len = entries.len();

        let walker = WalkDir::new(&target)
            .follow_links(false)
            .into_iter()
            .filter_entry(|e| e.file_name() != LIT_DIR);

        for item in walker {
            let item = item.context("Failed to walk directory")?;
            if !item.file_type().is_file() {
                continue;
            }

            let relative_str = to_portable_string(item.path(), &watchlist.root)?;
            if relative_str == format!("{LIT_DIR}/{WATCHLIST_FILE}") {
                continue;
            }

            entries.push(PathBuf::from(relative_str));
        }

        Self::save(watchlist, entries)?;
        let final_entries = Self::load(watchlist)?;
        Ok(final_entries.len() - initial_len)
    }

    /// Удаляет файлы внутри указанной директории из ватчлиста
    pub fn remove(watchlist: &Watchlist, target: impl AsRef<std::path::Path>) -> Result<usize> {
        let target = resolve_target(&watchlist.root, target.as_ref())?;
        let prefix = to_portable_string(&target, &watchlist.root)?;

        if prefix.is_empty() {
            bail!("Refusing to remove the entire project watchlist");
        }

        let mut entries = Self::load(watchlist)?;
        let initial_len = entries.len();

        entries.retain(|entry| {
            let s = entry.to_string_lossy();
            s != prefix && !s.starts_with(&format!("{prefix}/"))
        });

        let removed = initial_len - entries.len();
        Self::save(watchlist, entries)?;
        Ok(removed)
    }
}
