use std::{io, path::PathBuf};

use fs_err as fs;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::i18n::Language;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub theme: egui::Theme,
    pub language: Language,
    pub open_files: Vec<PathBuf>,
    pub current_project: Option<PathBuf>,
}

impl Config {
    pub fn load() -> Result<Self, ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or(ConfigError::ConfigFolderNotSupported)?;

        let config_path = config_dir.join("fexc");

        fs::create_dir_all(&config_path)
            .map_err(ConfigError::CreateDir)?;

        let config_file = config_path.join("config.toml");
        if config_file.exists() {
            let contents = fs::read_to_string(&config_file)
                .map_err(LoadConfigError::Io)
                .map_err(ConfigError::LoadConfig)?;

            let config: Config = toml::from_str(&contents)
                .map_err(LoadConfigError::Toml)
                .map_err(ConfigError::LoadConfig)?;

            Ok(config)
        } else {
            fs::File::create(&config_file)
                .map_err(ConfigError::CreateFile)?;

            let default_config = Config::default();
            let toml = toml::to_string(&default_config)
                .map_err(SaveConfigError::Toml)
                .map_err(ConfigError::SaveConfig)?;

            fs::write(&config_file, toml)
                .map_err(SaveConfigError::Io)
                .map_err(ConfigError::SaveConfig)?;

            Ok(default_config)
        }
    }

    pub fn save(&self) -> Result<(), ConfigError> {
        let config_dir = dirs::config_dir()
            .ok_or(ConfigError::ConfigFolderNotSupported)?;

        let config_path = config_dir.join("fexc");

        fs::create_dir_all(&config_path)
            .map_err(ConfigError::CreateDir)?;

        let config_file = config_path.join("config.toml");
        let toml = toml::to_string(self)
            .map_err(SaveConfigError::Toml)
            .map_err(ConfigError::SaveConfig)?;

        fs::write(&config_file, toml)
            .map_err(SaveConfigError::Io)
            .map_err(ConfigError::SaveConfig)?;

        Ok(())
    }
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Config folder is not found/supported on this platform")]
    ConfigFolderNotSupported,
    #[error("Failed to create config directory: {0}")]
    CreateDir(io::Error),
    #[error("Failed to create config file: {0}")]
    CreateFile(io::Error),
    #[error("Failed to load config file: {0}")]
    LoadConfig(LoadConfigError),
    #[error("Failed to save config file: {0}")]
    SaveConfig(SaveConfigError),
}

#[derive(Debug, Error)]
pub enum SaveConfigError {
    #[error("Failed to save config file: {0}")]
    Io(io::Error),
    #[error("Failed to serialize config file: {0}")]
    Toml(toml::ser::Error),
}

#[derive(Debug, Error)]
pub enum LoadConfigError {
    #[error("Failed to load config file: {0}")]
    Io(io::Error),
    #[error("Failed to deserialize config file: {0}")]
    Toml(toml::de::Error),
}

impl Default for Config {
    fn default() -> Self {
        Config {
            language: Language::English,
            theme: egui::Theme::Dark,
            open_files: Vec::new(),
            current_project: None,
        }
    }
}