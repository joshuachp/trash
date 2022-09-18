//! Gets the directories following the [freedesktop.org BaseDir Specification](https://specifications.freedesktop.org/basedir-spec/latest/index.html)

use std::{
    env::{self, VarError},
    fs,
    path::PathBuf,
};

use anyhow::Context;
use log::{info, warn};

use crate::error::XdgError;

/// Gets the `XDG_DATA_HOME` environment variable.
pub fn xdg_data_home() -> Result<PathBuf, XdgError> {
    match env::var("XDG_DATA_HOME").map(PathBuf::from) {
        Ok(path) if path.is_relative() => {
            warn!("XDG_DATA_HOME environment variable is a relative path, ignoring");
        }
        Err(VarError::NotPresent) => {
            warn!("XDG_DATA_HOME environment variable not present");
        }
        Err(VarError::NotUnicode(os_string)) => {
            return Err(XdgError::NotUnicode("XDG_DATA_HOME", os_string))
        }
        Ok(path) => {
            return Ok(path);
        }
    }

    match env::var("HOME").map(PathBuf::from) {
        Ok(home) if home.is_relative() => Err(XdgError::Relative("HOME")),
        Ok(mut home) => {
            home.push(".local/share");
            Ok(home)
        }
        Err(VarError::NotPresent) => Err(XdgError::NotPresent("HOME")),
        Err(VarError::NotUnicode(os_string)) => Err(XdgError::NotUnicode("HOME", os_string)),
    }
}

pub fn xdg_trash_dir() -> anyhow::Result<PathBuf> {
    let mut path = xdg_data_home().context("failed getting XDG_DATA_HOME environment variable")?;

    path.push("Trash");

    let exists = path
        .try_exists()
        .context("failed while checking the existence of the Trash directory")?;

    if !exists {
        info!("creating {:?} directory and its parents", &path);

        fs::create_dir_all(&path).context("failed to create Trash directory")?;
    }

    Ok(path)
}
