#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("failed to execute ssh command: {0}")]
    SSH(#[from] async_ssh2_tokio::Error),
    #[error("remote file system error: {0}")]
    RemoteFS(#[from] russh_sftp::client::error::Error),
    #[error("failed to establish connection: {0}")]
    Connection(String),
    #[error("failed to obtain an authentication method: {0}")]
    AuthMethod(String),
    #[error("local file system error: {0}")]
    LocalFileSystem(#[from] std::io::Error),
    #[error("failed to obtain config: {0}")]
    Config(#[from] serde_yaml::Error),
    #[error("invalid argument: {0}")]
    InvalidArgument(String),
    #[error("failed to render script template: {0}")]
    RenderScript(#[from] handlebars::RenderError),
    #[error("unstable benchmarks: {0}")]
    UnstableBenchmark(String),
}

pub type Result<T> = std::result::Result<T, Error>;
