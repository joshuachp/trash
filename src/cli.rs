use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(version, about = "Utility to manage the files Trash can")]
pub struct Cli {
    /// Trash directory to use
    #[clap(long, short)]
    pub trash_dir: Option<PathBuf>,

    #[clap(subcommand)]
    pub action: Action,
}

/// Action to perform on the trash files
#[derive(Subcommand, Debug)]
pub enum Action {
    /// List all the trashed files
    List,
}
