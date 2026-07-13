use color_eyre::eyre::{bail, eyre, Context, Result};
use scraper::{Html, Selector};
use std::{
    fmt,
    fs::File,
    io::prelude::*,
    path::{Path, PathBuf},
};
use tracing::{debug, trace};

const UNKNOWN_TYPE: &str = "UNKNOWN_TYPE";

/// Represents a Unix Timestamp (seconds since the epoch)
pub type Timestamp = i64;

/// FilePaths represent a path on disk
#[derive(Debug)]
pub struct FilePath {
    pub path: PathBuf,
    pub mime: Vec<String>,
    // pub rowid: OnceCell<RowId>,
    // pub indexed_at: OnceCell<Timestamp>,
}

impl FilePath {
    // TODO: add a find or create method to pull existing paths from the database
    pub fn new(path: PathBuf) -> Result<FilePath> {
        let mime = mime_types(&path).context("could not determine mime type")?;
        debug!(path = %path.display(), ?mime, "Created FilePath");
        Ok(FilePath {
            path,
            mime,
            // rowid: OnceCell::new(),
            // indexed_at: OnceCell::new(),
        })
    }

    pub fn guessed_mime(&self) -> String {
        self.mime
            .first()
            .unwrap_or(&UNKNOWN_TYPE.to_string())
            .to_string()
    }

    // TODO: this needs to be an Option type, local markdown files will not have URLs or file urls
    pub fn uri(&self) -> String {
        // TODO: return the actual URI, probably mime type dependent
        self.path.to_str().unwrap_or("No Uri").to_string()
    }

    pub fn path(&self) -> String {
        self.path.to_str().unwrap_or("No Path").to_string()
    }

    pub fn title(&self) -> String {
        self.path.to_str().unwrap_or("No Title").to_string()
    }

    pub fn open(&self) -> Result<File> {
        File::open(&self.path).context("could not open file")
    }

    pub fn text(&self) -> Result<String> {
        let mut buffer = String::new();
        self.open()?
            .read_to_string(&mut buffer)
            .context("could not read file")?;
        Ok(buffer)
    }

    /// Retrieve only the plain text from the file
    ///
    /// This function attempts to convert a document to plain text in a sensible way based on the guessed mime type.
    pub fn plain_text(&self) -> Result<String> {
        let guessed_mime = self.guessed_mime();
        trace!(path = %self.path(), mime = %guessed_mime, "Extracting plain text");
        match guessed_mime.as_str() {
            "text/html" => {
                // parse html to plain text
                match File::open(self.path.clone()) {
                    Ok(mut file) => {
                        let mut file_contents = String::new();
                        file.read_to_string(&mut file_contents)
                            .context("could not read file")?;
                        // get document text
                        let document = Html::parse_document(&file_contents);
                        let selector = Selector::parse("html")
                            .map_err(|_| eyre!("could not parse selector"))?;
                        match document.select(&selector).next() {
                            Some(element) => Ok(element.text().collect::<String>()),
                            None => bail!("could not find html tag"),
                        }
                    }
                    Err(_e) => Err(eyre!("could not read web archive")),
                }
            }
            "text/markdown" => Ok(self.text()?),
            "text/plain" => Ok(self.text()?),
            guessed_mime => {
                debug!(path = %self.path(), mime = guessed_mime, "Unknown file type");
                bail!("Unknown file type '{}': {}", guessed_mime, self.path())
            }
        }
    }
}

impl fmt::Display for FilePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} ({})",
            self.path(),
            self.mime.join(", "),
            // self.rowid.unwrap_or_else(|| "Unsaved")
        )
    }
}

/// Checks the mime type with the tree_magic_mini library. If that returns "text/plain" or fails to return anything then we use mime_guess to try and find something more specific or something at all.
fn mime_types(path: &Path) -> Result<Vec<String>> {
    // test mime type with tree_magic
    let inferred = infer::get_from_path(path)
        .context("could not read file successfully")?
        .map(|t| t.mime_type());
    trace!(path = %path.display(), ?inferred, "Inferred mime type");
    match inferred {
        // if the type is "text/plain" then check the extension for a better guess
        Some("text/plain") => Ok(mime_guess::from_path(path)
            .iter()
            .map(|m| m.essence_str().to_string())
            .collect::<Vec<String>>()),
        Some(other) => Ok(vec![other.to_string()]),
        None => Ok(mime_guess::from_path(path)
            .iter()
            .map(|m| m.essence_str().to_string())
            .collect::<Vec<String>>()),
    }
}
