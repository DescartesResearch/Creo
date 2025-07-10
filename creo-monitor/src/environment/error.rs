use std::path::PathBuf;

/// Errors that may occur during environment detection.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to check if path `{path}` exists: {source}")]
    ExistenceCheck {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read symlink `{path}`: {source}")]
    ReadSymlink {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to open file `{path}`: {source}")]
    FileOpen {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read line for file `{path}`: {source}")]
    ReadLine {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, Error>;
