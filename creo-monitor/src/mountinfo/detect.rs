use crate::fsutil;

use super::parser::parse_mount_info_line;
use super::{Error, Result};
use std::io::BufRead;
use std::path::{Path, PathBuf};

/// Detects and validates the cgroup v2 mount point by parsing the given `mountinfo` file.
///
/// This function returns the canonicalized absolute path of the cgroup v2 mount point,
/// ensuring the path exists and is a directory.
///
/// # Arguments
///
/// * `path` - Path to a Linux `mountinfo` file.
///
/// # Returns
///
/// A [`PathBuf`] with the canonicalized cgroup v2 mount point.
///
/// # Errors
///
/// Returns errors from [`detect_cgroup2_mount_point`] and:
///
/// - [`Error::Validation`] if the path cannot be canonicalized or accessed.
/// - [`Error::NotADirectory`] if the resolved path is not a directory.
///
/// # Example
///
/// ```no_run
/// use creo_monitor::mountinfo::detect_validated_cgroup2_mount_point;
///
/// let validated_root = detect_validated_cgroup2_mount_point("/proc/self/mountinfo").unwrap();
/// println!("Validated cgroup2 root: {}", validated_root.display());
/// ```
pub fn detect_validated_cgroup2_mount_point(path: impl AsRef<Path>) -> Result<PathBuf> {
    let raw = detect_cgroup2_mount_point(&path)?;
    let canonical = std::fs::canonicalize(&raw).map_err(|e| Error::Canonicalization {
        path: raw.clone(),
        source: e,
    })?;

    let metadata = std::fs::metadata(&canonical).map_err(|e| Error::Metadata {
        path: canonical.clone(),
        source: e,
    })?;

    if !metadata.is_dir() {
        return Err(Error::NotADirectory { path: canonical });
    }

    Ok(canonical)
}

/// Detects the cgroup v2 mount point by parsing a Linux `mountinfo` file.
///
/// This function scans the file for entries where the filesystem type is `cgroup2`
/// and returns the associated mount point. If multiple `cgroup2` entries exist,
/// the first one is returned.
///
/// # Arguments
///
/// * `path` - Path to a Linux mountinfo file (e.g., `/proc/self/mountinfo`).
///
/// # Returns
///
/// Returns a [`PathBuf`] with the mount point of the cgroup v2 filesystem.
///
/// # Errors
///
/// - [`Error::FileOpen`] if the file can't be opened.
/// - [`Error::ReadLine`] if reading from the file fails.
/// - [`Error::Parse`] if parsing any line fails.
/// - [`Error::MissingCgroup2Mount`] if no `cgroup2` mount is found.
///
/// # Example
///
/// ```no_run
/// use creo_monitor::mountinfo::detect_cgroup2_mount_point;
///
/// let root = detect_cgroup2_mount_point("/proc/self/mountinfo").unwrap();
/// println!("cgroup2 root: {}", root.display());
/// ```
pub fn detect_cgroup2_mount_point(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    let buf = fsutil::open_file_reader(path)?;

    detect_cgroup2_mount_point_from_reader(buf, path)
}

/// Internal implementation for detecting the cgroup v2 mount point from a reader.
///
/// # Arguments
///
/// * `reader` - Buffered reader over the mountinfo content.
/// * `origin` - Logical origin of the data, used in error messages.
///
/// # Returns
///
/// A [`PathBuf`] with the detected `cgroup2` mount point.
///
/// # Errors
///
/// - [`Error::ReadLine`] if reading a line fails.
/// - [`Error::Parse`] if a line fails to parse.
/// - [`Error::MissingCgroup2Mount`] if no matching entry is found.
fn detect_cgroup2_mount_point_from_reader<R: BufRead>(
    mut reader: R,
    origin: &Path,
) -> Result<PathBuf> {
    let mut line = String::with_capacity(256);
    let mut mount_point = None;

    while reader
        .read_line(&mut line)
        .map_err(|source| Error::ReadLine {
            path: origin.to_path_buf(),
            source,
        })?
        != 0
    {
        let mount_info = parse_mount_info_line(line.as_str()).map_err(|source| Error::Parse {
            path: origin.to_path_buf(),
            source,
        })?;
        if mount_info.fs_type == "cgroup2" {
            log::debug!(
                "Found `cgroup2` mount point with root `{}`: {}",
                mount_info.root,
                mount_info.mount_point
            );
            mount_point = Some(PathBuf::from(mount_info.mount_point));
            break;
        }

        line.clear();
    }

    match mount_point {
        Some(mp) => Ok(mp),
        None => Err(Error::MissingCgroup2Mount {
            path: origin.to_path_buf(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn new_cursor_from_contents(contents: &str) -> Cursor<Vec<u8>> {
        Cursor::new(contents.as_bytes().to_vec())
    }

    #[test]
    fn test_detect_single_cgroup2_mount() {
        let input =
            "42 35 0:39 / /sys/fs/cgroup rw nosuid,nodev,noexec,relatime - cgroup2 cgroup rw\n";
        let path = Path::new("/dummy");
        let reader = new_cursor_from_contents(input);

        let mount = detect_cgroup2_mount_point_from_reader(reader, path).unwrap();
        assert_eq!(mount, PathBuf::from("/sys/fs/cgroup"));
    }

    #[test]
    fn test_detect_first_of_multiple_cgroup2_mounts() {
        let input = "\
43 35 0:39 / /sys/fs/cgroup rw nosuid,nodev,noexec,relatime - cgroup2 cgroup rw
42 35 0:39 / /ignored rw nosuid,nodev,noexec,relatime - cgroup2 cgroup rw
";
        let path = Path::new("/dummy");
        let reader = new_cursor_from_contents(input);

        let mount = detect_cgroup2_mount_point_from_reader(reader, path).unwrap();
        assert_eq!(mount, PathBuf::from("/sys/fs/cgroup"));
    }

    #[test]
    fn test_detect_missing_cgroup2_mount() {
        let input = "25 1 0:24 / /proc rw,relatime - proc proc rw\n";
        let path = Path::new("/dummy");
        let reader = new_cursor_from_contents(input);

        let err = detect_cgroup2_mount_point_from_reader(reader, path).unwrap_err();
        match err {
            Error::MissingCgroup2Mount { path: err_path } => assert_eq!(err_path, path),
            other => panic!("unexpected error: {}", other),
        }
    }

    #[test]
    fn test_detect_invalid_line() {
        let input = "invalid mountinfo line";
        let path = Path::new("/dummy");
        let reader = new_cursor_from_contents(input);

        let err = detect_cgroup2_mount_point_from_reader(reader, path).unwrap_err();
        match err {
            Error::Parse { path: err_path, .. } => assert_eq!(err_path, path),
            other => panic!("unexpected error: {:?}", other),
        }
    }

    #[test]
    fn test_detect_from_tempfile() {
        let mut tmp = NamedTempFile::new().unwrap();
        writeln!(
            tmp,
            "42 35 0:39 / /sys/fs/cgroup rw nosuid,nodev,noexec,relatime - cgroup2 cgroup rw"
        )
        .unwrap();

        let mount = detect_cgroup2_mount_point(tmp.path()).unwrap();
        assert_eq!(mount, PathBuf::from("/sys/fs/cgroup"));
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_detect_validated_cgroup2_mount_point_symlink() {
        use std::os::unix::fs as unix_fs;
        let tempdir = tempfile::tempdir().unwrap();

        let symlink_path = tempdir.path().join("symlink_dir");
        unix_fs::symlink(tempdir.path(), &symlink_path).unwrap();

        let mountinfo_content = format!(
            "1 2 0:42 / {} cgroup rw,nosuid,nodev,noexec,relatime - cgroup2 none rw\n",
            symlink_path.display()
        );

        let tmpfile = NamedTempFile::new().unwrap();
        writeln!(&mut tmpfile.as_file(), "{}", mountinfo_content).unwrap();

        let resolved = detect_validated_cgroup2_mount_point(tmpfile.path()).unwrap();
        assert_eq!(resolved, std::fs::canonicalize(&symlink_path).unwrap());
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_detect_validated_cgroup2_mount_point_not_directory() {
        let tempdir = tempfile::tempdir().unwrap();
        let file_path = tempdir.path().join("file");
        std::fs::write(&file_path, "content").unwrap();

        let mountinfo_content = format!(
            "1 2 0:42 / {} cgroup rw,nosuid,nodev,noexec,relatime - cgroup2 none rw\n",
            file_path.display()
        );

        let tmpfile = NamedTempFile::new().unwrap();
        writeln!(&mut tmpfile.as_file(), "{}", mountinfo_content).unwrap();

        let err = detect_validated_cgroup2_mount_point(tmpfile.path()).unwrap_err();
        matches!(err, Error::NotADirectory { .. });
    }

    #[test]
    #[cfg(target_family = "unix")]
    fn test_detect_validated_cgroup2_mount_point_broken_symlink() {
        use std::os::unix::fs as unix_fs;
        let tempdir = tempfile::tempdir().unwrap();
        let target = tempdir.path().join("non_existent");
        let symlink = tempdir.path().join("symlink");
        unix_fs::symlink(&target, &symlink).unwrap();

        let mountinfo_content = format!(
            "1 2 0:42 / {} cgroup rw,nosuid,nodev,noexec,relatime - cgroup2 none rw\n",
            symlink.display()
        );

        let tmpfile = NamedTempFile::new().unwrap();
        writeln!(&mut tmpfile.as_file(), "{}", mountinfo_content).unwrap();

        let err = detect_validated_cgroup2_mount_point(tmpfile.path()).unwrap_err();
        matches!(err, Error::Canonicalization { .. });
    }
}
