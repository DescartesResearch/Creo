#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid container id: {0}")]
    InvalidContainerID(String),
    #[error("invalid pod id: {0}")]
    InvalidPodID(String),
    #[error("failed to connect to socket '{socket}': {source}")]
    SocketConnectError {
        socket: String,
        #[source]
        source: tonic::transport::Error,
    },
    #[error("failed to request container information from containerd socket: {0}")]
    ContainerDRequestError(#[source] tonic::Status),
}
pub type Result<T> = std::result::Result<T, Error>;
