use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    #[serde(default = "default_interval")]
    pub interval: u64,

    #[serde(default)]
    pub on_new_pr: Vec<ActionCommand>,

    #[serde(default)]
    pub on_select: Option<SelectCommand>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ActionCommand {
    pub name: String,
    pub command: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SelectCommand {
    pub command: String,
}

fn default_interval() -> u64 {
    120
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interval: default_interval(),
            on_new_pr: Vec::new(),
            on_select: None,
        }
    }
}

pub fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("gh-review-watcher")
        .join("config.toml")
}

pub fn load_config() -> Config {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        toml::from_str(&content).unwrap_or_else(|e| {
            eprintln!("Warning: failed to parse config: {e}");
            Config::default()
        })
    } else {
        Config::default()
    }
}
