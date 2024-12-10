#[derive(
    Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[serde(remote = "Self")]
pub struct NonEmptyString(String);

impl<'de> serde::Deserialize<'de> for NonEmptyString {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = String::deserialize(deserializer)?;

        if this.is_empty() {
            return Err(serde::de::Error::custom("string must not be empty"));
        }

        Ok(NonEmptyString(this))
    }
}

impl From<NonEmptyString> for String {
    fn from(value: NonEmptyString) -> Self {
        value.0
    }
}

impl AsRef<str> for NonEmptyString {
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}
