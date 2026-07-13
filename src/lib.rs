pub mod config;
pub mod kind;
pub mod model;
pub mod sources;

use sources::Source;
use std::path::PathBuf;

/// Lists all directories and files from the given source.
pub fn list_items(source: Source) -> Vec<PathBuf> {
    todo!()
}

/// Finds the path to the given file, optionally scoped to a source type.
pub fn find(source: Option<Source>, file_name: &str) -> Option<PathBuf> {
    todo!()
}

/// Finds the path to the given file, or creates it if it doesn't exist.
pub fn find_or_create(source: Option<Source>, file_name: &str) -> PathBuf {
    todo!()
}
