use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(version, about = "Utility to manage the files Trash can")]
pub struct Cli {
    /// Trash directory to use
    #[clap(long, short)]
    pub trash_dir: Option<PathBuf>,
}
