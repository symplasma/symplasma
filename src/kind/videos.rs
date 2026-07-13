use crate::{config::Config, kind::traits::Kind, kind::util::expand_tilde};
use std::path::{Path, PathBuf};
use tracing::{debug, trace};

pub struct Videos {
    path: PathBuf,
}

impl Videos {
    pub fn new(path: &Path) -> Self {
        Videos {
            path: path.to_path_buf(),
        }
    }

    pub fn dirs(config: &Config) -> Vec<PathBuf> {
        let mut dirs = Vec::new();

        for base_path in &config.videos {
            let expanded = expand_tilde(base_path);
            debug!(base_path = %base_path.display(), expanded = %expanded.display(), "Scanning for video directories");

            let walker = ignore::WalkBuilder::new(&expanded)
                .follow_links(true)
                .build()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false));

            for entry in walker {
                let path = entry.path();
                trace!(path = %path.display(), "Found video directory");
                dirs.push(path.to_path_buf());
            }
        }

        debug!(count = dirs.len(), "Finished scanning for video directories");
        dirs
    }
}

fn is_video_extension(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some(ext) => matches!(
            ext.to_lowercase().as_str(),
            "mp4" | "mkv" | "avi" | "mov" | "wmv" | "flv" | "webm" | "m4v" | "mpg" | "mpeg" | "3gp"
                | "ogv"
        ),
        None => false,
    }
}

impl Kind for Videos {
    fn files(config: &Config) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for base_path in &config.videos {
            let expanded = expand_tilde(base_path);
            debug!(base_path = %base_path.display(), expanded = %expanded.display(), "Scanning for video files");

            let walker = ignore::WalkBuilder::new(&expanded)
                .follow_links(true)
                .build()
                .filter_map(|e| e.ok())
                .filter(|e| e.file_type().map(|ft| ft.is_file()).unwrap_or(false));

            for entry in walker {
                let path = entry.path();
                if is_video_extension(path) {
                    trace!(path = %path.display(), "Found video file");
                    files.push(path.to_path_buf());
                }
            }
        }

        debug!(count = files.len(), "Finished scanning for video files");
        files
    }

    fn indexable(config: &Config, mime_type: &mime_type::MimeType) -> Vec<PathBuf> {
        todo!()
    }
}
