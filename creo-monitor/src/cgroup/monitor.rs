use dashmap::DashMap;

use crate::container::ContainerID;

use super::container::MonitoredContainer;
use super::stats::ContainerStatsEntry;

/// Aggregates container stats over time and tracks their lifecycle.
#[derive(Debug, Default)]
pub struct Monitor {
    containers: DashMap<ContainerID, MonitoredContainer>,
}

impl Monitor {
    /// Registers a new container at the specified path.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the container’s cgroup directory.
    /// * `container` - A `ContainerSlice` to be tracked.
    pub fn register_container(&self, container_id: ContainerID, container: MonitoredContainer) {
        self.containers.insert(container_id, container);
    }

    pub fn remove_container(&self, container_id: &ContainerID) {
        self.containers.remove(container_id);
    }

    /// Collects stats for all registered containers and removes any that are stale.
    ///
    /// # Arguments
    ///
    /// * `timestamp` - A timestamp (e.g., UNIX time) to associate with collected metrics.
    pub fn collect_stats(&self, timestamp: u64, out: &mut Vec<ContainerStatsEntry>) {
        self.containers.retain(|container_id, container| {
            match container
                .collector()
                .refresh_stats()
                .map(|stats| ContainerStatsEntry::new(timestamp, *container_id, stats))
            {
                Ok(metric) => {
                    out.push(metric);
                    true
                }
                Err(err) => {
                    log::error!(
                        target: "container monitor",
                        "failed reading container stats: container_id={}, error={}",
                        container_id,
                        err
                    );
                    false
                }
            }
        });
    }

    pub fn size(&self) -> usize {
        self.containers.len()
    }
}
