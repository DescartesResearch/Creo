//! This module provides core types and utilities for collecting and representing container resource usage statistics.
//!
//! It aggregates various cgroup-based stats modules such as CPU, memory, I/O, and network statistics,
//! and exposes a unified [`ContainerStats`] struct to represent a complete snapshot of container resource usage.
//!
//! # Main types
//!
//! - [`CollectedStats`]: Enum representing statistics collected for either a standalone container or a pod,
//!   including container and optional pod identifiers with a timestamp.
//! - [`ContainerStats`]: Struct aggregating multiple resource usage metrics collected from cgroup files,
//!   including CPU stats and limits, memory stats, I/O stats, and network stats.
//!
//! # Usage
//!
//! Typically, instances of [`ContainerStats`] are created by parsing various cgroup files for a container.
//! These stats can then be wrapped in [`CollectedStats`] to associate them with container and pod metadata
//! along with a timestamp for collection time.
//!

mod cpu;
mod error;
mod io;
mod memory;
mod net;
mod parser;

pub use cpu::{CpuLimit, CpuStat};
pub use error::StatParseError;
pub use io::IoStat;
pub use memory::{MemoryLimit, MemoryStat, MemoryUsage};
pub use net::NetworkStat;
pub use parser::{KeyValueStat, SingleLineStat};

use crate::container::ContainerID;

#[derive(Debug, Clone)]
pub struct ContainerStatsEntry {
    /// Timestamp (in UNIX epoch seconds)
    timestamp: u64,
    container_id: ContainerID,
    stats: CgroupStats,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing container")]
    MissingContainerError,
}

impl ContainerStatsEntry {
    pub fn new(timestamp: u64, container_id: ContainerID, stats: CgroupStats) -> Self {
        Self {
            timestamp,
            container_id,
            stats,
        }
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn container_id(&self) -> ContainerID {
        self.container_id
    }

    pub fn stats(&self) -> &CgroupStats {
        &self.stats
    }
}

/// Represents a full set of resource usage stats for a container, collected from cgroup files.
#[derive(Debug, Clone)]
pub struct CgroupStats {
    /// CPU usage statistics from `cpu.stat`.
    cpu_stat: Option<CpuStat>,
    /// CPU limits from `cpu.max`.
    cpu_limit: Option<CpuLimit>,
    /// Memory usage statistics from `memory.stat`.
    memory_stat: Option<MemoryStat>,
    /// Memory usage statistics from `memory.current`.
    memory_usage: Option<MemoryUsage>,
    /// Memory limit from `memory.max`.
    memory_limit: Option<MemoryLimit>,
    /// Block I/O usage statistics from `io.stat`.
    io_stat: Option<IoStat>,
    /// Network usage statistics from `/proc/<pid>/net/dev`.
    network_stat: Option<NetworkStat>,
}

impl CgroupStats {
    pub fn new(
        cpu_stat: Option<CpuStat>,
        cpu_limit: Option<CpuLimit>,
        memory_stat: Option<MemoryStat>,
        memory_usage: Option<MemoryUsage>,
        memory_limit: Option<MemoryLimit>,
        io_stat: Option<IoStat>,
        network_stat: Option<NetworkStat>,
    ) -> Self {
        Self {
            cpu_stat,
            cpu_limit,
            memory_stat,
            memory_usage,
            memory_limit,
            io_stat,
            network_stat,
        }
    }

    /// Returns CPU usage statistics from `cpu.stat`.
    pub fn cpu_stat(&self) -> Option<&CpuStat> {
        self.cpu_stat.as_ref()
    }

    /// Returns memory usage statistics from `memory.stat`.
    pub fn memory_stat(&self) -> Option<&MemoryStat> {
        self.memory_stat.as_ref()
    }

    /// Returns the memory usage statistics from `memory.current`.
    pub fn memory_usage(&self) -> Option<&MemoryUsage> {
        self.memory_usage.as_ref()
    }

    /// Returns block I/O statistics from `io.stat`.
    pub fn io_stat(&self) -> Option<&IoStat> {
        self.io_stat.as_ref()
    }

    /// Returns network statistics from `/proc/<pid>/net/dev`.
    pub fn network_stat(&self) -> Option<&NetworkStat> {
        self.network_stat.as_ref()
    }

    /// Returns the CPU limits from `cpu.max`.
    pub fn cpu_limit(&self) -> Option<&CpuLimit> {
        self.cpu_limit.as_ref()
    }

    /// Returns the memory limit from `memory.max`.
    pub fn memory_limit(&self) -> Option<&MemoryLimit> {
        self.memory_limit.as_ref()
    }
}
