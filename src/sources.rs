use std::path::PathBuf;

use strum::EnumIter;

/// Represents the different types of data sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter)]
pub enum Source {
    /// Collections of files representing circles.
    Circles,
    /// Collections of files representing projects.
    Projects,
    /// Collections of files representing repositories.
    Repos,
    /// Individual markdown files (mostly notes).
    Markdown,
    /// Individual picture files.
    Pictures,
    /// Individual video files.
    Videos,
    /// Individual music files.
    Music,
    /// Individual audio files.
    Audio,
}

impl Source {
    /// Returns all possible source types.
    pub fn all() -> Vec<Source> {
        todo!()
    }

    /// Returns whether this source type represents a collection of files
    /// (like Circles, Projects, Repos) or individual files.
    pub fn is_collection(&self) -> bool {
        todo!()
    }

    /// Returns the name of the source as a string.
    pub fn name(&self) -> &'static str {
        todo!()
    }

    /// Parses a source from a string name.
    pub fn from_name(name: &str) -> Option<Source> {
        todo!()
    }

    /// Returns the configured paths for this source type.
    pub fn paths(&self, config: &crate::config::Config) -> &[PathBuf] {
        todo!()
    }
}

impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // AI! use the appropriate `strum` macro to print the name rather than using a match statement
        match self {
            Source::Circles => write!(f, "circles"),
            Source::Projects => write!(f, "projects"),
            Source::Repos => write!(f, "repos"),
            Source::Markdown => write!(f, "markdown"),
            Source::Pictures => write!(f, "pictures"),
            Source::Videos => write!(f, "videos"),
            Source::Music => write!(f, "music"),
            Source::Audio => write!(f, "audio"),
        }
    }
}

impl std::str::FromStr for Source {
    type Err = SourceParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

/// Error returned when parsing a source from a string fails.
#[derive(Debug, Clone)]
pub struct SourceParseError {
    pub invalid_name: String,
}

impl std::fmt::Display for SourceParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl std::error::Error for SourceParseError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_all() {
        todo!()
    }

    #[test]
    fn test_source_is_collection() {
        todo!()
    }

    #[test]
    fn test_source_name() {
        todo!()
    }

    #[test]
    fn test_source_from_name() {
        todo!()
    }

    #[test]
    fn test_source_from_str() {
        todo!()
    }
}
