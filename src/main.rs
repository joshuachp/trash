use anyhow::Context;
use dir::xdg_data_home;

mod dir;
mod error;

fn main() -> anyhow::Result<()> {
    env_logger::try_init()?;

    let base_path = xdg_data_home().context("failed reading XDG_DATA_HOME environment variable")?;

    println!("{:?}", base_path);

    Ok(())
}
