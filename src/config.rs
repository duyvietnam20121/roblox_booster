use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Application configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Auto-enable booster when app starts
    pub auto_start_booster: bool,
    /// Auto-detect and boost Roblox when it launches
    pub auto_detect_roblox: bool,
    /// Priority level for Roblox processes (0=Normal, 1=Above Normal, 2=High)
    pub priority_level: u8,
    /// Clear memory cache when boosting
    pub clear_memory_cache: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_start_booster: false,
            auto_detect_roblox: true,
            priority_level: 2, // High priority
            clear_memory_cache: true,
        }
    }
}

impl Config {
    /// Get config file path using standard directories
    fn get_config_path() -> PathBuf {
        let mut path = if cfg!(target_os = "windows") {
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
        } else {
            std::env::var("HOME")
                .map(|h| {
                    let mut p = PathBuf::from(h);
                    p.push(".config");
                    p
                })
                .unwrap_or_else(|_| PathBuf::from("."))
        };
        
        path.push("roblox_booster");
        path.push("config.json");
        path
    }
    
    /// Load configuration from file
    #[must_use]
    pub fn load() -> Self {
        let path = Self::get_config_path();
        
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        
        fs::read_to_string(&path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_else(|| {
                let config = Self::default();
                let _ = config.save();
                config
            })
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path();
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }
        
        let json = serde_json::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&path, json)
            .context("Failed to write config file")?;
        
        Ok(())
    }
}