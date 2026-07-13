use std::path::PathBuf;

use strum::{Display, EnumIter, EnumString};

/// Represents the different types of data sources.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[strum(serialize_all = "lowercase")]
pub enum Source {
    /// Collections of files representing circles.
    Circles,
    /// Collections of files representing projects.
    Projects,
    /// Collections of files representing repositories.
    Repos,
    /// Individual markdown files (mostly notes).
    Markdown,
    /// Individual web page archive directories from WebScrapBook. (We might support single file formats later.)
    WebArchives,
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
