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

fn is_markdown_extension(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(ext.to_lowercase().as_str(), "md" | "markdown"),
        None => false,
    }
}

fn is_utf8_file(path: &Path) -> bool {
    std::fs::read(path)
        .map(|bytes| std::str::from_utf8(&bytes).is_ok())
        .unwrap_or(false)
}

impl Kind for Markdown {
    fn files(config: &Config) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for base_path in &config.markdown {
            let walker = walkdir::WalkDir::new(base_path)
                .follow_links(true)
                .into_iter()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().is_file());

            for entry in walker {
                let path = entry.path();
                if is_markdown_extension(path) && is_utf8_file(path) {
                    files.push(path.to_path_buf());
                }
            }
        }

        files
    }

    fn indexable(config: &Config, mime_type: &mime_type::MimeType) -> Vec<PathBuf> {
        todo!()
    }
}
