#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to connect to database: {0}")]
    ConnectionError(#[source] sqlx::Error),
    #[error("failed to run initial migration: {0}")]
    MigrationError(#[source] sqlx::migrate::MigrateError),
    #[error("failed to setup database connection: {0}")]
    SetupError(#[source] sqlx::Error),
    #[error("failed to insert stats: {0}")]
    InsertError(#[source] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
