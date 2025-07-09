use sqlx::{
    Decode, Type,
    error::BoxDynError,
    mysql::{MySql, MySqlTypeInfo, MySqlValueRef},
};

use crate::container;

#[derive(Debug, Clone, serde::Serialize, Copy)]
pub struct MachineID(pub [u8; 16]);

impl MachineID {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<MachineID> for String {
    fn from(value: MachineID) -> Self {
        let mut s = String::with_capacity(32);
        for byte in value.0 {
            use std::fmt::Write;
            write!(s, "{:02x}", byte).expect("write!() into String to never fail");
        }
        s
    }
}

impl From<container::MachineID> for MachineID {
    fn from(value: container::MachineID) -> Self {
        Self(value.as_raw())
    }
}

impl Type<MySql> for MachineID {
    fn type_info() -> MySqlTypeInfo {
        <&[u8] as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <Vec<u8> as Type<MySql>>::compatible(ty)
    }
}

impl<'r> Decode<'r, MySql> for MachineID {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let slice = <&'r [u8] as Decode<MySql>>::decode(value)?;
        let id_bytes: [u8; 16] = slice
            .try_into()
            .map_err(|_| "Invalid length for MachineId")?;
        Ok(MachineID(id_bytes))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ContainerID(pub [u8; 64]);

impl ContainerID {
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }
}

impl From<ContainerID> for String {
    fn from(value: ContainerID) -> Self {
        String::from_utf8_lossy(value.as_slice()).to_string()
    }
}

impl From<container::ContainerID> for ContainerID {
    fn from(value: container::ContainerID) -> Self {
        Self(value.as_raw())
    }
}

impl Type<MySql> for ContainerID {
    fn type_info() -> MySqlTypeInfo {
        <&[u8] as Type<MySql>>::type_info()
    }

    fn compatible(ty: &MySqlTypeInfo) -> bool {
        <Vec<u8> as Type<MySql>>::compatible(ty)
    }
}

impl<'r> Decode<'r, MySql> for ContainerID {
    fn decode(value: MySqlValueRef<'r>) -> Result<Self, BoxDynError> {
        let slice = <&'r [u8] as Decode<MySql>>::decode(value)?;
        let id_bytes: [u8; 64] = slice
            .try_into()
            .map_err(|_| "Invalid length for MachineId")?;
        Ok(ContainerID(id_bytes))
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ContainerStats {
    pub timestamp: u64,
    pub container_id: ContainerID,
    pub machine_id: MachineID,
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

impl ContainerStats {
    pub fn bind_all<'q>(
        &'q self,
        query: sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments>,
    ) -> sqlx::query::Query<'q, sqlx::MySql, sqlx::mysql::MySqlArguments> {
        query
            .bind(self.timestamp)
            .bind(self.container_id.as_slice())
            .bind(self.machine_id.as_slice())
            .bind(self.cpu_usage_usec)
            .bind(self.cpu_user_usec)
            .bind(self.cpu_system_usec)
            .bind(self.cpu_nr_periods)
            .bind(self.cpu_nr_throttled)
            .bind(self.cpu_throttled_usec)
            .bind(self.cpu_nr_bursts)
            .bind(self.cpu_burst_usec)
            .bind(self.cpu_quota)
            .bind(self.cpu_period)
            .bind(self.memory_anon)
            .bind(self.memory_file)
            .bind(self.memory_kernel_stack)
            .bind(self.memory_slab)
            .bind(self.memory_sock)
            .bind(self.memory_shmem)
            .bind(self.memory_file_mapped)
            .bind(self.memory_usage_bytes)
            .bind(self.memory_limit_bytes)
            .bind(self.io_rbytes)
            .bind(self.io_wbytes)
            .bind(self.io_rios)
            .bind(self.io_wios)
            .bind(self.net_rx_bytes)
            .bind(self.net_rx_packets)
            .bind(self.net_tx_bytes)
            .bind(self.net_tx_packets)
    }
}

impl From<(MachineID, &crate::cgroup::stats::ContainerStatsEntry)> for ContainerStats {
    fn from(
        (machine_id, stats_entry): (MachineID, &crate::cgroup::stats::ContainerStatsEntry),
    ) -> Self {
        let stats = stats_entry.stats();
        let cpu_stat = stats.cpu_stat();
        let cpu_limit = stats.cpu_limit();
        let memory_stat = stats.memory_stat();
        let memory_usage = stats.memory_usage();
        let memory_limit = stats.memory_limit();
        let io_stat = stats.io_stat();
        let net_stat = stats.network_stat();

        Self {
            timestamp: stats_entry.timestamp(),
            container_id: stats_entry.container_id().into(),
            machine_id,
            cpu_usage_usec: cpu_stat.map(|c| c.usage_usec),
            cpu_user_usec: cpu_stat.map(|c| c.user_usec),
            cpu_system_usec: cpu_stat.map(|c| c.system_usec),
            cpu_nr_periods: cpu_stat.map(|c| c.nr_periods),
            cpu_nr_throttled: cpu_stat.map(|c| c.nr_throttled),
            cpu_throttled_usec: cpu_stat.map(|c| c.throttled_usec),
            cpu_nr_bursts: cpu_stat.map(|c| c.nr_bursts),
            cpu_burst_usec: cpu_stat.map(|c| c.burst_usec),
            cpu_quota: cpu_limit.and_then(|c| c.quota),
            cpu_period: cpu_limit.map(|c| c.period),
            memory_anon: memory_stat.map(|m| m.anon),
            memory_file: memory_stat.map(|m| m.file),
            memory_kernel_stack: memory_stat.map(|m| m.kernel_stack),
            memory_slab: memory_stat.map(|m| m.slab),
            memory_sock: memory_stat.map(|m| m.sock),
            memory_shmem: memory_stat.map(|m| m.shmem),
            memory_file_mapped: memory_stat.map(|m| m.file_mapped),
            memory_usage_bytes: memory_usage.map(|m| m.usage_bytes),
            memory_limit_bytes: memory_limit.and_then(|m| m.limit_bytes),
            io_rbytes: io_stat.map(|i| i.rbytes),
            io_wbytes: io_stat.map(|i| i.wbytes),
            io_rios: io_stat.map(|i| i.rios),
            io_wios: io_stat.map(|i| i.wios),
            net_rx_bytes: net_stat.map(|n| n.rx_bytes),
            net_rx_packets: net_stat.map(|n| n.rx_packets),
            net_tx_bytes: net_stat.map(|n| n.tx_bytes),
            net_tx_packets: net_stat.map(|n| n.tx_packets),
        }
    }
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct ContainerMetadata {
    pub container_id: ContainerID,
    pub machine_id: MachineID,
    pub hostname: String,
    pub label_key: String,
    pub label_value: String,
}
