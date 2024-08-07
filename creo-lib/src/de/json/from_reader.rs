use std::io::Read;

pub trait FromJsonReader<'de>
where
    Self: Sized,
{
    fn from_json_reader<R: Read>(rdr: R) -> Result<Self, serde_json::Error>;
}

impl<'de, T> FromJsonReader<'de> for T
where
    T: serde::de::DeserializeOwned,
{
    fn from_json_reader<R: Read>(rdr: R) -> Result<Self, serde_json::Error> {
        let model: T = serde_json::from_reader(rdr)?;

        Ok(model)
    }
}
