use std::path::PathBuf;

/// Represents the application configuration loaded from KDL files.
#[derive(Debug, Clone)]
pub struct Config {
    pub circles_paths: Vec<PathBuf>,
    pub projects_paths: Vec<PathBuf>,
    pub repos_paths: Vec<PathBuf>,
    pub markdown_paths: Vec<PathBuf>,
    pub pictures_paths: Vec<PathBuf>,
    pub videos_paths: Vec<PathBuf>,
    pub music_paths: Vec<PathBuf>,
    pub audio_paths: Vec<PathBuf>,
}

impl Config {
    /// Loads the configuration from the XDG config directory.
    pub fn load() -> Result<Self, ConfigError> {
        todo!()
    }

    /// Returns the path to the config file according to XDG Base Directory Specification.
    pub fn config_path() -> PathBuf {
        todo!()
    }

    /// Parses a KDL configuration string into a Config struct.
    pub fn parse_kdl(content: &str) -> Result<Self, ConfigError> {
        todo!()
    }
}

impl Default for Config {
    fn default() -> Self {
        todo!()
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
        todo!()
    }
}

impl std::error::Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_load() {
        todo!()
    }

    #[test]
    fn test_config_path() {
        todo!()
    }

    #[test]
    fn test_parse_kdl() {
        todo!()
    }
}
