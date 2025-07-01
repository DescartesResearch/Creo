#[derive(Debug, Clone, sqlx::FromRow, serde::Serialize)]
pub struct ContainerStats {
    pub timestamp: u64,
    pub container_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub container_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pod_namespace: Option<String>,
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
            .bind(&self.container_id)
            .bind(&self.container_name)
            .bind(&self.pod_id)
            .bind(&self.pod_name)
            .bind(&self.pod_namespace)
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

impl From<&crate::stats::CollectedStats> for ContainerStats {
    fn from(value: &crate::stats::CollectedStats) -> Self {
        let (timestamp, container_id, pod_id, stats, container_meta, pod_meta) = match value {
            crate::stats::CollectedStats::Standalone {
                timestamp,
                container_id,
                stats: metrics,
                metadata,
            } => (*timestamp, container_id, None, metrics, metadata, &None),
            crate::stats::CollectedStats::Pod {
                timestamp,
                container_id,
                pod_id,
                stats: metrics,
                container_metadata,
                pod_metadata,
            } => (
                *timestamp,
                container_id,
                Some(pod_id),
                metrics,
                container_metadata,
                pod_metadata,
            ),
        };

        let cpu_stat = stats.cpu_stat();
        let cpu_limit = stats.cpu_limit();
        let memory_stat = stats.memory_stat();
        let memory_usage = stats.memory_usage();
        let memory_limit = stats.memory_limit();
        let io_stat = stats.io_stat();
        let net_stat = stats.network_stat();

        Self {
            timestamp,
            container_id: container_id.to_string(),
            container_name: container_meta
                .as_ref()
                .and_then(|m| m.name().map(|n| n.to_owned())),
            pod_id: pod_id.map(|p| p.to_string()),
            pod_name: pod_meta
                .as_ref()
                .and_then(|m| m.name().map(|n| n.to_owned())),
            pod_namespace: pod_meta
                .as_ref()
                .and_then(|m| m.namespace().map(|n| n.to_owned())),
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
