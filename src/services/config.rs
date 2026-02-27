use serde::{Deserialize, Serialize};
use std::{env, fs, path::PathBuf};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub dry_run: bool,

    #[serde(default)]
    pub root: Vec<Root>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Root {
    pub path: String,

    #[serde(default)]
    pub folders: HashMap<String, FolderRule>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct FolderRule {
    #[serde(default)]
    pub mime: Vec<String>,

    #[serde(default)]
    pub patterns: Vec<String>,

    #[serde(default)]
    pub use_regex: bool,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let path = Self::path();

        if !path.exists() {
            let default_config = Config::default();
            let toml = toml::to_string_pretty(&default_config)?;

            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
}

            fs::write(&path, toml)?;
            return Ok(default_config);
        }

        let content = fs::read_to_string(&path)?;
        let config = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn path() -> PathBuf {
        const CONFIG_FILE_NAME: &str = "bomdia.toml";

        if let Some(xdg) = env::var_os("XDG_CONFIG_HOME") {
            return PathBuf::from(xdg).join(CONFIG_FILE_NAME);
        }

        dirs_next::home_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join(".config")
            .join(CONFIG_FILE_NAME)
    }
}
