use docker_compose_types as dct;
use std::str::FromStr;

mod mongo;

#[derive(serde::Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum DatabaseDependency {
    Mongo,
}
use DatabaseDependency::*;

impl DatabaseDependency {
    pub(crate) fn as_docker_compose_service(
        &self,
        service_name: impl AsRef<str>,
    ) -> (String, dct::Service) {
        match self {
            Mongo => mongo::MongoDB::as_docker_compose_service(service_name),
        }
    }

    pub(crate) fn as_docker_compose_environment(
        &self,
        service_name: impl AsRef<str>,
    ) -> Vec<String> {
        match self {
            Mongo => mongo::MongoDB::as_docker_compose_environment(service_name),
        }
    }

    pub(crate) fn as_volume_name(&self, service_name: impl AsRef<str>) -> Option<String> {
        match self {
            Mongo => Some(mongo::MongoDB::as_volume_name(service_name)),
        }
    }

    pub(crate) fn as_service_name(&self, service_name: impl AsRef<str>) -> String {
        match self {
            Mongo => mongo::MongoDB::as_service_name(service_name),
        }
    }
}

impl FromStr for DatabaseDependency {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mongo" => Ok(Self::Mongo),
            _ => Err(format!("unknown database dependency {}", s)),
        }
    }
}
