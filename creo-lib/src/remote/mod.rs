pub mod archive;
mod upload;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("local IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    InvalidExcludePattern(#[from] glob::PatternError),
    #[error("remote IO error: {0}")]
    Remote(#[from] crate::ssh::Error),
}

type Result<T> = std::result::Result<T, Error>;

pub use upload::upload_and_extract_archive;
