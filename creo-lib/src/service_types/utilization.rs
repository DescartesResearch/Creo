use std::collections::HashMap;

use super::Label;

#[derive(serde::Deserialize, Debug)]
pub struct Utilization(pub HashMap<Label, f64>);
