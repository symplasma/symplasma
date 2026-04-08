use clap::{Parser, Subcommand};
use symplasma::sources::Source;

#[derive(Parser)]
#[command(name = "symplasma")]
#[command(
    about = "A CLI app to handle entities and common functionality for the Symplasma project"
)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub(crate) enum Commands {
    /// Configuration management
    Config {
        #[command(subcommand)]
        what: ConfigCommands,
    },
    /// List sources or items from a source
    List {
        #[command(subcommand)]
        what: ListCommands,
    },
    /// Find the path to a file
    Find {
        /// Optional source type to scope the search
        #[arg(short, long)]
        source: Option<Source>,
        /// The file name to find
        file_name: String,
    },
    /// Find the path to a file, or create it if it doesn't exist
    FindOrCreate {
        /// Optional source type to scope the search
        #[arg(short, long)]
        source: Option<Source>,
        /// The file name to find or create
        file_name: String,
    },
}

#[derive(Subcommand)]
pub(crate) enum ConfigCommands {
    /// Show the current configuration
    Show,
    /// Create the default configuration file
    CreateDefault,
}

#[derive(Subcommand)]
pub(crate) enum ListCommands {
    /// List all possible data sources
    Sources,
    /// List all files from a specific source
    Files {
        /// The source type to list items from
        source: Source,
    },
    /// List all items from a specific source
    Items {
        /// The source type to list items from
        source: Source,
    },
}
