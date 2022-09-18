use std::ffi::OsString;

#[derive(thiserror::Error, Debug)]
pub enum XdgError {
    #[error("{0} environment variable is a relative path")]
    Relative(&'static str),
    #[error("{0} environment variable is not present")]
    NotPresent(&'static str),
    #[error("{0} environment variable contains invalid unicode characters {1:#?}")]
    NotUnicode(&'static str, OsString),
}
