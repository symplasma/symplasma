use clap::{Parser, Subcommand};
use symplasma::sources::Source;
use symplasma::{find, find_or_create, list_items, list_sources};

#[derive(Parser)]
#[command(name = "symplasma")]
#[command(about = "A CLI app to handle entities and common functionality for the Symplasma project")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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
enum ListCommands {
    /// List all possible data sources
    Sources,
    /// List all items from a specific source
    Items {
        /// The source type to list items from
        source: Source,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::List { what } => match what {
            ListCommands::Sources => {
                handle_list_sources();
            }
            ListCommands::Items { source } => {
                handle_list_items(source);
            }
        },
        Commands::Find { source, file_name } => {
            handle_find(source, &file_name);
        }
        Commands::FindOrCreate { source, file_name } => {
            handle_find_or_create(source, &file_name);
        }
    }
}

fn handle_list_sources() {
    let sources = list_sources();
    for source in sources {
        println!("{}", source);
    }
}

fn handle_list_items(source: Source) {
    let items = list_items(source);
    for item in items {
        println!("{}", item.display());
    }
}

fn handle_find(source: Option<Source>, file_name: &str) {
    match find(source, file_name) {
        Some(path) => println!("{}", path.display()),
        None => {
            eprintln!("File not found: {}", file_name);
            std::process::exit(1);
        }
    }
}

fn handle_find_or_create(source: Option<Source>, file_name: &str) {
    let path = find_or_create(source, file_name);
    println!("{}", path.display());
}
