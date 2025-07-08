#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid container id: {0}")]
    InvalidContainerID(String),
    #[error("invalid pod id: {0}")]
    InvalidPodID(String),
    #[error("invalid machine id: {0}")]
    InvalidMachineID(String),
}
pub type Result<T> = std::result::Result<T, Error>;
