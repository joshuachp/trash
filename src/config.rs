use std::{convert::TryFrom, path::PathBuf};

use anyhow::anyhow;

use crate::{cli::Cli, dir::xdg_trash_dir};

#[derive(Debug)]
pub struct Config {
    trash_dir: PathBuf,
}

impl TryFrom<&Cli> for Config {
    type Error = anyhow::Error;

    fn try_from(value: &Cli) -> Result<Self, Self::Error> {
        let trash_dir = match value.trash_dir {
            Some(ref dir) => {
                if !dir.is_dir() {
                    return Err(anyhow!("{} is not a directory", dir.to_string_lossy()));
                }

                dir.clone()
            }
            None => xdg_trash_dir()?,
        };

        Ok(Config { trash_dir })
    }
}
