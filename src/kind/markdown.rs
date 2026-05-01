use crate::{config::Config, kind::traits::Kind};
use std::path::{Path, PathBuf};

pub struct Markdown {
    path: PathBuf,
}

impl Markdown {
    pub fn new(path: &Path) -> Self {
        Markdown {
            path: path.to_path_buf(),
        }
    }
}

impl Kind for Markdown {
    fn files(config: &Config) -> Vec<PathBuf> {
        let mut files = Vec::default();

        for path in &config.markdown {
            files.push(path.to_path_buf());
        }

        files
    }

    fn indexable(config: &Config, mime_type: &mime_type::MimeType) -> Vec<PathBuf> {
        todo!()
    }
}
