use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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
    /// Get config file path
    fn get_config_path() -> PathBuf {
        let mut path = dirs::config_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("roblox_booster");
        fs::create_dir_all(&path).ok();
        path.push("config.json");
        path
    }
    
    /// Load configuration from file
    pub fn load() -> Self {
        let path = Self::get_config_path();
        
        if let Ok(contents) = fs::read_to_string(&path) {
            if let Ok(config) = serde_json::from_str(&contents) {
                return config;
            }
        }
        
        // Return default if file doesn't exist or is invalid
        let config = Self::default();
        let _ = config.save(); // Try to save default config
        config
    }
    
    /// Save configuration to file
    pub fn save(&self) -> Result<(), String> {
        let path = Self::get_config_path();
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config: {}", e))?;
        
        fs::write(&path, json)
            .map_err(|e| format!("Failed to write config: {}", e))?;
        
        Ok(())
    }
}

// Simple cross-platform dirs implementation
mod dirs {
    use std::path::PathBuf;
    
    pub fn config_dir() -> Option<PathBuf> {
        #[cfg(target_os = "windows")]
        {
            std::env::var("APPDATA").ok().map(PathBuf::from)
        }
        
        #[cfg(not(target_os = "windows"))]
        {
            std::env::var("HOME").ok().map(|h| {
                let mut path = PathBuf::from(h);
                path.push(".config");
                path
            })
        }
    }
}