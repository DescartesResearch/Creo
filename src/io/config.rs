use creo_lib::de::FromYamlReader;
use std::path::Path;

use crate::{Error, Result};

pub fn parse_config<T: serde::de::DeserializeOwned + Send>(path: impl AsRef<Path>) -> Result<T> {
    let file = std::fs::File::open(path)?;
    T::from_yaml_reader(file).map_err(|err| Error::new(format!("failed to parse config: {}", err)))
}
