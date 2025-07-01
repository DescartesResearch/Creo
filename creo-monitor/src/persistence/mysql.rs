use sqlx::{MySqlPool, mysql::MySqlPoolOptions};

use super::{Error, Persister, Result, models};

#[derive(Debug, Clone)]
pub struct MySqlPersister {
    db: MySqlPool,
}

impl MySqlPersister {
    pub async fn new(url: &str) -> Result<Self> {
        let db = MySqlPoolOptions::new()
            .acquire_timeout(std::time::Duration::from_secs(10))
            .max_connections(10)
            .connect(url)
            .await
            .map_err(Error::ConnectionError)?;

        sqlx::migrate!()
            .run(&db)
            .await
            .map_err(Error::MigrationError)?;

        Ok(Self { db })
    }
}

impl Persister for MySqlPersister {
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
    async fn persist_stats(&self, stats: &[crate::stats::CollectedStats]) -> Result<()> {
        const INSERT_QUERY: &str = r#"
INSERT INTO container_stats (
    timestamp, container_id, container_name, pod_id, pod_name, pod_namespace,
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
    ?, ?, ?, ?, ?, ?,
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
            let flat_stat: models::ContainerStats = stat.into();

            let query = sqlx::query(INSERT_QUERY);
            let query = flat_stat.bind_all(query);
            query.execute(&mut *tx).await.map_err(Error::InsertError)?;
        }
        tx.commit().await.map_err(Error::InsertError)?;

        Ok(())
    }

    async fn query_stats_by_time_range(
        &self,
        from: u64,
        to: u64,
    ) -> Result<Vec<models::ContainerStats>> {
        let stats = sqlx::query_as::<_, models::ContainerStats>(
            r#"
            SELECT * FROM container_stats WHERE timestamp BETWEEN ? and ? ORDER BY timestamp
        "#,
        )
        .bind(from)
        .bind(to)
        .fetch_all(&self.db)
        .await
        .map_err(Error::ReadError)?;

        Ok(stats)
    }
}
