#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to discover containers in `{ROOT}`: {0}", ROOT=crate::CGROUP_ROOT)]
    DiscoverContainersError(#[source] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultLogExt<T, E> {
    fn ok_log(self) -> Option<T>;
}

impl<T, E> ResultLogExt<T, E> for std::result::Result<T, E>
where
    E: std::error::Error,
{
    fn ok_log(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                log::error!("{err}");
                None
            }
        }
    }
}
