use creo_lib::de::FromYamlReader;
use std::path::Path;

use crate::{Error, Result};

pub fn open_file(path: impl AsRef<Path>) -> Result<std::fs::File> {
    std::fs::File::open(path).map_err(|err| Error::new(format!("{}", err)))
}

pub fn parse_config<T: serde::de::DeserializeOwned>(path: impl AsRef<Path>) -> Result<T> {
    let file = open_file(path)?;
    T::from_yaml_reader(file).map_err(|err| Error::new(format!("failed to parse config: {}", err)))
}
