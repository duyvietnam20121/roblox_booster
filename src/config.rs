use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

/// Application configuration with safe defaults
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Auto-enable booster when app starts
    pub auto_start_booster: bool,
    
    /// Auto-detect and boost Roblox when it launches
    pub auto_detect_roblox: bool,
    
    /// Priority level (0=Normal, 1=Above Normal, 2=High)
    pub priority_level: u8,
    
    /// Memory optimization (placeholder, no actual memory modification)
    pub clear_memory_cache: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_start_booster: false,
            auto_detect_roblox: true,
            priority_level: 1, // Above Normal (conservative default)
            clear_memory_cache: true,
        }
    }
}

impl Config {
    /// Get platform-specific config file path
    fn get_config_path() -> PathBuf {
        #[cfg(target_os = "windows")]
        {
            std::env::var("APPDATA")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("roblox_booster")
                .join("config.json")
        }

        #[cfg(not(target_os = "windows"))]
        {
            std::env::var("HOME")
                .map(|h| PathBuf::from(h).join(".config"))
                .unwrap_or_else(|_| PathBuf::from("."))
                .join("roblox_booster")
                .join("config.json")
        }
    }

    /// Load configuration from file or create with defaults
    #[must_use]
    pub fn load() -> Self {
        let path = Self::get_config_path();

        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            let _ = fs::create_dir_all(parent);
        }

        // Try to load existing config
        fs::read_to_string(&path)
            .ok()
            .and_then(|contents| serde_json::from_str(&contents).ok())
            .unwrap_or_else(|| {
                // Create and save default config
                let config = Self::default();
                let _ = config.save();
                config
            })
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let path = Self::get_config_path();

        // Create parent directory if needed
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).context("Failed to create config directory")?;
        }

        // Serialize with pretty printing
        let json = serde_json::to_string_pretty(self).context("Failed to serialize config")?;

        // Write to file
        fs::write(&path, json).context("Failed to write config file")?;

        Ok(())
    }

    /// Validate configuration values
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.priority_level <= 2
    }

    /// Get priority level as human-readable string
    #[must_use]
    pub const fn priority_name(&self) -> &'static str {
        match self.priority_level {
            0 => "Normal",
            1 => "Above Normal",
            _ => "High",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(!config.auto_start_booster);
        assert!(config.auto_detect_roblox);
        assert_eq!(config.priority_level, 1);
        assert!(config.is_valid());
    }

    #[test]
    fn test_priority_names() {
        let mut config = Config::default();
        
        config.priority_level = 0;
        assert_eq!(config.priority_name(), "Normal");
        
        config.priority_level = 1;
        assert_eq!(config.priority_name(), "Above Normal");
        
        config.priority_level = 2;
        assert_eq!(config.priority_name(), "High");
    }

    #[test]
    fn test_validation() {
        let mut config = Config::default();
        
        config.priority_level = 0;
        assert!(config.is_valid());
        
        config.priority_level = 2;
        assert!(config.is_valid());
        
        config.priority_level = 3;
        assert!(!config.is_valid());
    }
}