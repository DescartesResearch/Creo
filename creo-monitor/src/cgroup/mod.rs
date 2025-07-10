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
//! - [`ContainerSlice`] — A variant enum distinguishing standalone vs. pod-scoped containers.
//! - [`CgroupMonitor`] — Maintains stat file handles and extracts runtime metrics.
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
//! # Platform Requirements
//!
//! - Linux with cgroup v2 support.
//! - Read access to `/sys/fs/cgroup` and `/proc/<pid>/net/dev`.
mod collector;
mod container;
mod monitor;
pub mod stats;
mod utils;

pub use collector::{Collector, CollectorBuilder};
pub use container::MonitoredContainer;
pub use monitor::Monitor;
