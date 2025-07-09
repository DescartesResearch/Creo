use std::collections::HashMap;

use crate::container::ContainerID;

use super::Result;

pub trait StatsPersister {
    fn persist_stats(
        &self,
        stats: &[crate::cgroup::stats::ContainerStatsEntry],
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}

pub trait MetadataPersister {
    fn persist_metadata(
        &self,
        metadata: (ContainerID, HashMap<String, String>),
    ) -> impl std::future::Future<Output = Result<()>> + Send;
}
