use serde::{Deserialize, Serialize};
use std::fs;
use std::io;

const CONFIG_PATH: &str = "config.json";

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Config {
    pub auto_start: bool,
    pub boost_interval_seconds: u64,
    pub enable_timer_resolution: bool,
    pub enable_memory_cleanup: bool,
    pub enable_auto_detection: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            auto_start: false,
            boost_interval_seconds: 60,
            enable_timer_resolution: true,
            enable_memory_cleanup: true,
            enable_auto_detection: true,
        }
    }
}

impl Config {
    pub fn load() -> Self {
        let contents = match fs::read_to_string(CONFIG_PATH) {
            Ok(contents) => contents,
            Err(_) => {
                let config = Self::default();
                let _ = config.save();
                return config;
            }
        };

        match serde_json::from_str(&contents) {
            Ok(config) => config,
            Err(_) => {
                let config = Self::default();
                let _ = config.save();
                config
            }
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let data = serde_json::to_string_pretty(self)
            .map_err(|err| io::Error::new(io::ErrorKind::InvalidData, err))?;
        fs::write(CONFIG_PATH, data)
    }
}
