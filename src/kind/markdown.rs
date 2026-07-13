use crate::{config::Config, kind::traits::Kind};
use std::path::{Path, PathBuf};
use tracing::{debug, trace};

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

fn expand_tilde(path: &Path) -> PathBuf {
    if let Some(path_str) = path.to_str() {
        if let Some(stripped) = path_str.strip_prefix("~/") {
            if let Some(home) = directories::UserDirs::new().map(|d| d.home_dir().to_path_buf()) {
                let expanded = home.join(stripped);
                trace!(original = %path.display(), expanded = %expanded.display(), "Expanded tilde path");
                return expanded;
            }
        } else if path_str == "~" {
            if let Some(home) = directories::UserDirs::new().map(|d| d.home_dir().to_path_buf()) {
                trace!(original = %path.display(), expanded = %home.display(), "Expanded tilde path");
                return home;
            }
        }
    }
    path.to_path_buf()
}

fn is_markdown_extension(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(ext.to_lowercase().as_str(), "md" | "markdown"),
        None => false,
    }
}

fn is_utf8_file(path: &Path) -> bool {
    let result = std::fs::read(path)
        .map(|bytes| std::str::from_utf8(&bytes).is_ok())
        .unwrap_or(false);
    if !result {
        trace!(path = %path.display(), "File is not valid UTF-8 or could not be read");
    }
    result
}

impl Kind for Markdown {
    fn files(config: &Config) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for base_path in &config.markdown {
            let expanded = expand_tilde(base_path);
            debug!(base_path = %base_path.display(), expanded = %expanded.display(), "Scanning for markdown files");

            let walker = ignore::WalkBuilder::new(&expanded)
                .follow_links(true)
                .build()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false));

            for entry in walker {
                let path = entry.path();
                if is_markdown_extension(path) && is_utf8_file(path) {
                    trace!(path = %path.display(), "Found markdown file");
                    files.push(path.to_path_buf());
                }
            }
        }

        debug!(count = files.len(), "Finished scanning for markdown files");
        files
    }

    fn indexable(config: &Config, mime_type: &mime_type::MimeType) -> Vec<PathBuf> {
        todo!()
    }
}
