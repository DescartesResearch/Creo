use std::io::BufRead;

/// Represents network statistics for a single interface, as reported in `/proc/net/dev`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct NetworkStat {
    /// Bytes received.
    pub rx_bytes: u64,
    /// Packets received.
    pub rx_packets: u64,
    /// Receive errors.
    pub rx_errs: u64,
    /// Dropped packets while receiving.
    pub rx_drop: u64,
    /// FIFO buffer errors while receiving.
    pub rx_fifo: u64,
    /// Frame alignment errors while receiving.
    pub rx_frame: u64,
    /// Compressed packets received.
    pub rx_compressed: u64,
    /// Multicast packets received.
    pub rx_multicast: u64,

    /// Bytes transmitted.
    pub tx_bytes: u64,
    /// Packets transmitted.
    pub tx_packets: u64,
    /// Transmit errors.
    pub tx_errs: u64,
    /// Dropped packets while transmitting.
    pub tx_drop: u64,
    /// FIFO buffer errors while transmitting.
    pub tx_fifo: u64,
    /// Collisions detected while transmitting.
    pub tx_colls: u64,
    /// Carrier loss errors while transmitting.
    pub tx_carrier: u64,
    /// Compressed packets transmitted.
    pub tx_compressed: u64,
}

impl std::ops::AddAssign for NetworkStat {
    fn add_assign(&mut self, rhs: Self) {
        self.rx_bytes += rhs.rx_bytes;
        self.rx_packets += rhs.rx_packets;
        self.rx_errs += rhs.rx_errs;
        self.rx_drop += rhs.rx_drop;
        self.rx_fifo += rhs.rx_fifo;
        self.rx_frame += rhs.rx_frame;
        self.rx_compressed += rhs.rx_compressed;
        self.rx_multicast += rhs.rx_multicast;
        self.tx_bytes += rhs.tx_bytes;
        self.tx_packets += rhs.tx_packets;
        self.tx_errs += rhs.tx_errs;
        self.tx_drop += rhs.tx_drop;
        self.tx_fifo += rhs.tx_fifo;
        self.tx_colls += rhs.tx_colls;
        self.tx_carrier += rhs.tx_carrier;
        self.tx_compressed += rhs.tx_compressed;
    }
}

const IGNORED_INTERFACES: [&str; 4] = ["lo", "veth", "docker", "nerdctl"];

/// Parses a single line of network interface data from `/proc/net/dev`.
///
/// # Arguments
///
/// * `line` - A string slice representing the line with interface stats.
///
/// # Returns
///
/// Returns `Some((iface, fields))` if the line contains an interface and its data,
/// or `None` if the format is invalid. `iface` is the interface name (e.g., "eth0"),
/// and `fields` is a vector of whitespace-separated data values.
fn parse_interface_line(line: &str) -> Option<(&str, impl Iterator<Item = &str>)> {
    let (iface, data) = line.trim().split_once(':')?;
    Some((iface, data.split_whitespace()))
}

/// Determines whether a network interface should be ignored based on its name.
///
/// # Arguments
///
/// * `iface` - The name of the network interface (e.g., "lo", "eth0").
///
/// # Returns
///
/// Returns `true` if the interface matches any prefix in `IGNORED_INTERFACES`,
/// meaning it should be excluded from statistics collection.
fn is_ignored_interface(iface: &str) -> bool {
    IGNORED_INTERFACES
        .iter()
        .any(|prefix| iface.starts_with(prefix))
}

/// Parses network interface statistics from an iterator of string fields.
///
/// Extracts the receive/transmit byte and packet counters from the first 16
/// fields of a `/proc/net/dev` line (after the interface name).
///
/// # Arguments
///
/// * `fields` - An iterator over whitespace-separated field strings representing
///   a line of interface data. The line must contain at least 16 fields to be valid.
///
/// # Returns
///
/// Returns `Some(NetworkStat)` if at least 16 fields are present and parsed,
/// or `None` if there are too few fields.
fn stats_from_fields<'a>(mut fields: impl Iterator<Item = &'a str>) -> Option<NetworkStat> {
    Some(NetworkStat {
        rx_bytes: fields.next()?.parse().unwrap_or(0),
        rx_packets: fields.next()?.parse().unwrap_or(0),
        rx_errs: fields.next()?.parse().unwrap_or(0),
        rx_drop: fields.next()?.parse().unwrap_or(0),
        rx_fifo: fields.next()?.parse().unwrap_or(0),
        rx_frame: fields.next()?.parse().unwrap_or(0),
        rx_compressed: fields.next()?.parse().unwrap_or(0),
        rx_multicast: fields.next()?.parse().unwrap_or(0),
        tx_bytes: fields.next()?.parse().unwrap_or(0),
        tx_packets: fields.next()?.parse().unwrap_or(0),
        tx_errs: fields.next()?.parse().unwrap_or(0),
        tx_drop: fields.next()?.parse().unwrap_or(0),
        tx_fifo: fields.next()?.parse().unwrap_or(0),
        tx_colls: fields.next()?.parse().unwrap_or(0),
        tx_carrier: fields.next()?.parse().unwrap_or(0),
        tx_compressed: fields.next()?.parse().unwrap_or(0),
    })
}

impl NetworkStat {
    /// Constructs a `NetworkStat` by reading and parsing network statistics
    /// from a reader implementing `std::io::Read` (e.g., a file or buffer).
    ///
    /// This method is intended for parsing files like `/proc/net/dev` on Linux,
    /// where each line represents a network interface with its traffic stats.
    ///
    /// # Arguments
    ///
    /// * `reader` - A mutable reference to an object implementing `std::io::Read`,
    ///   from which the interface statistics will be read.
    ///
    /// # Returns
    ///
    /// Returns `Ok(NetworkStat)` with accumulated statistics if parsing succeeds,
    /// or an `Err(std::io::Error)` if reading from the input fails.
    pub fn from_reader<R: BufRead>(buf: &mut R) -> std::io::Result<Self> {
        let mut stat = NetworkStat::default();
        let mut line = String::new();

        // Skip headers (first two lines)
        for _ in 0..2 {
            buf.read_line(&mut line)?;
            line.clear();
        }

        while buf.read_line(&mut line)? != 0 {
            if let Some((iface, fields)) = parse_interface_line(&line) {
                if !is_ignored_interface(iface) {
                    if let Some(s) = stats_from_fields(fields) {
                        stat += s;
                    };
                }
            }
            line.clear();
        }

        Ok(stat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_input() {
        let data = b"";
        let stat = NetworkStat::from_reader(&mut &data[..]).unwrap();
        assert_eq!(stat, NetworkStat::default());
    }

    #[test]
    fn test_only_headers() {
        let data = b"\
Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
";
        let stat = NetworkStat::from_reader(&mut &data[..]).unwrap();
        assert_eq!(stat, NetworkStat::default());
    }

    #[test]
    fn test_parse_complete_network_stat() {
        let data = b"\
Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo: 422198341   75815    0    0    0     0          0         0 422198341   75815    0    0    0     0       0          0
  eth0: 10240    100     0    0    0     0          0         0  20480   200     0    0    0     0       0          0
";
        let stat = NetworkStat::from_reader(&mut &data[..]).unwrap();
        assert_eq!(stat.rx_bytes, 10240);
        assert_eq!(stat.rx_packets, 100);
        assert_eq!(stat.rx_errs, 0);
        assert_eq!(stat.rx_drop, 0);
        assert_eq!(stat.rx_fifo, 0);
        assert_eq!(stat.rx_frame, 0);
        assert_eq!(stat.rx_compressed, 0);
        assert_eq!(stat.rx_multicast, 0);
        assert_eq!(stat.tx_bytes, 20480);
        assert_eq!(stat.tx_packets, 200);
        assert_eq!(stat.tx_errs, 0);
        assert_eq!(stat.tx_drop, 0);
        assert_eq!(stat.tx_fifo, 0);
        assert_eq!(stat.tx_colls, 0);
        assert_eq!(stat.tx_carrier, 0);
        assert_eq!(stat.tx_compressed, 0);
    }

    #[test]
    fn test_malformed_line_too_few_fields() {
        let data = b"\
Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
 badif: 123 456
";
        let stat = NetworkStat::from_reader(&mut &data[..]).unwrap();
        assert_eq!(stat, NetworkStat::default());
    }

    #[test]
    fn test_ignored_interface() {
        let data = b"\
Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
    lo: 999 999 0 0 0 0 0 0 999 999 0 0 0 0 0 0
    docker0: 999 999 0 0 0 0 0 0 999 999 0 0 0 0 0 0
    veth0: 999 999 0 0 0 0 0 0 999 999 0 0 0 0 0 0
    nerdctl0: 999 999 0 0 0 0 0 0 999 999 0 0 0 0 0 0
";
        let stat = NetworkStat::from_reader(&mut &data[..]).unwrap();
        assert_eq!(stat, NetworkStat::default());
    }

    #[test]
    fn test_unparsable_values() {
        let data = b"\
Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
  eth0: xyz abc 0 0 0 0 0 0  20480 200 0 0 0 0 0 0
";
        let stat = NetworkStat::from_reader(&mut &data[..]).unwrap();
        assert_eq!(stat.rx_bytes, 0);
        assert_eq!(stat.rx_packets, 0);
        assert_eq!(stat.tx_bytes, 20480);
        assert_eq!(stat.tx_packets, 200);
    }

    #[test]
    fn test_multiple_valid_interfaces() {
        let data = b"\
Inter-|   Receive                                                |  Transmit
 face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed
  eth0: 100 200 0 0 0 0 0 0  300 400 0 0 0 0 0 0
  eth1: 10 20 0 0 0 0 0 0  30 40 0 0 0 0 0 0
";
        let stat = NetworkStat::from_reader(&mut &data[..]).unwrap();
        assert_eq!(stat.rx_bytes, 110);
        assert_eq!(stat.rx_packets, 220);
        assert_eq!(stat.tx_bytes, 330);
        assert_eq!(stat.tx_packets, 440);
    }
}
