pub mod options;

use clap::Parser;
use symplasma::config::Config;
use symplasma::sources::Source;
use symplasma::{find, find_or_create, list_items, list_sources};

use crate::options::{Cli, Commands, ConfigCommands, ListCommands};

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Config { what } => match what {
            ConfigCommands::Show => {
                handle_config_show();
            }
            ConfigCommands::CreateDefault => {
                handle_config_create_default();
            }
        },
        Commands::List { what } => match what {
            ListCommands::Sources => {
                handle_list_sources();
            }
            ListCommands::Files { source } => {
                handle_list_files(source);
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

fn handle_config_show() {
    match Config::load() {
        Ok(config) => {
            println!("Config file: {}", Config::config_path().display());
            println!("{:#?}", config);
        }
        Err(e) => {
            eprintln!("Error loading config: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_config_create_default() {
    let path = Config::config_path();
    if path.exists() {
        eprintln!("Config file already exists at: {}", path.display());
        eprintln!("Remove it first if you want to create a new default config.");
        std::process::exit(1);
    }

    match Config::write_default() {
        Ok(()) => {
            println!("Created default config at: {}", path.display());
        }
        Err(e) => {
            eprintln!("Error creating default config: {}", e);
            std::process::exit(1);
        }
    }
}

fn handle_list_sources() {
    let sources = list_sources();
    for source in sources {
        println!("{}", source);
    }
}

fn handle_list_files(source: Source) {
    let items = list_items(source);
    for item in items {
        println!("{}", item.display());
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
