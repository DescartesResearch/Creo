use crate::container::ContainerID;

use super::collector::Collector;

/// Represents a discovered container and its runtime context, i.e., process ids.
#[derive(Debug)]
pub struct MonitoredContainer {
    container_id: ContainerID,
    pids: Vec<u32>,
    collector: Collector,
}

impl MonitoredContainer {
    /// Constructs a [`MonitoredContainer`].
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
    /// # use creo_monitor::container::ContainerID;
    /// # use creo_monitor::cgroup::{MonitoredContainer, CollectorBuilder};
    /// let id = ContainerID::new(*b"abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd").unwrap();
    /// let pids = vec![1234, 5678];
    /// let monitor = CollectorBuilder::default().build();
    /// let slice = MonitoredContainer::new(id, pids, monitor);
    /// ```
    pub fn new(
        container_id: crate::container::ContainerID,
        pids: Vec<u32>,
        collector: Collector,
    ) -> Self {
        Self {
            container_id,
            pids,
            collector,
        }
    }

    /// Returns the container ID associated with this slice.
    ///
    /// # Returns
    ///
    /// A reference to the container’s `ContainerID`.
    pub fn container_id(&self) -> &crate::container::ContainerID {
        &self.container_id
    }

    /// Returns a reference to the list of PIDs associated with the container.
    ///
    /// # Returns
    ///
    /// A slice of process IDs (`&[u32]`).
    pub fn pids(&self) -> &[u32] {
        self.pids.as_slice()
    }

    pub fn collector(&mut self) -> &mut Collector {
        &mut self.collector
    }
}
