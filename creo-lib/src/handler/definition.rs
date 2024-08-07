use std::collections::HashMap;

use crate::service_types::{ResourceType, Utilization};

use super::Function;

#[derive(Clone, Debug)]
pub struct Definition {
    pub directory: std::path::PathBuf,
    pub utilization: HashMap<ResourceType, f64>,
}

impl Definition {
    pub fn new<P: AsRef<std::path::Path>>(handler_dir: P, utilization: Utilization) -> Self {
        Self {
            directory: handler_dir.as_ref().to_path_buf(),
            utilization: utilization.0,
        }
    }
}

impl TryInto<Function> for Definition {
    type Error = std::io::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        crate::io::parse_handler_function(self.directory)
    }
}

impl TryInto<Function> for &Definition {
    type Error = std::io::Error;

    fn try_into(self) -> Result<Function, Self::Error> {
        crate::io::parse_handler_function(&self.directory)
    }
}
