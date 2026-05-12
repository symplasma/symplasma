use std::path::PathBuf;

use crate::kind::traits::Kind;

pub struct WebScrapBookArchive {
    path: PathBuf,
}
impl Kind for WebScrapBookArchive {
    fn files(config: &crate::config::Config) -> Vec<PathBuf> {
        todo!()
    }

    fn indexable(config: &crate::config::Config, mime_type: &mime_type::MimeType) -> Vec<PathBuf> {
        todo!()
    }
}
