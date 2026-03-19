use std::fs;
use std::path::PathBuf;

use crate::types::{Settings, OverlayPosition};
use crate::error::{AppError, Result};

const CONFIG_FILE_NAME: &str = "settings.json";

#[derive(Debug)]
pub struct SettingsStore;

impl SettingsStore {
    pub fn new() -> Self {
        SettingsStore
    }

    pub fn load() -> Result<Settings> {
        let config_dir = Self::get_config_dir()?;
        let config_path = config_dir.join(CONFIG_FILE_NAME);
        
        if config_path.exists() {
            let content = fs::read_to_string(config_path)
                .map_err(|e| AppError::IoError(e.to_string()))?;
            let settings = serde_json::from_str(&content)
                .map_err(|e| AppError::ConfigError(format!("Failed to parse settings: {}", e)))?;
            Ok(settings)
        } else {
            // Return default settings if config file doesn't exist
            Ok(Settings::default())
        }
    }

    pub fn save(settings: &Settings) -> Result<()> {
        let config_dir = Self::get_config_dir()?;
        let config_path = config_dir.join(CONFIG_FILE_NAME);
        
        let content = serde_json::to_string_pretty(settings)
            .map_err(|e| AppError::ConfigError(format!("Failed to serialize settings: {}", e)))?;
        fs::write(config_path, content)
            .map_err(|e| AppError::IoError(e.to_string()))?;
        Ok(())
    }

    pub fn reset() -> Result<Settings> {
        let default_settings = Settings::default();
        Self::save(&default_settings)?;
        Ok(default_settings)
    }

    fn get_config_dir() -> Result<PathBuf> {
        let mut config_dir = dirs::config_dir()
            .ok_or_else(|| AppError::ConfigError("Configuration directory not found".to_string()))?;
        
        config_dir.push("murmure2");
        
        if !config_dir.exists() {
            fs::create_dir_all(&config_dir)
                .map_err(|e| AppError::IoError(e.to_string()))?;
        }
        
        Ok(config_dir)
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            shortcut: "Ctrl+Space".to_string(),
            language: "auto".to_string(),
            overlay_position: OverlayPosition { x: 0, y: 0 }, // Centered position
            setup_completed: false,
        }
    }
}