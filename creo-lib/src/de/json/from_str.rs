pub trait FromJsonStr<'de>
where
    Self: Sized,
{
    fn from_json_str(str: &'de str) -> Result<Self, serde_json::Error>;
}

impl<'de, T> FromJsonStr<'de> for T
where
    T: serde::de::Deserialize<'de>,
{
    fn from_json_str(str: &'de str) -> Result<Self, serde_json::Error> {
        let model: T = serde_json::from_str(str)?;

        Ok(model)
    }
}
