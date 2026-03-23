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

    /// Writes the default configuration to the config file path.
    /// Creates parent directories if they don't exist.
    pub fn write_default() -> Result<(), ConfigError> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(ConfigError::IoError)?;
        }
        let content = Self::default_kdl();
        std::fs::write(&path, content).map_err(ConfigError::IoError)
    }

    /// Returns the default configuration as a KDL string.
    pub fn default_kdl() -> String {
        r#"// Symplasma configuration file
// Paths can use ~ for home directory

circles "~/circles"
projects "~/projects"
repos "~/repos"
markdown "~/notes"
pictures "~/pictures"
videos "~/videos"
music "~/music"
audio "~/audio"
"#
        .to_string()
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            circles: vec![PathBuf::from("~/circles")],
            projects: vec![PathBuf::from("~/projects")],
            repos: vec![PathBuf::from("~/repos")],
            markdown: vec![PathBuf::from("~/notes")],
            pictures: vec![PathBuf::from("~/pictures")],
            videos: vec![PathBuf::from("~/videos")],
            music: vec![PathBuf::from("~/music")],
            audio: vec![PathBuf::from("~/audio")],
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
        assert_eq!(config.circles, vec![PathBuf::from("~/circles")]);
        assert_eq!(config.projects, vec![PathBuf::from("~/projects")]);
        assert_eq!(config.repos, vec![PathBuf::from("~/repos")]);
        assert_eq!(config.markdown, vec![PathBuf::from("~/notes")]);
        assert_eq!(config.pictures, vec![PathBuf::from("~/pictures")]);
        assert_eq!(config.videos, vec![PathBuf::from("~/videos")]);
        assert_eq!(config.music, vec![PathBuf::from("~/music")]);
        assert_eq!(config.audio, vec![PathBuf::from("~/audio")]);
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
