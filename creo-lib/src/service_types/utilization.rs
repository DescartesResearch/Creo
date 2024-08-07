use std::collections::HashMap;

use super::ResourceType;

#[derive(serde::Deserialize, Debug)]
pub struct Utilization(pub HashMap<ResourceType, f64>);
