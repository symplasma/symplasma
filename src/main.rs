pub mod options;

use clap::Parser;
use symplasma::sources::Source;
use symplasma::{find, find_or_create, list_items, list_sources};

use crate::options::{Cli, Commands, ListCommands};

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
