use crate::config::Config;
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
    /// The resolved title for this page, computed lazily alongside `html_info`
    /// from (in order of precedence): the `data-scrapbook-title` attribute on
    /// the `html` tag, the `<title>` tag, the first `<h1>` tag, the Open Graph
    /// title, and the Twitter card title.
    resolved_title: OnceLock<Option<String>>,
    // TODO: don't forget to add the creation time from "html.data-scrapbook-create"
    // TODO augment this with a source enum
}

/// Converts newline-type characters to spaces, then squeezes and trims
/// whitespace. Returns `None` if the result is empty.
fn normalize_title(raw: &str) -> Option<String> {
    let normalized: String = raw
        .chars()
        .map(|c| if c == '\n' || c == '\r' { ' ' } else { c })
        .collect();
    let squeezed = normalized.split_whitespace().collect::<Vec<_>>().join(" ");
    let trimmed = squeezed.trim().to_string();
    if trimmed.is_empty() {
        None
    } else {
        Some(trimmed)
    }
}

/// Extracts the `data-scrapbook-title` attribute from the `html` tag, if present.
fn extract_scrapbook_title(document: &Html) -> Option<String> {
    let selector = Selector::parse("html").ok()?;
    let html_tag = document.select(&selector).next()?;
    let raw = html_tag.value().attr("data-scrapbook-title")?;
    normalize_title(raw)
}

/// Extracts the text content of the first `h1` tag, if present.
fn extract_first_h1(document: &Html) -> Option<String> {
    let selector = Selector::parse("h1").ok()?;
    let h1 = document.select(&selector).next()?;
    let text: String = h1.text().collect::<Vec<_>>().join(" ");
    normalize_title(&text)
}

/// Extracts the Open Graph title from parsed webpage info, if present.
fn extract_og_title(html_info: &webpage::HTML) -> Option<String> {
    let value = html_info.opengraph.properties.get("title")?;
    normalize_title(value)
}

/// Extracts the Twitter card title (`meta[name="twitter:title"]`), if present.
fn extract_twitter_title(document: &Html) -> Option<String> {
    let selector = Selector::parse(r#"meta[name="twitter:title"]"#).ok()?;
    let meta = document.select(&selector).next()?;
    let raw = meta.value().attr("content")?;
    normalize_title(raw)
}

/// Resolves the title for a web archive using the following precedence:
///
/// 1. `data-scrapbook-title` attribute on the `html` tag
/// 2. `<title>` tag (as parsed by the `webpage` crate)
/// 3. First `<h1>` tag
/// 4. Open Graph title (`og:title`)
/// 5. Twitter card title (`twitter:title`)
fn extract_title(document: &Html, html_info: &webpage::HTML) -> Option<String> {
    if let Some(title) = extract_scrapbook_title(document) {
        return Some(title);
    }
    if let Some(title) = html_info.title.as_deref().and_then(normalize_title) {
        return Some(title);
    }
    if let Some(title) = extract_first_h1(document) {
        return Some(title);
    }
    if let Some(title) = extract_og_title(html_info) {
        return Some(title);
    }
    if let Some(title) = extract_twitter_title(document) {
        return Some(title);
    }
    None
}

/// A representation of archived web pages
///
/// Currently, this is mostly intended to work with pages archived by [WebScrapbook](https://github.com/danny0838/webscrapbook).
/// In the future, we may support other archive formats like [monolith](https://github.com/Y2Z/monolith) and [Web ARChive file format (WARC)](https://www.loc.gov/preservation/digital/formats/fdd/fdd000236.shtml).
impl WebArchive {
    pub fn dirs(config: &Config) -> Vec<PathBuf> {
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
                items.push(entry.path());
            }
        }

        items
    }

    pub fn items(config: &Config) -> Vec<Self> {
        let mut items = Vec::new();

        for entry in Self::files(config) {
            match WebArchive::new_from_pathbuf(entry.clone()) {
                Ok(archive) => {
                    trace!(path = %entry.display(), "Loaded web archive");
                    items.push(archive);
                }
                Err(e) => {
                    trace!(path = %entry.display(), error = %e, "Failed to load web archive");
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
    /// The title is not resolved here; it is lazily resolved (along with the
    /// rest of the parsed page info) the first time it is requested. See
    /// [`extract_title`] for the precedence used.
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
                    // TODO we should probably insert the title here, but that might prevent lazy loading
                    source: Link::new(uri, None, disk_path),
                    html_info: OnceLock::new(),
                    resolved_title: OnceLock::new(),
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

    /// Ensures that `html_info` and `resolved_title` have been loaded.
    ///
    /// This reads the file once, parses it both as a `scraper::Html` document
    /// (for the custom title sources) and via `webpage::HTML` (for the
    /// standard title/Open Graph/text content parsing), and resolves the
    /// title using the precedence described in [`extract_title`].
    ///
    /// # Errors
    ///
    /// This function will return an error if the file cannot be read, if the
    /// webpage cannot be parsed, or if we cannot store the results in the
    /// `OnceLock`s.
    fn ensure_loaded(&self) -> Result<()> {
        if self.html_info.get().is_some() {
            return Ok(());
        }
        trace!(url = %self.url(), "Lazily loading html_info and title for web archive");

        let path = self.source.disk_path.path();
        let content = std::fs::read_to_string(&path).context("could not read web archive file")?;

        // TODO we're parsing the document twice here, need to decide which crate to use and only parse once
        let document = Html::parse_document(&content);

        let html = HTML::from_string(content.clone(), Some(self.source.raw_uri().to_owned()))
            .context("could not collect webpage info")?;

        let resolved = extract_title(&document, &html);

        self.html_info
            .set(html)
            .map_err(|_| eyre!("could not set html_info"))?;
        self.resolved_title
            .set(resolved)
            .map_err(|_| eyre!("could not set resolved_title"))?;

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
        self.ensure_loaded()?;
        self.html_info.get().ok_or(eyre!("could not get html_info"))
    }

    fn url(&self) -> &str {
        self.source.raw_uri()
    }

    /// Returns the resolved title for this web archive, lazily computed the
    /// first time it is requested. See [`extract_title`] for the precedence
    /// of title sources used.
    fn title(&self) -> Option<&str> {
        if self.ensure_loaded().is_err() {
            return None;
        }
        self.resolved_title.get().and_then(|opt| opt.as_deref())
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
        write!(f, "{} {}", self.title().unwrap_or("NO TITLE"), self.source)
    }
}

impl Kind for WebArchive {
    fn files(config: &Config) -> Vec<PathBuf> {
        let mut items = Vec::new();

        for path in Self::dirs(config) {
            let index_path = if path.is_dir() {
                path.join("index.html")
            } else {
                path.clone()
            };
            if !index_path.is_file() {
                trace!(path = %path.display(), "No index.html found in web archive entry, skipping");
                continue;
            }
            items.push(index_path);
        }

        items
    }

    fn indexable(config: &crate::config::Config, mime_type: &mime_type::MimeType) -> Vec<PathBuf> {
        todo!()
    }
}
