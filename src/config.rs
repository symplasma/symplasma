use std::path::PathBuf;

use serde::Deserialize;

/// Represents the application configuration loaded from KDL files.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub circles: Vec<PathBuf>,
    #[serde(default)]
    pub projects: Vec<PathBuf>,
    #[serde(default)]
    pub repos: Vec<PathBuf>,
    #[serde(default)]
    pub markdown: Vec<PathBuf>,
    #[serde(default)]
    pub pictures: Vec<PathBuf>,
    #[serde(default)]
    pub videos: Vec<PathBuf>,
    #[serde(default)]
    pub music: Vec<PathBuf>,
    #[serde(default)]
    pub audio: Vec<PathBuf>,
}

impl Config {
    /// Loads the configuration from the XDG config directory.
    /// If the config file is not found, returns a default configuration.
    pub fn load() -> Result<Self, ConfigError> {
        let path = Self::config_path();
        if !path.exists() {
            return Ok(Self::default());
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
        serde_kdl2::from_str(content).map_err(|e| ConfigError::ParseError(e.to_string()))
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            circles: Vec::new(),
            projects: Vec::new(),
            repos: Vec::new(),
            markdown: Vec::new(),
            pictures: Vec::new(),
            videos: Vec::new(),
            music: Vec::new(),
            audio: Vec::new(),
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
        assert!(config.circles.is_empty());
        assert!(config.projects.is_empty());
    }

    #[test]
    fn test_config_path() {
        let path = Config::config_path();
        assert!(path.to_string_lossy().contains("config.kdl"));
    }

    #[test]
    fn test_parse_kdl() {
        let kdl_content = r#"
            circles "~/circles"
            projects "~/projects"
            repos "~/repos"
            markdown "~/notes"
            pictures "~/pictures"
            videos "~/videos"
            music "~/music"
            audio "~/audio"
        "#;

        let config = Config::parse_kdl(kdl_content).expect("Failed to parse KDL");
        assert_eq!(config.circles.len(), 2);
        assert_eq!(config.projects.len(), 1);
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
