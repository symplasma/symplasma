pub mod cli;
pub mod options;

use std::error::Error;

use clap::Parser;
use symplasma::config::Config;
use tracing::debug;
use tracing_subscriber::EnvFilter;

use crate::cli::run;
use crate::options::Cli;

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

    run(cli, config);

    Ok(())
}
