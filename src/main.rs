pub mod options;

use std::error::Error;

use clap::Parser;
use strum::IntoEnumIterator as _;
use symplasma::config::Config;
use symplasma::sources::Source;
use symplasma::{find, find_or_create, list_files};

use crate::options::{Cli, Commands, ConfigCommands, SourceCommands};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let config = Config::load()?;

    match cli.command {
        Commands::Config { what } => match what {
            ConfigCommands::Show => {
                handle_config_show();
            }
            ConfigCommands::CreateDefault => {
                handle_config_create_default();
            }
        },
        Commands::Sources => {
            handle_list_sources();
        }
        Commands::Circles { what } => {
            handle_source_command(&config, Source::Circles, what);
        }
        Commands::Projects { what } => {
            handle_source_command(&config, Source::Projects, what);
        }
        Commands::Repos { what } => {
            handle_source_command(&config, Source::Repos, what);
        }
        Commands::Markdown { what } => {
            handle_source_command(&config, Source::Markdown, what);
        }
        Commands::WebArchives { what } => {
            handle_source_command(&config, Source::WebScrapBookArchive, what);
        }
        Commands::Pictures { what } => {
            handle_source_command(&config, Source::Pictures, what);
        }
        Commands::Videos { what } => {
            handle_source_command(&config, Source::Videos, what);
        }
        Commands::Music { what } => {
            handle_source_command(&config, Source::Music, what);
        }
        Commands::Audio { what } => {
            handle_source_command(&config, Source::Audio, what);
        }
    }

    Ok(())
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
    for source in Source::iter() {
        println!("{}", source);
    }
}

fn handle_source_command(config: &Config, source: Source, what: SourceCommands) {
    match what {
        SourceCommands::List => handle_list_files(config, source),
        SourceCommands::Find { file_name } => handle_find(Some(source), &file_name),
        SourceCommands::FindOrCreate { file_name } => {
            handle_find_or_create(Some(source), &file_name)
        }
    }
}

fn handle_list_files(config: &Config, source: Source) {
    let items = list_files(config, source);
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
