use super::stats::{CgroupStats, KeyValueStat, SingleLineStat};
use std::fs::File;
use std::io::BufReader;

use super::utils;

/// Monitors resource usage for a single container using cgroup and procfs data.
#[derive(Debug)]
pub struct Collector {
    cpu_stat_file: Option<BufReader<File>>,
    cpu_limit_file: Option<BufReader<File>>,
    memory_stat_file: Option<BufReader<File>>,
    memory_usage_file: Option<BufReader<File>>,
    memory_limit_file: Option<BufReader<File>>,
    io_stat_file: Option<BufReader<File>>,
    network_stat_files: Vec<BufReader<File>>,
}

impl Collector {
    /// Collects and returns resource usage statistics for the container.
    ///
    /// # Returns
    ///
    /// A `ContainerStats` object representing the latest usage metrics.
    ///
    /// # Errors
    ///
    /// Returns an I/O error if reading from any stat file fails.
    pub fn refresh_stats(&mut self) -> std::io::Result<CgroupStats> {
        let cpu_stat = utils::read_and_rewind(
            self.cpu_stat_file.as_mut(),
            super::stats::CpuStat::from_reader,
        )?;

        let cpu_limit = utils::read_and_rewind(
            self.cpu_limit_file.as_mut(),
            super::stats::CpuLimit::from_reader,
        )?;
        let memory_stat = utils::read_and_rewind(
            self.memory_stat_file.as_mut(),
            super::stats::MemoryStat::from_reader,
        )?;
        let memory_usage = utils::read_and_rewind(
            self.memory_usage_file.as_mut(),
            super::stats::MemoryUsage::from_reader,
        )?;
        let memory_limit = utils::read_and_rewind(
            self.memory_limit_file.as_mut(),
            super::stats::MemoryLimit::from_reader,
        )?;
        let io_stat = utils::read_and_rewind(
            self.io_stat_file.as_mut(),
            super::stats::IoStat::from_reader,
        )?;
        let network_stat = utils::read_all_and_rewind(
            self.network_stat_files.as_mut(),
            super::stats::NetworkStat::from_reader,
        )?;
        Ok(super::stats::CgroupStats::new(
            cpu_stat,
            cpu_limit,
            memory_stat,
            memory_usage,
            memory_limit,
            io_stat,
            network_stat,
        ))
    }
}

#[derive(Debug, Default)]
pub struct CollectorBuilder {
    cpu_stat_file: Option<BufReader<File>>,
    cpu_limit_file: Option<BufReader<File>>,
    memory_stat_file: Option<BufReader<File>>,
    memory_usage_file: Option<BufReader<File>>,
    memory_limit_file: Option<BufReader<File>>,
    io_stat_file: Option<BufReader<File>>,
    network_stat_files: Vec<BufReader<File>>,
}

impl CollectorBuilder {
    /// Sets the path to the `cpu.stat` file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the CPU statistics file (usually from cgroup v2).
    ///
    /// # Returns
    ///
    /// The builder with the `cpu_stat_file` set.
    pub fn set_cpu_stat_file(&mut self, path: impl AsRef<std::path::Path>) -> &mut Self {
        self.cpu_stat_file = utils::open_file(path);
        self
    }

    /// Sets the path to the CPU limit file (e.g., `cpu.max`).
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the CPU limit configuration file.
    ///
    /// # Returns
    ///
    /// The builder with the `cpu_limit_file` set.
    pub fn set_cpu_limit_file(&mut self, path: impl AsRef<std::path::Path>) -> &mut Self {
        self.cpu_limit_file = utils::open_file(path);
        self
    }

    /// Sets the path to the memory statistics file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the memory.stat file (from cgroup v2).
    ///
    /// # Returns
    ///
    /// The builder with the `memory_stat_file` set.
    pub fn set_memory_stat_file(&mut self, path: impl AsRef<std::path::Path>) -> &mut Self {
        self.memory_stat_file = utils::open_file(path);
        self
    }

    /// Sets the path to the current memory usage file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the memory usage file (e.g., `memory.current`).
    ///
    /// # Returns
    ///
    /// The builder with the `memory_usage_file` set.
    pub fn set_memory_usage_file(&mut self, path: impl AsRef<std::path::Path>) -> &mut Self {
        self.memory_usage_file = utils::open_file(path);
        self
    }

    /// Sets the path to the memory limit file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the memory limit file (e.g., `memory.max`).
    ///
    /// # Returns
    ///
    /// The builder with the `memory_limit_file` set.
    pub fn set_memory_limit_file(&mut self, path: impl AsRef<std::path::Path>) -> &mut Self {
        self.memory_limit_file = utils::open_file(path);
        self
    }

    /// Sets the path to the I/O statistics file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the I/O statistics file (e.g., `io.stat`).
    ///
    /// # Returns
    ///
    /// The builder with the `io_stat_file` set.
    pub fn set_io_stat_file(&mut self, path: impl AsRef<std::path::Path>) -> &mut Self {
        self.io_stat_file = utils::open_file(path);
        self
    }

    /// Sets one or more paths to network statistics files (e.g., `/proc/net/dev`).
    ///
    /// # Arguments
    ///
    /// * `paths` - A slice of paths to network statistics files.
    ///
    /// # Returns
    ///
    /// The builder with the `network_stat_files` vector populated.
    pub fn set_network_stat_files(&mut self, paths: &[impl AsRef<std::path::Path>]) -> &mut Self {
        self.network_stat_files = paths.iter().filter_map(utils::open_file).collect();
        self
    }

    /// Builds the `ContainerMonitor` from the provided paths.
    ///
    /// Any fields not explicitly set will be `None` or empty, depending on the type.
    ///
    /// # Returns
    ///
    /// A fully constructed `ContainerMonitor`.
    pub fn build(self) -> Collector {
        Collector {
            cpu_stat_file: self.cpu_stat_file,
            cpu_limit_file: self.cpu_limit_file,
            memory_stat_file: self.memory_stat_file,
            memory_usage_file: self.memory_usage_file,
            memory_limit_file: self.memory_limit_file,
            io_stat_file: self.io_stat_file,
            network_stat_files: self.network_stat_files,
        }
    }
}
