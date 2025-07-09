use sqlx::MySqlPool;

use super::models::MachineID;
use super::{Error, Result, StatsPersister, models};

#[derive(Debug, Clone)]
pub struct MySqlStatsPersister {
    db: MySqlPool,
    machine_id: MachineID,
}

impl MySqlStatsPersister {
    pub fn new(db: MySqlPool, machine_id: crate::container::MachineID) -> Self {
        Self {
            db,
            machine_id: machine_id.into(),
        }
    }
}

impl StatsPersister for MySqlStatsPersister {
    /// Inserts a list of collected container or pod statistics into the database.
    ///
    /// This function wraps the insertions in a single transaction. If any insert fails,
    /// the entire transaction is rolled back. It supports both standalone container stats
    /// and stats collected from pods.
    ///
    /// # Arguments
    ///
    /// * `collected_stats` - A slice of `CollectedStats` representing container/pod statistics
    ///   collected at a point in time.
    ///
    /// # Errors
    ///
    /// Returns an `Error::InsertError` if the database transaction or any insert query fails.
    async fn persist_stats(
        &self,
        stats: &[crate::cgroup::stats::ContainerStatsEntry],
    ) -> Result<()> {
        const INSERT_QUERY: &str = r#"
INSERT INTO container_stats (
    timestamp, container_id, machine_id,
    cpu_usage_usec, cpu_user_usec, cpu_system_usec,
    cpu_nr_periods, cpu_nr_throttled, cpu_throttled_usec,
    cpu_nr_bursts, cpu_burst_usec,
    cpu_quota, cpu_period,
    memory_anon, memory_file, memory_kernel_stack, memory_slab,
    memory_sock, memory_shmem, memory_file_mapped,
    memory_usage_bytes,
    memory_limit_bytes,
    io_rbytes, io_wbytes, io_rios, io_wios,
    net_rx_bytes, net_rx_packets, net_tx_bytes, net_tx_packets
) VALUES (
    ?, ?, ?,
    ?, ?, ?,
    ?, ?, ?,
    ?, ?,
    ?, ?,
    ?, ?, ?, ?,
    ?, ?, ?,
    ?,
    ?,
    ?, ?, ?, ?,
    ?, ?, ?, ?
)
"#;
        let mut tx: sqlx::Transaction<'_, sqlx::MySql> =
            self.db.begin().await.map_err(Error::InsertError)?;

        for stat in stats {
            let flat_stat: models::ContainerStats = (self.machine_id, stat).into();

            let query = sqlx::query(INSERT_QUERY);
            let query = flat_stat.bind_all(query);
            query.execute(&mut *tx).await.map_err(Error::InsertError)?;
        }
        tx.commit().await.map_err(Error::InsertError)?;

        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct MySqlMetadataPersister {
    db: MySqlPool,
    machine_id: MachineID,
    hostname: String,
}

impl MySqlMetadataPersister {
    // TODO: maybe split host metadata such as hostname into own table?
    pub fn new(db: MySqlPool, machine_id: crate::container::MachineID, hostname: String) -> Self {
        Self {
            db,
            machine_id: machine_id.into(),
            hostname,
        }
    }
}

impl super::MetadataPersister for MySqlMetadataPersister {
    async fn persist_metadata(
        &self,
        (container_id, labels): (
            crate::container::ContainerID,
            std::collections::HashMap<String, String>,
        ),
    ) -> Result<()> {
        const INSERT_QUERY: &str = r#"
INSERT INTO container_metadata (
    container_id, machine_id, hostname, label_key, label_value
) VALUES (
    ?, ?, ?, ?, ?
)
ON DUPLICATE KEY UPDATE
    label_value = VALUES(label_value)
"#;
        let mut tx: sqlx::Transaction<'_, sqlx::MySql> =
            self.db.begin().await.map_err(Error::InsertError)?;

        for (key, value) in labels {
            let query = sqlx::query(INSERT_QUERY);
            let c_id: super::models::ContainerID = container_id.into();
            let query = query
                .bind(c_id.as_slice())
                .bind(self.machine_id.as_slice())
                .bind(&self.hostname)
                .bind(key)
                .bind(value);
            query.execute(&mut *tx).await.map_err(Error::InsertError)?;
        }
        tx.commit().await.map_err(Error::InsertError)?;

        Ok(())
    }
}
