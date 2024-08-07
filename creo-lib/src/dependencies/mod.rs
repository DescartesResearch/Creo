use docker_compose_types as dct;
use std::str::FromStr;

mod database;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DependencyType {
    Database(database::DatabaseDependency),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Deserialize)]
pub struct Dependency {
    pub name: DependencyType,
    #[serde(default)]
    pub init: Option<String>,
}

use DependencyType::*;

impl DependencyType {
    pub(crate) fn as_docker_compose_service(
        &self,
        service_name: impl AsRef<str>,
    ) -> (String, dct::Service) {
        match self {
            Database(db) => db.as_docker_compose_service(service_name),
        }
    }

    pub fn as_service_name(&self, service_name: impl AsRef<str>) -> String {
        match self {
            Database(db) => db.as_service_name(service_name),
        }
    }

    pub(crate) fn as_docker_compose_environment(
        &self,
        service_name: impl AsRef<str>,
    ) -> Vec<String> {
        match self {
            Database(db) => db.as_docker_compose_environment(service_name),
        }
    }

    pub(crate) fn as_volume_name(&self, service_name: impl AsRef<str>) -> Option<String> {
        match self {
            Database(db) => db.as_volume_name(service_name),
        }
    }
}

impl FromStr for DependencyType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_once(":") {
            Some(("db", name)) => {
                let db = database::DatabaseDependency::from_str(name)?;
                Ok(Self::Database(db))
            }
            _ => Err(format!("unknown dependency name {}", s)),
        }
    }
}

impl<'de> serde::Deserialize<'de> for DependencyType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        FromStr::from_str(&s).map_err(serde::de::Error::custom)
    }
}
