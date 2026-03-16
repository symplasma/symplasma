pub mod config;
pub mod sources;

use sources::Source;
use std::path::PathBuf;

/// Returns a list of all possible data sources.
pub fn list_sources() -> Vec<Source> {
    todo!()
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_sources() {
        todo!()
    }

    #[test]
    fn test_list_items() {
        todo!()
    }

    #[test]
    fn test_find() {
        todo!()
    }

    #[test]
    fn test_find_or_create() {
        todo!()
    }
}
