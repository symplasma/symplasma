use crate::model::file_path::FilePath;
use color_eyre::eyre::{Context, Result};
use std::{
    fmt::{self, Display},
    sync::Arc,
};
use tracing::{debug, trace};
use url::Url;

/// Holds link information
///
/// This class is intended to hold the required fields for inserting into the link database.
#[derive(Debug)]
pub struct Link {
    /// The raw link before attempting to parse
    raw_uri: String,
    /// The result of parsing the raw_uri with the Rust [Url] library
    pub url: Result<Url>,
    /// An optional title for the link, provided at creation time or pulled from the link later
    // TODO change to a more descriptive enum that clarifies missing titles vs unloaded titles
    pub title: Option<String>,
    /// The file on disk from which the link originated
    // TODO: probably need to make this an option
    pub disk_path: Arc<FilePath>,
    // TODO augment this with a source enum
}

impl Link {
    pub fn new(uri: String, title: Option<String>, disk_path: Arc<FilePath>) -> Self {
        // TODO: make this return a result object or set an appropriate status for unparseable links
        let parsed_url = Url::parse(&uri).context("Could not parse Url");
        match &parsed_url {
            Ok(url) => trace!(uri, %url, "Parsed link URL"),
            Err(e) => debug!(uri, error = %e, "Could not parse link URL"),
        }
        Link {
            raw_uri: uri,
            url: parsed_url,
            title,
            disk_path,
        }
    }

    pub fn raw_uri(&self) -> &str {
        &self.raw_uri
    }

    pub fn url(&self) -> &Result<Url> {
        &self.url
    }

    pub fn title(&self) -> Option<&str> {
        self.title.as_deref()
    }

    pub fn disk_path(&self) -> Arc<FilePath> {
        self.disk_path.clone()
    }

    pub fn scheme(&self) -> Option<&str> {
        match &self.url {
            Ok(u) => Some(u.scheme()),
            Err(_) => None,
        }
    }

    pub fn host(&self) -> Option<String> {
        match &self.url {
            Ok(u) => u.host().map(|h| h.to_string()),
            Err(_) => None,
        }
    }

    pub fn port_or_known_default(&self) -> u16 {
        self.url
            .as_ref()
            .map_or(0, |u| u.port_or_known_default().unwrap_or(0))
    }

    pub fn url_path(&self) -> Option<&str> {
        match &self.url {
            Ok(u) => Some(u.path()),
            Err(_) => None,
        }
    }

    pub fn query(&self) -> Option<&str> {
        self.url.as_ref().map_or(None, |u| u.query())
    }

    pub fn fragment(&self) -> Option<&str> {
        self.url.as_ref().map_or(None, |u| u.fragment())
    }

    /// Determines if we should store the link in the database
    ///
    /// Link schemes we should not store:
    ///
    /// * `javascript`
    /// * invalid links with no scheme
    ///
    /// Other reasons not to store a link
    ///
    /// * anchor links
    fn should_store(&self) -> bool {
        // does link have a valid FilePath in the database?

        if self.raw_uri.starts_with('#') {
            trace!(uri = %self.raw_uri, "Not storing anchor link");
            return false;
        };
        let store = !matches!(self.scheme(), Some("javascript") | None);
        if !store {
            trace!(uri = %self.raw_uri, "Not storing link with excluded scheme");
        }
        store
    }
}

impl Display for Link {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the PathBuf's display implementation
        write!(f, "{}", self.raw_uri)
    }
}
