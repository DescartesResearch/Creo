//! Container discovery and resource monitoring using cgroup-based introspection.
//!
//! This module provides tools to identify, track, and collect runtime statistics
//! for containers using the Linux cgroup filesystem (primarily cgroup v2). It
//! enables integration of container lifecycle and resource usage insights into
//! a broader monitoring or observability framework.
//!
//! # Features
//!
//! - Scans cgroup filesystem paths to discover containers and pods.
//! - Tracks container process IDs and associates them with monitoring state.
//! - Collects per-container resource statistics (CPU, memory, I/O, network).
//! - Cleans up stale containers no longer present in the cgroup tree.
//!
//! # Key Components
//!
//! - [`ContainerScanner`] — A trait for scanning cgroup roots and registering containers.
//! - [`ContainerSlice`] — A variant enum distinguishing standalone vs. pod-scoped containers.
//! - [`ContainerMonitor`] — Maintains stat file handles and extracts runtime metrics.
//! - [`Monitor`] — Aggregates all active containers, manages lifecycle and stat collection.
//!
//! # Supported Stats
//!
//! The following cgroup and procfs files are monitored, if available:
//!
//! - `cpu.stat` and `cpu.max`
//! - `memory.stat`, `memory.current`, and `memory.max`
//! - `io.stat`
//! - `/proc/<pid>/net/dev` (for each PID) for network stats
//!
//! # Example
//!
//! ```rust
//! # use creo_monitor::cgroup::{Monitor, ContainerScanner};
//! # use creo_monitor::cgroup::v2::Scanner;
//!
//! # #[tokio::test]
//! # async fn example() {
//! let scanner = Scanner::default();
//! let mut monitor = Monitor::default();
//! let mut containerd_meta_provider = ContainerDMetaProvider::new().await;
//!
//! match scanner.scan_path(std::path::Path::new("/sys/fs/cgroup"), &mut monitor, &mut
//! containerd_meta_provider) {
//!     Ok(_) => {},
//!     Err(err) => println!("Failed to scan path: {}", err),
//! }
//! monitor.collect_stats(1_723_456_789); // Supply a timestamp
//! let stats = monitor.drain_stats();
//! println!("Collected {} metrics", stats.len());
//! # }
//! ```
//!
//! # Platform Requirements
//!
//! - Linux with cgroup v2 support (via the `cgroup-v2` Cargo feature).
//! - Read access to `/sys/fs/cgroup` and `/proc/<pid>/net/dev`.
//!
//! # Optional Features
//!
//! - `cgroup-v2`: Enables the v2 hierarchy scanner and metrics parsing.
//!
//! # See Also
//!
//! - [`crate::stats`] — Defines the stat types and parsing logic used here.

use std::io::BufReader;

use crate::container::{ContainerDMetaDataProvider, ContainerMeta, PodMeta};
use crate::stats::{CollectedStats, KeyValueStat, SingleLineStat};

mod utils;
#[cfg(feature = "cgroup-v2")]
pub mod v2;

/// A trait for recursively scanning a root path in the cgroup hierarchy
/// to discover running container slices.
pub trait ContainerScanner {
    /// Recursively scans the given root path and registers any discovered containers
    /// into the provided `Monitor`.
    ///
    /// # Arguments
    ///
    /// * `path` - The cgroup root path to search for containers.
    /// * `monitor` - A mutable reference to a `Monitor` instance to register discovered containers with.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if reading or accessing the cgroup filesystem fails.
    fn scan_path(
        &self,
        path: &std::path::Path,
        monitor: &mut Monitor,
        containerd_meta_provider: &mut ContainerDMetaDataProvider,
    ) -> impl std::future::Future<Output = std::io::Result<()>> + Send;
}

/// Represents a discovered container and its runtime context, i.e., process ids and optionally an
/// associated pod id.
#[derive(Debug)]
pub enum ContainerSlice {
    /// A standalone container (e.g. docker, podman).
    Standalone {
        container_id: crate::container::ContainerID,
        pids: Vec<u32>,
        monitor: ContainerMonitor,
        metadata: Option<ContainerMeta>,
    },
    /// A container that is part of a pod, identified by a `PodID`.
    Pod {
        container_id: crate::container::ContainerID,
        pod_id: crate::container::PodID,
        pids: Vec<u32>,
        monitor: ContainerMonitor,
        container_metadata: Option<ContainerMeta>,
        pod_metadata: Option<PodMeta>,
    },
}

impl ContainerSlice {
    /// Constructs a `ContainerSlice::Standalone` variant.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The unique identifier for the container.
    /// * `pids` - A list of process IDs associated with the container.
    /// * `path` - Path to the container’s cgroup directory.
    ///
    ///  # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// # use creo_monitor::container::ContainerID;
    /// # use creo_monitor::cgroup::ContainerSlice;
    /// let id = ContainerID::new(*b"abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd").unwrap();
    /// let pids = vec![1234, 5678];
    /// let slice = ContainerSlice::new_standalone(id, pids, Path::new("/sys/fs/cgroup/my-container"), None);
    /// ```
    pub fn new_standalone(
        container_id: crate::container::ContainerID,
        pids: Vec<u32>,
        path: impl AsRef<std::path::Path>,
        metadata: Option<ContainerMeta>,
    ) -> Self {
        let monitor = ContainerMonitor::from_cgroup(path, &pids);
        Self::Standalone {
            container_id,
            pids,
            monitor,
            metadata,
        }
    }

    /// Constructs a `ContainerSlice::Pod` variant.
    ///
    /// # Arguments
    ///
    /// * `container_id` - The container's unique identifier.
    /// * `pod_id` - The pod identifier to which the container belongs.
    /// * `pids` - A list of process IDs associated with the container.
    /// * `path` - Path to the container’s cgroup directory.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// # use creo_monitor::container::{ContainerID, PodID};
    /// # use creo_monitor::cgroup::ContainerSlice;
    /// let cid = ContainerID::new(*b"abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd").unwrap();
    /// let pid = PodID::new(*b"abc123abc123abc123abc123abc123ab").unwrap();
    /// let pids = vec![1234, 5678];
    /// let slice = ContainerSlice::new_pod(cid, pid, pids, Path::new("/sys/fs/cgroup/my-pod-container"), None, None);
    /// ```
    pub fn new_pod(
        container_id: crate::container::ContainerID,
        pod_id: crate::container::PodID,
        pids: Vec<u32>,
        path: impl AsRef<std::path::Path>,
        container_metadata: Option<ContainerMeta>,
        pod_metadata: Option<PodMeta>,
    ) -> Self {
        let monitor = ContainerMonitor::from_cgroup(path, &pids);
        Self::Pod {
            container_id,
            pod_id,
            pids,
            monitor,
            container_metadata,
            pod_metadata,
        }
    }

    /// Returns the container ID associated with this slice.
    ///
    /// # Returns
    ///
    /// A reference to the container’s `ContainerID`.
    pub fn container_id(&self) -> &crate::container::ContainerID {
        match self {
            Self::Standalone { container_id, .. } => container_id,
            Self::Pod { container_id, .. } => container_id,
        }
    }

    /// Returns a reference to the list of PIDs associated with the container.
    ///
    /// # Returns
    ///
    /// A slice of process IDs (`&[u32]`).
    pub fn pids(&self) -> &[u32] {
        match self {
            Self::Standalone { pids, .. } => pids,
            Self::Pod { pids, .. } => pids,
        }
    }
}

/// Monitors resource usage for a single container using cgroup and procfs data.
#[derive(Debug)]
pub struct ContainerMonitor {
    cpu_stat_file: Option<BufReader<std::fs::File>>,
    cpu_limit_file: Option<BufReader<std::fs::File>>,
    memory_stat_file: Option<BufReader<std::fs::File>>,
    memory_usage_file: Option<BufReader<std::fs::File>>,
    memory_limit_file: Option<BufReader<std::fs::File>>,
    io_stat_file: Option<BufReader<std::fs::File>>,
    network_stat_files: Vec<BufReader<std::fs::File>>,
}

impl ContainerMonitor {
    /// Creates a new `ContainerMonitor` for the specified cgroup path and associated PIDs.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the container’s cgroup directory.
    /// * `pids` - A list of process IDs used to locate network statistics in `/proc/<pid>/net/dev`.
    ///
    /// # Returns
    ///
    /// A new `ContainerMonitor` instance.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::path::Path;
    /// # use creo_monitor::cgroup::ContainerMonitor;
    /// let pids = vec![1234];
    /// let monitor = ContainerMonitor::from_cgroup(Path::new("/sys/fs/cgroup/test"), &pids);
    /// ```
    pub fn from_cgroup(path: impl AsRef<std::path::Path>, pids: &[u32]) -> Self {
        let path = path.as_ref();
        Self {
            cpu_stat_file: Self::open_file(path.join("cpu.stat")),
            cpu_limit_file: Self::open_file(path.join("cpu.max")),
            memory_stat_file: Self::open_file(path.join("memory.stat")),
            memory_usage_file: Self::open_file(path.join("memory.current")),
            memory_limit_file: Self::open_file(path.join("memory.max")),
            io_stat_file: Self::open_file(path.join("io.stat")),
            network_stat_files: pids
                .iter()
                .filter_map(|p| {
                    Self::open_file(std::path::PathBuf::from_iter(&[
                        crate::ROOT,
                        &p.to_string(),
                        "net",
                        "dev",
                    ]))
                })
                .collect(),
        }
    }

    /// Collects and returns resource usage statistics for the container.
    ///
    /// # Returns
    ///
    /// A `ContainerStats` object representing the latest usage metrics.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if reading from any stat file fails.
    pub fn refresh_stats(&mut self) -> std::io::Result<super::stats::ContainerStats> {
        let cpu_stat = utils::read_and_rewind(
            self.cpu_stat_file.as_mut(),
            crate::stats::CpuStat::from_reader,
        )?;

        let cpu_limit = utils::read_and_rewind(
            self.cpu_limit_file.as_mut(),
            crate::stats::CpuLimit::from_reader,
        )?;
        let memory_stat = utils::read_and_rewind(
            self.memory_stat_file.as_mut(),
            crate::stats::MemoryStat::from_reader,
        )?;
        let memory_usage = utils::read_and_rewind(
            self.memory_usage_file.as_mut(),
            crate::stats::MemoryUsage::from_reader,
        )?;
        let memory_limit = utils::read_and_rewind(
            self.memory_limit_file.as_mut(),
            crate::stats::MemoryLimit::from_reader,
        )?;
        let io_stat = utils::read_and_rewind(
            self.io_stat_file.as_mut(),
            crate::stats::IoStat::from_reader,
        )?;
        let network_stat = utils::read_all_and_rewind(
            self.network_stat_files.as_mut(),
            crate::stats::NetworkStat::from_reader,
        )?;
        Ok(crate::stats::ContainerStats::new(
            cpu_stat,
            cpu_limit,
            memory_stat,
            memory_usage,
            memory_limit,
            io_stat,
            network_stat,
        ))
    }

    #[inline]
    fn open_file(path: impl AsRef<std::path::Path>) -> Option<BufReader<std::fs::File>> {
        Some(BufReader::new(std::fs::File::open(path).ok()?))
    }
}

/// Aggregates container stats over time and tracks their lifecycle.
#[derive(Debug, Default)]
pub struct Monitor {
    containers: std::collections::HashMap<std::path::PathBuf, ContainerSlice>,
    collected: Vec<super::stats::CollectedStats>,
}

impl Monitor {
    /// Registers a new container at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the container’s cgroup directory.
    /// * `container` - A `ContainerSlice` to be tracked.
    pub fn register_container(
        &mut self,
        path: impl AsRef<std::path::Path>,
        container: ContainerSlice,
    ) {
        let path = path.as_ref();
        self.containers.insert(path.to_path_buf(), container);
    }

    /// Returns `true` if a container is already registered at the given path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the container’s cgroup directory.
    ///
    /// # Returns
    ///
    /// `true` if a container is registered, `false` otherwise.
    pub fn is_tracking_path(&self, path: impl AsRef<std::path::Path>) -> bool {
        self.containers.contains_key(path.as_ref())
    }

    /// Collects stats for all registered containers and removes any that are stale.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - A timestamp (e.g., UNIX time) to associate with collected metrics.
    pub fn collect_stats(&mut self, timestamp: u64) {
        self.containers.retain(|path, container| {
            if Self::is_stale(path) {
                log::debug!(
                    "Removing stale container ({}) at `{}`",
                    container.container_id().as_str(),
                    path.display()
                );
                return false;
            }

            let result = match container {
                ContainerSlice::Standalone {
                    monitor,
                    container_id,
                    metadata,
                    ..
                } => monitor.refresh_stats().map(|stats| {
                    crate::stats::CollectedStats::new_standalone(
                        timestamp,
                        *container_id,
                        stats,
                        metadata.clone(),
                    )
                }),
                ContainerSlice::Pod {
                    monitor,
                    container_id,
                    pod_id,
                    container_metadata,
                    pod_metadata,
                    ..
                } => monitor.refresh_stats().map(|stats| {
                    crate::stats::CollectedStats::new_pod(
                        timestamp,
                        *container_id,
                        *pod_id,
                        stats,
                        container_metadata.clone(),
                        pod_metadata.clone(),
                    )
                }),
            };

            match result {
                Ok(metric) => {
                    self.collected.push(metric);
                    true
                }
                Err(err) => {
                    log::error!(
                        target: "container monitor",
                        "Failed reading stats: container_id={}, path={}, error={}",
                        container.container_id(),
                        path.display(),
                        err
                    );
                    false
                }
            }
        });
    }

    /// Determines whether a cgroup path no longer exists and should be cleaned up.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to check for staleness (i.e., filesystem existence).
    ///
    /// # Returns
    ///
    /// `true` if the path no longer exists, `false` otherwise.
    fn is_stale(path: impl AsRef<std::path::Path>) -> bool {
        !path.as_ref().exists()
    }

    /// Returns and clears all collected container statistics.
    ///
    /// # Returns
    ///
    /// A vector of `CollectedStats` for the last collection cycle.
    pub fn drain_stats(&mut self) -> Vec<CollectedStats> {
        std::mem::take(&mut self.collected)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContainerRuntime {
    Docker,
    ContainerD,
    Podman,
}
