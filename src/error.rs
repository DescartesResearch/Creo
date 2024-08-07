use std::error;

#[derive(Debug)]
pub struct Error(String);

impl Error {
    pub fn new(msg: String) -> Self {
        Self(msg)
    }
    pub fn with_log<E: error::Error>(msg: String, err: E) -> Self {
        log::debug!("Error: {}", err);
        Self(msg)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Error {}", self.0)
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl<E: std::error::Error> From<E> for Error {
    fn from(value: E) -> Self {
        Self::new(value.to_string())
    }
}
