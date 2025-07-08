use std::env;
use std::fs::{self, File};
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

pub mod api;
pub mod cgroup;
pub mod container;
pub mod discovery;
pub mod error;
pub mod grpc;
pub mod persistence;

// in container it is really important to have "--privileged"
// check for container environment
//  check if /rootfs is there
//  check if namespaces are different
//  check if /proc/self/cgroup contains containerized parts
//  check if in container env, e.g., /.dockerenv
//
//  if in container and /rootfs missing: error
//  if not detected in container, but /proc/self/mountinfo returns multiple cgroup mounts -> warn
//  about missing "--privileged"
//  if in container and /rootfs there: everything as expected

// check /proc/<pid>/mountinfo for cgroup root

// Get PID of container
// check /proc/<pid>/cgroup for cgroup stat files
//  file format: <hierarchy-id>:<controller-list>:<cgroup-path>
//      <hierarchy-id>:
//          v1: arbitrary number
//          v2: always '0'
//      <controller-list>:
//          v1: comma-separated list of controllers, e.g., cpu,memory
//          v2: always empty, i.e., ''
//      <cgroup-path>:
//          v1: path of the controllers in the controller-list relative to the cgroup root
//          v2: unified path of all controllers relative to the cgroup root
//
// TODO: check if anything different from /rootfs/sys/fs/cgroup and /sys/fs/cgroup
// TODO: check if I can use /rootfs/var/run/containerd/containerd.sock
//
// Containerd API:
//  at startup: list namespaces -> for each namespace list tasks -> filter only running tasks ->
//  get pid from responses
//  subscribe to topic==/tasks/start -> read namespace from event -> read pid from event
//  subscribe to topic==/tasks/delete -> read namespace from event -> check if id (i.e. exec_id) is
//  "" (means that root exec_id is deleted) -> stop tracking

pub mod containerd {
    pub mod runc {
        pub mod v1 {
            tonic::include_proto!("containerd.runc.v1");
        }
    }
    pub mod v1 {
        pub mod types {
            tonic::include_proto!("containerd.v1.types");
        }
    }
    pub mod types {
        tonic::include_proto!("containerd.types");
    }
    pub mod events {
        tonic::include_proto!("containerd.events");
    }
    pub mod services {
        pub mod containers {
            pub mod v1 {
                tonic::include_proto!("containerd.services.containers.v1");
            }
        }
        pub mod events {
            pub mod v1 {
                tonic::include_proto!("containerd.services.events.v1");
            }
        }
        pub mod tasks {
            pub mod v1 {
                tonic::include_proto!("containerd.services.tasks.v1");
            }
        }
        pub mod namespaces {
            pub mod v1 {
                tonic::include_proto!("containerd.services.namespaces.v1");
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to check if path `{path}` exists: {source}")]
    ExistenceCheckError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read symlink `{path}`: {source}")]
    ReadSymlinkError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to open file `{path}`: {source}")]
    FileOpenError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read line for file `{path}`: {source}")]
    ReadLineError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}

pub enum RuntimeEnvironment {
    Host,
    Container,
}

pub fn detect_runtime_environment(rootfs: impl AsRef<Path>) -> RuntimeEnvironment {
    match has_rootfs_proc(rootfs.as_ref()) {
        Ok(true) => match is_namespace_different_from_host(rootfs.as_ref()) {
            Ok(true) => {
                return RuntimeEnvironment::Container;
            }
            Ok(false) => {}
            Err(err) => log::warn!("{}", err),
        },
        Ok(false) => {}
        Err(err) => log::warn!("{}", err),
    }

    match in_container_cgroup() {
        Ok(true) => {
            return RuntimeEnvironment::Container;
        }
        Ok(false) => {}
        Err(err) => log::warn!("{}", err),
    }

    if in_container_env() {
        return RuntimeEnvironment::Container;
    }

    RuntimeEnvironment::Host
}

fn has_rootfs_proc(rootfs: impl AsRef<Path>) -> Result<bool, Error> {
    let path = rootfs.as_ref().join("proc");

    path.try_exists()
        .map_err(|source| Error::ExistenceCheckError {
            path: path.to_path_buf(),
            source,
        })
}

fn is_namespace_different_from_host(rootfs: impl AsRef<Path>) -> Result<bool, Error> {
    let path = Path::new("/proc/self/ns/pid");
    let self_ns = fs::read_link(path).map_err(|source| Error::ReadSymlinkError {
        path: path.to_path_buf(),
        source,
    })?;

    let path = rootfs.as_ref().join("proc/1/ns/pid");
    let host_ns = fs::read_link(&path).map_err(|source| Error::ReadSymlinkError {
        path: path.to_path_buf(),
        source,
    })?;

    Ok(self_ns != host_ns)
}

fn in_container_cgroup() -> Result<bool, Error> {
    let path = Path::new("/proc/self/cgroup");
    let mut buf = BufReader::new(File::open(path).map_err(|source| Error::FileOpenError {
        path: path.to_path_buf(),
        source,
    })?);

    let mut line = String::with_capacity(256);

    while buf
        .read_line(&mut line)
        .map_err(|source| Error::ReadLineError {
            path: path.to_path_buf(),
            source,
        })?
        != 0
    {
        if line.contains("docker")
            || line.contains("kubepods")
            || line.contains("containerd")
            || line.contains("libpod")
        {
            return Ok(true);
        }

        if line.split("/").any(|part| part.len() >= 32 && is_hex(part)) {
            return Ok(true);
        }

        line.clear();
    }

    Ok(false)
}

fn is_hex(s: &str) -> bool {
    s.chars().all(|c| "a..f".contains(c) || "1..9".contains(c))
}

fn in_container_env() -> bool {
    fs::metadata("/.dockerenv").is_ok()
        || fs::metadata("/run/.containerenv").is_ok()
        || env::var("container").is_ok()
}

#[derive(Debug, thiserror::Error)]
pub enum MountInfoError {
    #[error("missing mount info separator '-' in line `{0}`")]
    MissingSeparatorInLine(String),
    #[error("not enough pre-separator fields in line `{0}`")]
    NotEnoughPreSeparatorFields(String),
    #[error("not enough post-separator fields in line `{0}`")]
    NotEnoughPostSeparatorFields(String),
}

/// Extracts the mountpoint from the given mountinfo line if the filesystem type is cgroup2.
///
/// # Notes
///
/// See [`proc_pid_mountinfo(5)`](https://man7.org/linux/man-pages/man5/proc_pid_mountinfo.5.html)
/// for the expected format.
fn extract_cgroup_v2_mount_point(line: &str) -> std::result::Result<Option<&str>, MountInfoError> {
    // MountInfo Layout
    // <mount-id> <parent-id> <major>:<minor> <root> <mount-point> <optional-fields> - <fs-type> <source> <super-options>
    let parts = match line.split_once("-") {
        None => {
            return Err(MountInfoError::MissingSeparatorInLine(line.to_owned()));
        }
        Some(parts) => parts,
    };

    let fields = parts.0;
    let info = parts.1;
    let mut it = fields.split_whitespace().take(6);

    for _ in 0..3 {
        it.next()
            .ok_or_else(|| MountInfoError::NotEnoughPreSeparatorFields(line.to_owned()))?;
    }
    let mount_point = it
        .next()
        .ok_or_else(|| MountInfoError::NotEnoughPreSeparatorFields(line.to_owned()))?;

    if it.count() < 2 {
        return Err(MountInfoError::NotEnoughPreSeparatorFields(line.to_owned()));
    }

    if fields.split_whitespace().take(6).count() < 6 {
        return Err(MountInfoError::NotEnoughPreSeparatorFields(line.to_owned()));
    }

    let mut it = info.split_whitespace();
    let fs_type = it
        .next()
        .ok_or_else(|| MountInfoError::NotEnoughPostSeparatorFields(line.to_owned()))?;
    if it.take(2).count() < 2 {
        return Err(MountInfoError::NotEnoughPostSeparatorFields(
            line.to_owned(),
        ));
    }

    if fs_type == "cgroup2" {
        return Ok(Some(mount_point));
    }

    Ok(None)
}

#[derive(Debug, thiserror::Error)]
pub enum CgroupError {
    #[error("failed to open file `{path}`: {source}")]
    FileOpenError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to read line for file `{path}`: {source}")]
    ReadLineError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("failed to detect cgroup v2 mount point in file `{path}`")]
    DetectionError { path: PathBuf },
}

pub fn detect_cgroup_root(path: impl AsRef<Path>) -> Result<PathBuf, CgroupError> {
    let path = path.as_ref();
    let mut buf =
        BufReader::new(
            File::open(path).map_err(|source| CgroupError::FileOpenError {
                path: path.to_path_buf(),
                source,
            })?,
        );

    let mut line = String::with_capacity(256);
    let mut mount_point = None;

    while buf
        .read_line(&mut line)
        .map_err(|source| CgroupError::ReadLineError {
            path: path.to_path_buf(),
            source,
        })?
        != 0
    {
        match extract_cgroup_v2_mount_point(line.as_str()) {
            Ok(Some(mp)) => {
                log::debug!("Found cgroup v2 mount point at `{}`", mp);
                mount_point = Some(PathBuf::from(mp));
                // break;
            }
            Ok(None) => {}
            Err(err) => {
                log::warn!("{}", err);
            }
        }

        line.clear();
    }

    match mount_point {
        None => Err(CgroupError::DetectionError {
            path: path.to_path_buf(),
        }),
        Some(mp) => Ok(mp),
    }
}
