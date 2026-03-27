use serde::Deserialize;
use std::fs;
use std::io::Write;
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
    // Check XDG-style ~/.config first (common on Linux and user preference on macOS),
    // then fall back to platform-native config dir (~/Library/Application Support on macOS)
    if let Ok(home) = std::env::var("HOME") {
        let xdg_path = PathBuf::from(&home)
            .join(".config")
            .join("gh-review-watcher")
            .join("config.toml");
        if xdg_path.exists() {
            return xdg_path;
        }
    }

    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("~/.config"))
        .join("gh-review-watcher")
        .join("config.toml")
}

fn log(msg: &str) {
    if let Ok(mut f) = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("/tmp/gh-review-watcher.log")
    {
        let now = chrono::Local::now().format("%H:%M:%S");
        let _ = writeln!(f, "[{now}] {msg}");
    }
}

pub fn load_config() -> Config {
    let path = config_path();
    if path.exists() {
        let content = fs::read_to_string(&path).unwrap_or_default();
        match toml::from_str::<Config>(&content) {
            Ok(config) => {
                log(&format!(
                    "Config loaded: interval={}, on_new_pr={} hooks, on_select={}",
                    config.interval,
                    config.on_new_pr.len(),
                    config.on_select.is_some()
                ));
                config
            }
            Err(e) => {
                log(&format!("Config parse error: {e}"));
                eprintln!("Warning: failed to parse config: {e}");
                Config::default()
            }
        }
    } else {
        log(&format!("Config not found at {}", path.display()));
        Config::default()
    }
}
