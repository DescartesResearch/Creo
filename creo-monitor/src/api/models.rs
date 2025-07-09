use std::collections::HashMap;

use crate::persistence;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ContainerIdentifier {
    pub container_id: String,
    pub machine_id: String,
}

impl serde::Serialize for ContainerIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let s = format!("{}:{}", self.container_id, self.machine_id);
        serializer.serialize_str(&s)
    }
}

impl ContainerIdentifier {
    pub fn new(container_id: String, machine_id: String) -> Self {
        Self {
            container_id,
            machine_id,
        }
    }
}

#[derive(Debug, serde::Serialize)]
pub struct ContainerStats {
    pub timestamp: u64,
    pub cpu_usage_usec: Option<u64>,
    pub cpu_user_usec: Option<u64>,
    pub cpu_system_usec: Option<u64>,
    pub cpu_nr_periods: Option<u64>,
    pub cpu_nr_throttled: Option<u64>,
    pub cpu_throttled_usec: Option<u64>,
    pub cpu_nr_bursts: Option<u64>,
    pub cpu_burst_usec: Option<u64>,
    pub cpu_quota: Option<u64>,
    pub cpu_period: Option<u64>,
    pub memory_anon: Option<u64>,
    pub memory_file: Option<u64>,
    pub memory_kernel_stack: Option<u64>,
    pub memory_slab: Option<u64>,
    pub memory_sock: Option<u64>,
    pub memory_shmem: Option<u64>,
    pub memory_file_mapped: Option<u64>,
    pub memory_usage_bytes: Option<u64>,
    pub memory_limit_bytes: Option<u64>,
    pub io_rbytes: Option<u64>,
    pub io_wbytes: Option<u64>,
    pub io_rios: Option<u64>,
    pub io_wios: Option<u64>,
    pub net_rx_bytes: Option<u64>,
    pub net_rx_packets: Option<u64>,
    pub net_tx_bytes: Option<u64>,
    pub net_tx_packets: Option<u64>,
}

impl From<persistence::ContainerStats> for ContainerStats {
    fn from(value: persistence::ContainerStats) -> Self {
        Self {
            timestamp: value.timestamp,
            cpu_usage_usec: value.cpu_usage_usec,
            cpu_user_usec: value.cpu_user_usec,
            cpu_system_usec: value.cpu_system_usec,
            cpu_nr_periods: value.cpu_nr_periods,
            cpu_nr_throttled: value.cpu_nr_throttled,
            cpu_throttled_usec: value.cpu_throttled_usec,
            cpu_nr_bursts: value.cpu_nr_bursts,
            cpu_burst_usec: value.cpu_burst_usec,
            cpu_quota: value.cpu_quota,
            cpu_period: value.cpu_period,
            memory_anon: value.memory_anon,
            memory_file: value.memory_file,
            memory_kernel_stack: value.memory_kernel_stack,
            memory_slab: value.memory_slab,
            memory_sock: value.memory_sock,
            memory_shmem: value.memory_shmem,
            memory_file_mapped: value.memory_file_mapped,
            memory_usage_bytes: value.memory_usage_bytes,
            memory_limit_bytes: value.memory_limit_bytes,
            io_rbytes: value.io_rbytes,
            io_wbytes: value.io_wbytes,
            io_rios: value.io_rios,
            io_wios: value.io_wios,
            net_rx_bytes: value.net_rx_bytes,
            net_rx_packets: value.net_rx_packets,
            net_tx_bytes: value.net_tx_bytes,
            net_tx_packets: value.net_tx_packets,
        }
    }
}

#[derive(Debug, Default, serde::Serialize)]
pub struct ContainerMetadata {
    pub hostname: String,
    pub labels: HashMap<String, String>,
}
