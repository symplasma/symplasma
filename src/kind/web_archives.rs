use crate::config::{self, Config};
use crate::kind::traits::Kind;
use crate::kind::util::expand_tilde;
use crate::model::file_path::FilePath;
use crate::model::link::Link;

use color_eyre::eyre::{self, eyre, Context, ContextCompat, Result};
use scraper::{Html, Selector};
use std::fmt;
use std::path::PathBuf;
use std::sync::{Arc, OnceLock};
use std::{
    fmt::Display,
    io::{BufRead, BufReader},
};
use tracing::{debug, trace};
use webpage::HTML;

#[derive(Debug)]
pub struct WebArchive {
    /// The link from which this page was archived
    source: Link,
    /// holds title, plain text, open graph, and various other data
    html_info: OnceLock<webpage::HTML>,
    // TODO: don't forget to add the creation time from "html.data-scrapbook-create"
    // TODO augment this with a source enum
}

/// A representation of archived web pages
///
/// Currently, this is mostly intended to work with pages archived by [WebScrapbook](https://github.com/danny0838/webscrapbook).
/// In the future, we may support other archive formats like [monolith](https://github.com/Y2Z/monolith) and [Web ARChive file format (WARC)](https://www.loc.gov/preservation/digital/formats/fdd/fdd000236.shtml).
impl WebArchive {
    pub fn items(config: &Config) -> Vec<Self> {
        let mut items = Vec::new();

        for base_path in &config.web_archives {
            let expanded_base = expand_tilde(base_path);
            let data_dir = expanded_base.join("data");
            let dir = if data_dir.is_dir() {
                data_dir
            } else {
                expanded_base
            };
            debug!(dir = %dir.display(), "Scanning for web archives");

            let entries = match std::fs::read_dir(&dir) {
                Ok(entries) => entries,
                Err(e) => {
                    debug!(dir = %dir.display(), error = %e, "Could not read directory");
                    continue;
                }
            };

            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                let index_path = if path.is_dir() {
                    path.join("index.html")
                } else {
                    path.clone()
                };
                if !index_path.is_file() {
                    trace!(path = %path.display(), "No index.html found in web archive entry, skipping");
                    continue;
                }
                match WebArchive::new_from_pathbuf(index_path.clone()) {
                    Ok(archive) => {
                        trace!(path = %index_path.display(), "Loaded web archive");
                        items.push(archive);
                    }
                    Err(e) => {
                        trace!(path = %index_path.display(), error = %e, "Failed to load web archive");
                    }
                }
            }
        }

        debug!(count = items.len(), "Finished scanning for web archives");
        items
    }

    pub(crate) fn new_from_pathbuf(path: PathBuf) -> Result<WebArchive> {
        trace!(path = %path.display(), "Creating WebArchive from path");
        let file_path = FilePath::new(path)?;
        Self::new(file_path.into())
    }

    /// Creates a new WebArchive
    ///
    /// If we can successfully read `disk_path` we return a WebArchive otherwise we return an error.
    /// If you want to work with a page prior to it being archived on disk, use a [Link] instead.
    ///
    /// ## Source Link
    ///
    /// Tries to retrieve the link from the `data-scrapbook-source` attribute of the `html` tag. This should be added automatically by [WebScrapbook](https://github.com/danny0838/webscrapbook).
    ///
    /// ## Title
    ///
    /// We try to retrieve the title by:
    ///
    /// 1. Concatenating the text content of all `title` tags (some sites mistakenly use more than one).
    /// 2. Finding the first `H1` on the page and pulling its text content.
    pub fn new(disk_path: Arc<FilePath>) -> Result<Self> {
        // read file from disk
        match disk_path.open() {
            Ok(file) => {
                // TODO: use lol_html to remove scripts, styles, and other cruft to speed additional parsing tasks
                // read additional data not picked up by the webpage crate
                // e.g. the attributes added by WebScrapbook and the first H1 as title if not provided
                let reader = BufReader::new(file);
                let file_contents = reader
                    .lines()
                    .take(2)
                    .filter_map(|l| l.ok())
                    .collect::<Vec<String>>()
                    .join("\n");
                // get source uri
                let document = Html::parse_document(&file_contents);
                let selector =
                    Selector::parse("html").map_err(|_| eyre!("could not parse html selector"))?;
                let html_tag = document
                    .select(&selector)
                    .next()
                    .context("could not find HTML tag")?;
                let uri = html_tag
                    .value()
                    .attr("data-scrapbook-source")
                    .context("could not get 'data-scrapbook-source' attr")?
                    .to_string();
                // TODO: get the archive date from "data-scrapbook-create"
                debug!(uri, path = %disk_path.path(), "Created WebArchive from disk");

                Ok(WebArchive {
                    source: Link::new(uri, None, disk_path),
                    html_info: OnceLock::new(),
                })
            }
            Err(_e) => {
                debug!(path = %disk_path.path(), "Could not read web archive from disk");
                Err(eyre!("could not read web archive"))
            }
        }
    }

    pub(crate) fn open(&self) -> eyre::Result<()> {
        let disk_path = self.source.disk_path().path();
        debug!("Opening disk path {}...", disk_path);
        // TODO need to make this open the local file
        open::that_detached(&disk_path)?;
        Ok(())
    }

    /// Returns the html info of this [`WebArchive`].
    ///
    /// This function lazily loads information for the webpage. We're doing this because loading the whole file and parsing it is a time consuming operation.
    ///
    /// # Errors
    ///
    /// This function will return an error if the webpage cannot be parsed, if we cannot store the result in the `OnceLock` or if we cannot read the existing `webpage::HTML` from the `OnceLock`.
    fn html_info(&self) -> Result<&webpage::HTML> {
        if self.html_info.get().is_none() {
            trace!(url = %self.url(), "Lazily loading html_info for web archive");
            let html = HTML::from_file(
                &self.source.disk_path.path(),
                Some(self.source.raw_uri().to_owned()),
            )
            .context("could not collect webpage info")?;

            self.html_info
                .set(html)
                // TODO replace the debug format specifier with a more idiomatic call
                .map_err(|e| eyre::eyre!("{:?}", e))?;
        }
        self.html_info.get().ok_or(eyre!("could not get html_info"))
    }

    fn url(&self) -> &str {
        self.source.raw_uri()
    }

    fn title(&self) -> Option<&str> {
        // TODO: get the title from the first H1 if there is no title tag
        //       note that this will likely cause a slowdown in parsing as we'll have to look at much more of the file

        // TODO do we want to update the source title here?
        self.source.title()
    }

    fn disk_path(&self) -> Arc<FilePath> {
        self.source.disk_path()
    }

    fn plain_text(&self) -> Option<&str> {
        self.html_info().map(|h| h.text_content.as_str()).ok()
    }

    /// Returns a reference to a vector of links from the file
    ///
    /// We are returning a result since we are parsing the file here.
    /// The file could be fully parsed in the constructor and the error thrown there, but this allows us to defer parsing until the call to links.
    /// Though, this may cause the file to be opened twice.
    fn links(&mut self) -> Result<Vec<Link>> {
        let links: Vec<Link> = self
            .html_info()
            .context("could not get html_info")?
            .links
            .iter()
            .map(|l| Link::new(l.url.clone(), Some(l.text.clone()), self.disk_path()))
            .collect();
        debug!(count = links.len(), url = %self.url(), "Extracted links from web archive");
        Ok(links)
    }
}

impl Display for WebArchive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Delegate to the PathBuf's display implementation
        write!(f, "{}", self.source)
    }
}

impl Kind for WebArchive {
    fn files(config: &crate::config::Config) -> Vec<PathBuf> {
        todo!()
    }

    fn indexable(config: &crate::config::Config, mime_type: &mime_type::MimeType) -> Vec<PathBuf> {
        todo!()
    }
}
