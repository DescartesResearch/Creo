use std::path::PathBuf;

use crate::fsutil;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    FileOpen(#[from] fsutil::FileOpenError),
    #[error("failed to read line for file `{path}`: {source}")]
    ReadLine {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to detect cgroup v2 mount point in file `{path}`")]
    MissingCgroup2Mount { path: PathBuf },
    #[error("failed to parse line in file `{path}`: {source}")]
    Parse {
        path: PathBuf,
        #[source]
        source: super::parser::ParseError,
    },
    #[error("failed to canonicalize cgroup2 mount path `{path}`: {source}")]
    Canonicalization {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read metadata of cgroup2 mount path `{path}`: {source}")]
    Metadata {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("cgroup2 mount path `{path}` is not a directory")]
    NotADirectory { path: PathBuf },
}

pub type Result<T> = std::result::Result<T, Error>;
