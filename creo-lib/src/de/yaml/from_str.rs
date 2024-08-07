pub trait FromYamlStr<'de>
where
    Self: Sized,
{
    fn from_yaml_str(str: &'de str) -> Result<Self, serde_yaml::Error>;
}

impl<'de, T> FromYamlStr<'de> for T
where
    T: serde::de::Deserialize<'de>,
{
    fn from_yaml_str(str: &'de str) -> Result<T, serde_yaml::Error> {
        let model: T = serde_yaml::from_str(str)?;

        Ok(model)
    }
}
