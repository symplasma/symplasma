//! Traits that allow standard interaction with all kinds

use mime_type::MimeType;
use std::path::PathBuf;

use crate::config::Config;

pub trait Kind {
    /// Lists all files of interest for this kind.
    ///
    /// The list of files can be constrained by mime type.
    fn files(config: &Config) -> Vec<PathBuf>;

    // Lists all items of a given kind.
    //
    // Items are differentiated from files since they items are objects that may or may not correspond to a file path.
    // fn items(mime_type: &MimeType) -> Item;

    /// A list of files that should be passed to search indexers.
    ///
    /// may need to return file type as well as some kinds may result in multiple types of files
    fn indexable(config: &Config, mime_type: &MimeType) -> Vec<PathBuf>;

    // collections: directories into which new items can be collected

    // storage: directories into which new items can be collected
}
