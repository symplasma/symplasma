pub mod options;

use std::error::Error;

use clap::Parser;
use strum::IntoEnumIterator as _;
use symplasma::config::Config;
use symplasma::kind::markdown::Markdown;
use symplasma::kind::traits::Kind as _;
use symplasma::kind::web_archives::WebArchive;
use symplasma::sources::Source;
use symplasma::{find, find_or_create};
use tracing::debug;
use tracing_subscriber::EnvFilter;

use crate::options::{Cli, Commands, ConfigCommands, SourceCommands};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| {
        let level = match cli.verbose {
            0 => "info",
            1 => "debug",
            _ => "trace",
        };
        EnvFilter::new(level)
    });
    tracing_subscriber::fmt().with_env_filter(env_filter).init();

    debug!(command = ?cli.command, "Parsed CLI arguments");

    let config = Config::load()?;
    debug!(?config, "Loaded configuration");

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
            handle_source_command(&config, Source::WebArchives, what);
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
    debug!("Handling config show command");
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
    debug!(path = %path.display(), "Handling config create-default command");
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
    debug!("Handling list sources command");
    for source in Source::iter() {
        println!("{}", source);
    }
}

fn handle_source_command(config: &Config, source: Source, what: SourceCommands) {
    debug!(?source, ?what, "Handling source command");
    match what {
        SourceCommands::Files => handle_list_files(config, source),
        SourceCommands::Dirs => handle_list_dirs(config, source),
        SourceCommands::List => handle_list_items(config, source),
        SourceCommands::Find { file_name } => handle_find(Some(source), &file_name),
        SourceCommands::FindOrCreate { file_name } => {
            handle_find_or_create(Some(source), &file_name)
        }
    }
}

fn handle_list_files(config: &Config, source: Source) {
    let items = match source {
        Source::Circles => todo!(),
        Source::Projects => todo!(),
        Source::Repos => todo!(),
        Source::Markdown => Markdown::files(config),
        Source::WebArchives => WebArchive::files(config),
        Source::Pictures => todo!(),
        Source::Videos => todo!(),
        Source::Music => todo!(),
        Source::Audio => todo!(),
    };
    debug!(count = items.len(), ?source, "Found files");
    for item in items {
        println!("{}", item.display());
    }
}

fn handle_list_dirs(config: &Config, source: Source) {
    let items = match source {
        Source::Circles => todo!(),
        Source::Projects => todo!(),
        Source::Repos => todo!(),
        Source::Markdown => todo!(),
        Source::WebArchives => todo!(),
        Source::Pictures => todo!(),
        Source::Videos => todo!(),
        Source::Music => todo!(),
        Source::Audio => todo!(),
    };
    debug!(?source, "Found directories");
    // for item in items {
    //     println!("{}", item.display());
    // }
}

fn handle_list_items(config: &Config, source: Source) {
    let items = match source {
        Source::Circles => todo!(),
        Source::Projects => todo!(),
        Source::Repos => todo!(),
        Source::Markdown => todo!(),
        Source::WebArchives => WebArchive::items(config),
        Source::Pictures => todo!(),
        Source::Videos => todo!(),
        Source::Music => todo!(),
        Source::Audio => todo!(),
    };
    debug!(count = items.len(), ?source, "Found items");
    for item in items {
        println!("{}", item);
    }
}

fn handle_find(source: Option<Source>, file_name: &str) {
    debug!(?source, file_name, "Handling find command");
    match find(source, file_name) {
        Some(path) => println!("{}", path.display()),
        None => {
            eprintln!("File not found: {}", file_name);
            std::process::exit(1);
        }
    }
}

fn handle_find_or_create(source: Option<Source>, file_name: &str) {
    debug!(?source, file_name, "Handling find-or-create command");
    let path = find_or_create(source, file_name);
    println!("{}", path.display());
}
