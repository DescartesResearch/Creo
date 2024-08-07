use std::hash::Hash;

use super::{ResourceIntensity, ResourceType};

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(remote = "Self")]
pub struct Resource {
    pub resource: ResourceType,
    pub fraction: u8,
    pub intensity: ResourceIntensity,
}

impl<'de> serde::Deserialize<'de> for Resource {
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

impl PartialEq for Resource {
    fn eq(&self, other: &Self) -> bool {
        self.resource == other.resource
    }
}

impl Eq for Resource {}

impl Hash for Resource {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.resource.hash(state);
    }
}
