use super::{Error, Result};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::{env, fs};

/// Returns true if the given rootfs path contains a mounted `/proc`.
///
/// # Arguments
///
/// * `rootfs` - Path to the root filesystem (e.g., `/`, or a chroot/container mount).
///
/// # Returns
///
/// * `Ok(true)` if `/proc` exists under the provided rootfs.
/// * `Ok(false)` if it does not exist.
///
/// # Errors
///
/// Returns [`Error::ExistenceCheck`] if checking the existence of the `/proc` directory fails.
pub fn contains_proc_mount(rootfs: impl AsRef<Path>) -> Result<bool> {
    let path = rootfs.as_ref().join("proc");

    path.try_exists()
        .map_err(|source| Error::ExistenceCheck { path, source })
}

/// Returns true if the init process PID namespace is different from the current process.
///
/// # Arguments
///
/// * `rootfs` - Path to the container’s root filesystem, used to resolve `/proc/1/ns/pid`.
///
/// # Returns
///
/// * `Ok(true)` if PID namespaces differ (likely in a container).
/// * `Ok(false)` if they are the same.
///
/// # Errors
///
/// * Returns [`Error::ReadSymlink`] if reading the symbolic link for either PID namespace fails.
pub fn is_pid_namespace_isolated(rootfs: impl AsRef<Path>) -> Result<bool> {
    let self_ns_path = Path::new("/proc/self/ns/pid");
    let self_ns = fs::read_link(self_ns_path).map_err(|source| Error::ReadSymlink {
        path: self_ns_path.to_path_buf(),
        source,
    })?;

    let root_ns_path = rootfs.as_ref().join("proc/1/ns/pid");
    let root_ns = fs::read_link(&root_ns_path).map_err(|source| Error::ReadSymlink {
        path: root_ns_path.to_path_buf(),
        source,
    })?;

    Ok(self_ns != root_ns)
}

/// Returns true if the current cgroup hierarchy suggests a containerized environment.
///
/// # Returns
///
/// * `Ok(true)` if container-specific strings or hex-encoded IDs are found in the cgroup info.
/// * `Ok(false)` if no indicators are found.
///
/// # Errors
///
/// * [`Error::FileOpen`] if `/proc/self/cgroup` cannot be opened.
/// * [`Error::ReadLine`] if a line from the file cannot be read.
pub fn matches_container_cgroup() -> Result<bool> {
    let path = Path::new("/proc/self/cgroup");
    let mut buf = BufReader::new(File::open(path).map_err(|source| Error::FileOpen {
        path: path.to_path_buf(),
        source,
    })?);

    let mut line = String::with_capacity(256);

    while buf.read_line(&mut line).map_err(|source| Error::ReadLine {
        path: path.to_path_buf(),
        source,
    })? != 0
    {
        if line.contains("docker")
            || line.contains("kubepods")
            || line.contains("containerd")
            || line.contains("libpod")
        {
            return Ok(true);
        }

        if line
            .split("/")
            .any(|part| part.len() >= 32 && is_non_empty_hex_string(part))
        {
            return Ok(true);
        }

        line.clear();
    }

    Ok(false)
}

/// Returns true if environment markers (files or variables) suggest a containerized environment.
///
/// # Returns
///
/// * `true` if known container markers exist (e.g., `/.dockerenv`, `container` env var).
/// * `false` otherwise.
pub fn has_container_indicators() -> bool {
    fs::metadata("/.dockerenv").is_ok()
        || fs::metadata("/run/.containerenv").is_ok()
        || env::var("container").is_ok()
}

/// Returns true if the input string is not empty and contains only ASCII hex digits.
///
/// # Arguments
///
/// * `s` - A string slice to validate.
///
/// # Returns
///
/// * `true` if the string is fully hexadecimal.
/// * `false` otherwise.
pub fn is_non_empty_hex_string(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    s.chars().all(|c| c.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_hex_string_valid_hex() {
        assert!(is_non_empty_hex_string("deadbeef12345678"));
        assert!(is_non_empty_hex_string("ABCDEFabcdef0123456789"));
    }

    #[test]
    fn test_is_hex_string_invalid_hex() {
        assert!(!is_non_empty_hex_string("deadbeefXYZ"));
        assert!(!is_non_empty_hex_string("1234!@#$"));
        assert!(!is_non_empty_hex_string(""));
    }
}
