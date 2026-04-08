use std::path::{Path, PathBuf};

use crate::config::Config;

pub struct Markdown {
    path: PathBuf,
}

impl Markdown {
    pub fn list_files(config: &Config) {}

    pub fn new(path: &Path) -> Self {
        Markdown {
            path: path.to_path_buf(),
        }
    }
}
