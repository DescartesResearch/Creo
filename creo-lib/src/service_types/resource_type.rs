use std::fmt;

#[derive(serde::Deserialize, Clone, PartialEq, Eq, Hash, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ResourceType {
    Cpu,
    Memory,
    NetworkReceive,
    NetworkTransmit,
    DiskRead,
    DiskWrite,
}

use ResourceType::*;

impl fmt::Display for ResourceType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Cpu => "CPU",
            Memory => "MEMORY",
            NetworkReceive => "NETWORK_RECEIVE",
            NetworkTransmit => "NETWORK_TRANSMIT",
            DiskRead => "DISK_READ",
            DiskWrite => "DISK_WRITE",
        };
        f.write_str(s)
    }
}
