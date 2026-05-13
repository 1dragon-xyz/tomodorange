use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub work_minutes: u32,
    pub break_minutes: u32,
    pub always_on_top: bool,
    pub sound_enabled: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            work_minutes: 25,
            break_minutes: 5,
            always_on_top: true,
            sound_enabled: true,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let config_path = Self::get_path();
        
        if let Ok(content) = fs::read_to_string(&config_path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }
        
        // If file doesn't exist or is invalid, save default
        let default_config = Self::default();
        let _ = default_config.save();
        default_config
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_path();
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }
        let content = toml::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }

    pub fn get_path() -> PathBuf {
        if let Some(proj_dirs) = ProjectDirs::from("com", "michaels", "tomodorange") {
            proj_dirs.config_dir().join("config.toml")
        } else {
            PathBuf::from("config.toml")
        }
    }
}
