use std::convert::TryInto;

use clap::Parser;
use cli::Cli;
use config::Config;

mod cli;
mod config;
mod dir;
mod error;

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let config: Config = Cli::parse().try_into()?;

    println!("{:?}", config);

    Ok(())
}
