use dirs_next;
use serde::Deserialize;
use serde::Serialize;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

fn get_config_path() -> PathBuf {
    const CONFIG_FILE_NAME: &str = "bomdia.toml";
    if let Some(xdg) = env::var_os("XDG_CONFIG_HOME") {
        return PathBuf::from(xdg).join(CONFIG_FILE_NAME);
    }
    dirs_next::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(format!(".config/{}", CONFIG_FILE_NAME))
}

pub fn get_config() -> Config {
    let path = get_config_path();

    if !path.exists() {
        let default_config = Config::default();
        let toml_str = toml::to_string_pretty(&default_config).unwrap_or_else(|err| {
            eprintln!("Error serializing default config: {}", err);
            process::exit(1);
        });
        fs::write(&path, toml_str).unwrap_or_else(|err| {
            eprintln!("Failed to create file {}: {}", path.display(), err);
            process::exit(1);
        });

        println!("Config created at: {}", path.display());
        return default_config;
    }

    let conteudo = fs::read_to_string(&path).unwrap_or_else(|err| {
        eprintln!("Failed to read file {:#?}: {}", path, err);
        process::exit(1);
    });

    let config: Config = toml::from_str(&conteudo).unwrap_or_else(|err| {
        eprintln!("Error processing file format: {}", err);
        process::exit(1);
    });
    config
}
