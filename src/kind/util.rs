use std::path::{Path, PathBuf};
use tracing::trace;

/// Expands a leading `~` in a path to the user's home directory.
pub fn expand_tilde(path: &Path) -> PathBuf {
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
