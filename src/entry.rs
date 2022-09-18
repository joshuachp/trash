//! Entry in the Trash folder.
//!
//! The file name in the $trash/files of a new file will be the original file name. If another file
//! with the same name exists we will add a incremental number between the extension and the file
//! name devided by dots.
//!
//! For example `file.txt` will became `file.2.txt` if a `file.txt` already exists.
//!
//! This is the same behaviour as in Gnome Nautilus as for 2022-09-18.

use std::path::PathBuf;

use chrono::{DateTime, Local};

/// Rappresenta trashed file and its information
#[derive(Debug)]
struct Entry {
    file_path: PathBuf,
    info_path: PathBuf,
    info: FileInfo,
}

#[derive(Debug)]
struct FileInfo {
    path: PathBuf,
    deletion_date: DateTime<Local>,
}
