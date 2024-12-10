use std::{fmt::Display, hash::Hash};

use super::{Bucket, Label};

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(remote = "Self")]
pub struct Property {
    pub label: Label,
    pub fraction: u8,
    pub bucket: Bucket,
}

impl<'de> serde::Deserialize<'de> for Property {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        if this.fraction < 1 || this.fraction > 100 {
            return Err(serde::de::Error::custom(format!(
                "expected fraction to be in the range of 1..=100, but was {}",
                this.fraction
            )));
        }

        Ok(this)
    }
}

impl PartialEq for Property {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl Eq for Property {}

impl Hash for Property {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.label.hash(state);
    }
}

impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}, {}%)", self.label, self.bucket, self.fraction)
    }
}
