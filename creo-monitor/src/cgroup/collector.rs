use super::stats::{CgroupStats, KeyValueStat, SingleLineStat};
use std::fs::File;
use std::path::PathBuf;

use super::utils;

/// Monitors resource usage for a single container using cgroup and procfs data.
#[derive(Debug)]
pub struct Collector {
    cpu_stat_file_path: Option<PathBuf>,
    cpu_limit_file_path: Option<PathBuf>,
    memory_stat_file_path: Option<PathBuf>,
    memory_usage_file_path: Option<PathBuf>,
    memory_limit_file_path: Option<PathBuf>,
    io_stat_file_path: Option<PathBuf>,
    network_stat_file_paths: Vec<PathBuf>,
    network_stat_files_cache: Vec<File>,
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
        let cpu_stat = utils::read(
            self.cpu_stat_file_path
                .as_ref()
                .and_then(utils::open_file)
                .as_mut(),
            |buf| super::stats::CpuStat::from_reader(buf),
        )?;

        let cpu_limit = utils::read(
            self.cpu_limit_file_path
                .as_ref()
                .and_then(utils::open_file)
                .as_mut(),
            |buf| super::stats::CpuLimit::from_reader(buf),
        )?;
        let memory_stat = utils::read(
            self.memory_stat_file_path
                .as_ref()
                .and_then(utils::open_file)
                .as_mut(),
            |buf| super::stats::MemoryStat::from_reader(buf),
        )?;
        let memory_usage = utils::read(
            self.memory_usage_file_path
                .as_ref()
                .and_then(utils::open_file)
                .as_mut(),
            |buf| super::stats::MemoryUsage::from_reader(buf),
        )?;
        let memory_limit = utils::read(
            self.memory_limit_file_path
                .as_ref()
                .and_then(utils::open_file)
                .as_mut(),
            |buf| super::stats::MemoryLimit::from_reader(buf),
        )?;
        let io_stat = utils::read(
            self.io_stat_file_path
                .as_ref()
                .and_then(utils::open_file)
                .as_mut(),
            |buf| super::stats::IoStat::from_reader(buf),
        )?;
        self.network_stat_files_cache.extend(
            self.network_stat_file_paths
                .iter()
                .flat_map(utils::open_file),
        );
        let network_stat = utils::read_all(&mut self.network_stat_files_cache, |buf| {
            super::stats::NetworkStat::from_reader(buf)
        })?;
        self.network_stat_files_cache.clear();
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
    cpu_stat_file_path: Option<PathBuf>,
    cpu_limit_file_path: Option<PathBuf>,
    memory_stat_file_path: Option<PathBuf>,
    memory_usage_file_path: Option<PathBuf>,
    memory_limit_file_path: Option<PathBuf>,
    io_stat_file_path: Option<PathBuf>,
    network_stat_file_paths: Vec<PathBuf>,
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
        self.cpu_stat_file_path = Some(path.as_ref().to_path_buf());
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
        self.cpu_limit_file_path = Some(path.as_ref().to_path_buf());
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
        self.memory_stat_file_path = Some(path.as_ref().to_path_buf());
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
        self.memory_usage_file_path = Some(path.as_ref().to_path_buf());
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
        self.memory_limit_file_path = Some(path.as_ref().to_path_buf());
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
        self.io_stat_file_path = Some(path.as_ref().to_path_buf());
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
        self.network_stat_file_paths = paths.iter().map(|p| p.as_ref().to_path_buf()).collect();
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
        let cap = self.network_stat_file_paths.len();
        Collector {
            cpu_stat_file_path: self.cpu_stat_file_path,
            cpu_limit_file_path: self.cpu_limit_file_path,
            memory_stat_file_path: self.memory_stat_file_path,
            memory_usage_file_path: self.memory_usage_file_path,
            memory_limit_file_path: self.memory_limit_file_path,
            io_stat_file_path: self.io_stat_file_path,
            network_stat_file_paths: self.network_stat_file_paths,
            network_stat_files_cache: Vec::with_capacity(cap),
        }
    }
}
