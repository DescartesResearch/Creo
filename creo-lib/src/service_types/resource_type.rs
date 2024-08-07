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
        match *self {
            Cpu => f.write_str("CPU"),
            Memory => f.write_str("MEMORY"),
            NetworkReceive => f.write_str("NETWORK_RECEIVE"),
            NetworkTransmit => f.write_str("NETWORK_TRANSMIT"),
            DiskRead => f.write_str("DISK_READ"),
            DiskWrite => f.write_str("DISK_WRITE"),
        }
    }
}
