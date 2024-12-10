use std::{collections::HashSet, fmt::Display};

use super::Property;

#[derive(serde::Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(remote = "Self")]
pub struct ServiceType {
    pub fraction: u8,
    pub properties: Vec<Property>,
}

impl Display for ServiceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Service Type ({:?}, {}%)",
            self.properties, self.fraction
        )
    }
}

impl<'de> serde::Deserialize<'de> for ServiceType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;

        if this.properties.is_empty() {
            return Err(serde::de::Error::custom("resources list must be non empty"));
        }

        let sum: u8 = this
            .properties
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
        for resource in &this.properties {
            if !seen.insert(resource) {
                return Err(serde::de::Error::custom(format!(
                    "expected resources to be unique, but found duplicate resource {}",
                    resource.label
                )));
            }
        }

        Ok(this)
    }
}

#[derive(serde::Deserialize, Debug)]
pub struct ServiceTypeVec(pub crate::de::NonEmptyVec<ServiceType>);
