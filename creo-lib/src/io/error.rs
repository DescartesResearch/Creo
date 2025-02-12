#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("invalid yaml file format: {0}")]
    ParsingYaml(#[from] serde_yaml::Error),
    #[error("invalid json file format: {0}")]
    ParsingJson(#[from] serde_json::Error),
}

impl From<Error> for std::io::Error {
    fn from(value: Error) -> Self {
        std::io::Error::new(std::io::ErrorKind::InvalidData, value)
    }
}
