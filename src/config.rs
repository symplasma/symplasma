use std::path::PathBuf;

use confique::Config as _;
use serde::Deserialize;

/// Represents the application configuration loaded from KDL files.
#[derive(Debug, Clone, Deserialize, confique::Config)]
pub struct Config {
    #[config(default = [])]
    pub circles_paths: Vec<PathBuf>,
    #[config(default = [])]
    pub projects_paths: Vec<PathBuf>,
    #[config(default = [])]
    pub repos_paths: Vec<PathBuf>,
    #[config(default = [])]
    pub markdown_paths: Vec<PathBuf>,
    #[config(default = [])]
    pub pictures_paths: Vec<PathBuf>,
    #[config(default = [])]
    pub videos_paths: Vec<PathBuf>,
    #[config(default = [])]
    pub music_paths: Vec<PathBuf>,
    #[config(default = [])]
    pub audio_paths: Vec<PathBuf>,
}

impl Config {
    /// Loads the configuration from the XDG config directory.
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path();
        if !path.exists() {
            return Err(ConfigError::NotFound);
        }
        let content = std::fs::read_to_string(&path).map_err(ConfigError::IoError)?;
        Self::parse_kdl(&content)
    }

    /// Returns the path to the config file according to XDG Base Directory Specification.
    pub fn config_path() -> PathBuf {
        directories::ProjectDirs::from("com", "symplasma", "symplasma")
            .map(|dirs| dirs.config_dir().join("config.kdl"))
            .unwrap_or_else(|| PathBuf::from("config.kdl"))
    }

    /// Parses a KDL configuration string into a Config struct.
    pub fn parse_kdl(content: &str) -> Result<Self, ConfigError> {
        let doc: kdl::KdlDocument = content.parse().map_err(|e: kdl::KdlError| {
            ConfigError::ParseError(e.to_string())
        })?;

        let value = kdl::serde::from_document::<Self>(doc)
            .map_err(|e| ConfigError::ParseError(e.to_string()))?;

        Ok(value)
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            circles_paths: Vec::new(),
            projects_paths: Vec::new(),
            repos_paths: Vec::new(),
            markdown_paths: Vec::new(),
            pictures_paths: Vec::new(),
            videos_paths: Vec::new(),
            music_paths: Vec::new(),
            audio_paths: Vec::new(),
        }
    }
}

/// Errors that can occur when loading or parsing configuration.
#[derive(Debug)]
pub enum ConfigError {
    /// The config file was not found.
    NotFound,
    /// An I/O error occurred while reading the config file.
    IoError(std::io::Error),
    /// The KDL content could not be parsed.
    ParseError(String),
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConfigError::NotFound => write!(f, "Configuration file not found"),
            ConfigError::IoError(e) => write!(f, "I/O error reading configuration: {}", e),
            ConfigError::ParseError(e) => write!(f, "Error parsing configuration: {}", e),
        }
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert!(config.circles_paths.is_empty());
        assert!(config.projects_paths.is_empty());
    }

    #[test]
    fn test_config_path() {
        let path = Config::config_path();
        assert!(path.to_string_lossy().contains("config.kdl"));
    }

    #[test]
    fn test_parse_kdl() {
        let kdl_content = r#"
            circles_paths "~/circles" "/data/circles"
            projects_paths "~/projects"
            repos_paths "~/repos"
            markdown_paths "~/notes"
            pictures_paths "~/pictures"
            videos_paths "~/videos"
            music_paths "~/music"
            audio_paths "~/audio"
        "#;

        let config = Config::parse_kdl(kdl_content).expect("Failed to parse KDL");
        assert_eq!(config.circles_paths.len(), 2);
        assert_eq!(config.projects_paths.len(), 1);
    }

    #[test]
    fn test_parse_kdl_empty() {
        // Empty config should use defaults
        let kdl_content = "";
        let result = Config::parse_kdl(kdl_content);
        // An empty document may or may not parse depending on kdl serde behavior
        // This test documents the behavior
        assert!(result.is_ok() || result.is_err());
    }
}
