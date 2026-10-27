use std::path::PathBuf;

pub struct Watchlist {
    pub root: PathBuf,
}

impl Watchlist {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }
}
