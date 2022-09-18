use anyhow::Context;
use clap::Parser;
use cli::Cli;
use dir::xdg_data_home;

mod cli;
mod dir;
mod error;

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let cli = Cli::parse();

    let mut trash_home =
        xdg_data_home().context("failed reading XDG_DATA_HOME environment variable")?;
    trash_home.push("Trash");

    Ok(())
}
