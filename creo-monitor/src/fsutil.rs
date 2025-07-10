use std::fs::File;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

/// Error that occurs when opening a file fails.
#[derive(Debug, thiserror::Error)]
#[error("failed to open file `{path}`: {source}")]
pub struct FileOpenError {
    pub path: PathBuf,
    #[source]
    pub source: io::Error,
}

/// Opens a file at the given path and wraps it in a [`BufReader`].
///
/// # Errors
///
/// Returns a [`FileOpenError`] if the file cannot be opened.
///
/// # Example
/// ```no_run
/// # use creo_monitor::fsutil;
/// let reader = fsutil::open_file_reader("/some/file.txt")?;
/// # Ok::<(), fsutil::FileOpenError>(())
/// ```
pub fn open_file_reader(path: impl AsRef<Path>) -> Result<BufReader<File>, FileOpenError> {
    let path = path.as_ref();
    let file = File::open(path).map_err(|source| FileOpenError {
        path: path.to_path_buf(),
        source,
    })?;
    Ok(BufReader::new(file))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_open_file_reader_success() {
        let tmp = tempfile::NamedTempFile::new().expect("failed to create temp file");
        let path = tmp.path();
        let reader = open_file_reader(path).expect("should open test file");
        let metadata = reader.get_ref().metadata().unwrap();
        assert!(metadata.is_file());
    }

    #[test]
    fn test_open_file_reader_error() {
        let result = open_file_reader("/definitely/does/not/exist");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.path, PathBuf::from("/definitely/does/not/exist"));
        assert_eq!(err.source.kind(), std::io::ErrorKind::NotFound);
    }
}
