use std::collections::HashSet;

use super::Resource;

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(remote = "Self")]
pub struct ServiceType {
    pub fraction: u8,
    pub resources: Vec<Resource>,
}

impl<'de> serde::Deserialize<'de> for ServiceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        if this.resources.is_empty() {
            return Err(serde::de::Error::custom("resources list must be non empty"));
        }

        let sum: u8 = this
            .resources
            .iter()
            .map(|resource| resource.fraction)
            .sum();
        if sum != 100 {
            return Err(serde::de::Error::custom(format!(
                "expected fractions of resources to sum up to 100%, but sum was {}%",
                sum
            )));
        }
        let mut seen = HashSet::new();
        for resource in &this.resources {
            if !seen.insert(resource) {
                return Err(serde::de::Error::custom(format!(
                    "expected resources to be unique, but found duplicate resource {}",
                    resource.resource
                )));
            }
        }

        Ok(this)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ServiceTypeVec(pub Vec<ServiceType>);
