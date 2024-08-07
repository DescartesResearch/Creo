use std::io::Read;

pub trait FromYamlReader<'de>
where
    Self: Sized,
{
    fn from_yaml_reader<R: Read>(rdr: R) -> Result<Self, serde_yaml::Error>;
}

impl<'de, T> FromYamlReader<'de> for T
where
    T: serde::de::DeserializeOwned,
{
    fn from_yaml_reader<R: Read>(rdr: R) -> Result<Self, serde_yaml::Error> {
        let model: T = serde_yaml::from_reader(rdr)?;

        Ok(model)
    }
}
