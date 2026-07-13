use clap::{Parser, Subcommand};

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
    /// List all possible data sources
    Sources,
    /// Circles source commands
    Circles {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Projects source commands
    Projects {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Repos source commands
    Repos {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Markdown source commands
    Markdown {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Web archives source commands
    #[command(alias = "web-scrap-book-archives")]
    WebArchives {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Pictures source commands
    Pictures {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Videos source commands
    Videos {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Music source commands
    Music {
        #[command(subcommand)]
        what: SourceCommands,
    },
    /// Audio source commands
    Audio {
        #[command(subcommand)]
        what: SourceCommands,
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
pub(crate) enum SourceCommands {
    /// List all files from this source
    Files,
    /// List all directories from this source
    Dirs,
    /// List all items from this source
    List,
    /// Find the path to a file
    Find {
        /// The file name to find
        file_name: String,
    },
    /// Find the path to a file, or create it if it doesn't exist
    FindOrCreate {
        /// The file name to find or create
        file_name: String,
    },
}
