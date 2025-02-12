pub mod application;
pub mod compose;
pub(crate) mod constants;
pub mod de;
mod dependencies;
pub mod generator;
pub mod graph;
pub mod handler;
mod http_method;
pub mod io;
mod load;
pub mod metrics;
pub mod programming_language;
pub mod remote;
pub mod schema;
pub mod selection;
mod service_types;
pub mod ssh;
pub mod stats;
pub mod template;

pub use service_types::ServiceTypeVec;

pub const VERSION: &str = "1.0.0";
pub use constants::*;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid port number: expected a port number between {min} and {max}, but got {got}")]
    InvalidPortNumber { got: u32, min: u32, max: u32 },
}

#[derive(Debug, Clone, Copy, serde::Deserialize)]
#[serde(remote = "Self")]
pub struct Port(u32);

impl Port {
    const MIN: u32 = 30000;
    const MAX: u32 = 49151;

    pub fn new(port: u32) -> Result<Self, Error> {
        if !(Self::MIN..=Self::MAX).contains(&port) {
            return Err(Error::InvalidPortNumber {
                got: port,
                min: Self::MIN,
                max: Self::MAX,
            });
        }
        Ok(Self(port))
    }
}

impl Default for Port {
    fn default() -> Self {
        Self(30100)
    }
}

impl From<Port> for u32 {
    fn from(val: Port) -> Self {
        val.0
    }
}

impl<'de> serde::Deserialize<'de> for Port {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let port = u32::deserialize(deserializer)?;
        let this = Self::new(port).map_err(serde::de::Error::custom)?;
        Ok(this)
    }
}

impl serde::Serialize for Port {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        u32::serialize(&self.0, serializer)
    }
}
