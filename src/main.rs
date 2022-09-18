use std::fs;

use anyhow::Context;
use dir::xdg_data_home;
use log::info;

mod dir;
mod error;

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let mut trash_home =
        xdg_data_home().context("failed reading XDG_DATA_HOME environment variable")?;

    trash_home.push("Trash");

    trash_home
        .as_path()
        .try_exists()
        .and_then(|exists| {
            if !exists {
                info!("creating {:?} directory and its parents", trash_home);
                return fs::create_dir_all(trash_home);
            }
            Ok(())
        })
        .context("failed while making sure Trash direcotry exists")?;

    Ok(())
}
