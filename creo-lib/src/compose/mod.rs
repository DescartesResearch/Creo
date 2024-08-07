mod application;
mod service;
use std::io::Read;

pub use crate::de::FromYamlReader;

pub use application::create_application_compose;
pub use service::{create_service_compose_with_build, create_service_compose_with_image};

pub struct Compose(pub docker_compose_types::Compose);

impl From<docker_compose_types::Compose> for Compose {
    fn from(value: docker_compose_types::Compose) -> Self {
        Self(value)
    }
}

impl Compose {
    pub fn from_reader<R: Read>(r: R) -> std::io::Result<Self> {
        let dc = docker_compose_types::Compose::from_yaml_reader(r)
            .map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err))?;
        Ok(Self(dc))
    }
}
