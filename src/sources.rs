use std::path::PathBuf;

use strum::{Display, EnumIter, EnumString};

use crate::config::Config;
use crate::kind::markdown::Markdown;
use crate::kind::traits::Kind as _;
use crate::kind::web_archives::WebArchive;

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

    /// Lists all files for this source.
    ///
    /// Sources that don't yet have an implementation return an empty list.
    pub fn files(&self, config: &Config) -> Vec<PathBuf> {
        match self {
            Source::Markdown => Markdown::files(config),
            Source::WebArchives => WebArchive::files(config),
            _ => Vec::new(),
        }
    }

    /// Lists all directories for this source.
    ///
    /// Sources that don't yet have an implementation return an empty list.
    pub fn dirs(&self, config: &Config) -> Vec<PathBuf> {
        match self {
            Source::Circles => todo!(),
            Source::Projects => todo!(),
            Source::Repos => todo!(),
            Source::Markdown => Markdown::dirs(config),
            Source::WebArchives => WebArchive::dirs(config),
            Source::Pictures => todo!(),
            Source::Videos => todo!(),
            Source::Music => todo!(),
            Source::Audio => todo!(),
        }
    }

    /// Prints all items for this source.
    ///
    /// Sources that don't yet have an implementation do nothing.
    pub fn print_items(&self, config: &Config) {
        match self {
            Source::WebArchives => {
                for item in WebArchive::items(config) {
                    println!("{item}");
                }
            }
            _ => {}
        }
    }
}
