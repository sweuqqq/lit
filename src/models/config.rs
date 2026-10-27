use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, clap::Args, Debug, Clone)]
pub struct Model {
    #[arg(long)]
    pub key: Option<String>,

    #[arg(long)]
    pub username: Option<String>,
}

impl Default for Model {
    fn default() -> Self {
        Self {
            key: Some("".to_string()),
            username: Some("User".to_string()),
        }
    }
}
