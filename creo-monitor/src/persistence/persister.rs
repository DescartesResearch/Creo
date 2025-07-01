use super::{Result, models};

pub trait Persister {
    fn persist_stats(
        &self,
        stats: &[crate::stats::CollectedStats],
    ) -> impl std::future::Future<Output = Result<()>> + Send;

    fn query_stats_by_time_range(
        &self,
        from: u64,
        to: u64,
    ) -> impl std::future::Future<Output = Result<Vec<models::ContainerStats>>> + Send;
}
