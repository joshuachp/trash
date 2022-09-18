use clap::Parser;
use cli::{Action, Cli};
use config::Config;

mod cli;
mod config;
mod dir;
mod entry;
mod error;

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let cli = Cli::parse();
    let config = Config::try_from(&cli)?;

    match cli.action {
        Action::List => todo!(),
    }
}
