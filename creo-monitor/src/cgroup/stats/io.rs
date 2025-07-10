//! This module provides parsing utilities for I/O statistics as reported in Linux cgroup `io.stat` files.
//!
//! It supports parsing of multi-device I/O statistics, where each line typically corresponds to
//! a single block device and contains multiple key-value pairs representing read/write byte counts
//! and operation counts.
//!
//! # Key features
//!
//! - **Aggregation across devices:** The parser sums statistics from all devices reported in the
//!   `io.stat` file, producing a single aggregated [`IoStat`] structure.
//! - **Flexible parsing:** Each line can contain multiple key-value pairs separated by whitespace,
//!   with key-value pairs themselves using `=` as a delimiter.
//! - **Robust error handling:** Invalid key-value pairs or values result in clear parse errors,
//!   while unknown keys and malformed pairs are ignored gracefully.
//!
//! # Parsing assumptions
//!
//! - Each line starts with a device identifier (e.g., `8:0`), followed by one or more
//!   whitespace-separated key-value pairs like `rbytes=1024`.
//! - Duplicate keys across devices are accumulated (added) rather than overwritten.
//! - Unknown keys or malformed pairs are ignored rather than causing a failure.
//!
//! # Example
//!
//! ```rust
//! use std::io::BufReader;
//! use creo_monitor::cgroup::stats::{IoStat, KeyValueStat};
//!
//! let data = "\
//! 8:0 rbytes=1024 wbytes=2048 rios=12 wios=24
//! 254:0 rbytes=1024 wbytes=2048 rios=12 wios=24
//! ";
//! let mut reader = BufReader::new(data.as_bytes());
//! let io_stat = IoStat::from_reader(&mut reader).unwrap();
//!
//! assert_eq!(io_stat.rbytes, 2048);
//! assert_eq!(io_stat.wbytes, 4096);
//! assert_eq!(io_stat.rios, 24);
//! assert_eq!(io_stat.wios, 48);
//! ```

use std::collections::HashMap;
use std::sync::LazyLock;

use super::parser::KeyValueStat;

/// Represents aggregated I/O statistics collected from the Linux `io.stat` file
/// in the cgroup filesystem. Fields are summed across all devices present in the file.
///
/// This struct is typically populated using [`IoStat::from_reader`].
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct IoStat {
    /// Total number of bytes read across all devices.
    pub rbytes: u64,
    /// Total number of bytes written across all devices.
    pub wbytes: u64,
    /// Total number of read operations across all devices.
    pub rios: u64,
    /// Total number of write operations across all devices.
    pub wios: u64,
}

impl IoStat {
    /// Adds to the `rbytes` field.
    fn add_rbytes(&mut self, rbytes: u64) {
        self.rbytes += rbytes;
    }

    /// Adds to the `wbytes` field.
    fn add_wbytes(&mut self, wbytes: u64) {
        self.wbytes += wbytes;
    }

    /// Adds to the `rios` field.
    fn add_rios(&mut self, rios: u64) {
        self.rios += rios;
    }

    /// Adds to the `wios` field.
    fn add_wios(&mut self, wios: u64) {
        self.wios += wios;
    }
}

type Accumulator = fn(&mut IoStat, u64);

static ACCUMULATORS: LazyLock<HashMap<&'static str, Accumulator>> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, Accumulator> = HashMap::with_capacity(4);

    m.insert("rbytes", IoStat::add_rbytes);
    m.insert("wbytes", IoStat::add_wbytes);
    m.insert("rios", IoStat::add_rios);
    m.insert("wios", IoStat::add_wios);

    m
});

impl KeyValueStat for IoStat {
    const SPLIT_CHAR: Option<char> = Some('=');
    const SKIP_LINES: usize = 0;
    const SKIP_VALUES: usize = 1;
    const ALLOW_DUPLICATE_KEYS: bool = true;
    const ALLOW_MULTIPLE_KV_PER_LINE: bool = true;
    #[inline]
    fn field_handlers() -> &'static HashMap<&'static str, fn(&mut Self, u64)> {
        &ACCUMULATORS
    }
}

#[cfg(test)]
mod tests {
    use crate::cgroup::stats::StatParseError;
    use crate::cgroup::stats::error::extract_stat_parse_error;

    use super::*;

    #[test]
    fn test_parse_empty_io_stat() {
        let data = "";
        let stat = IoStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat, IoStat::default());
    }

    #[test]
    fn test_parse_complete_io_stat() {
        let data = "\
8:0 rbytes=1024 wbytes=2048 rios=12 wios=24
254:0 rbytes=1024 wbytes=2048 rios=12 wios=24
";
        let stat = IoStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.rbytes, 2048);
        assert_eq!(stat.wbytes, 4096);
        assert_eq!(stat.rios, 24);
        assert_eq!(stat.wios, 48);
    }

    #[test]
    fn test_parse_partial_io_stat() {
        let data = "\
8:0 rbytes=1024 wbytes=2048
254:0 rios=12 wios=24
";
        let stat = IoStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.rbytes, 1024);
        assert_eq!(stat.wbytes, 2048);
        assert_eq!(stat.rios, 12);
        assert_eq!(stat.wios, 24);
    }

    #[test]
    fn test_parse_invalid_io_stat() {
        let data = "\
8:0 rbytes=abc wbytes=def
254:0 rios=12 wios=24
";
        let err = IoStat::from_reader(&mut data.as_bytes()).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        let err = extract_stat_parse_error(&err);
        match err {
            StatParseError::InvalidKeyValue {
                key, value, line, ..
            } => {
                assert_eq!(key, "rbytes");
                assert_eq!(value, "abc");
                assert_eq!(*line, 1);
            }
            _ => panic!("Expected InvalidKeyValue error"),
        }
    }

    #[test]
    fn test_ignore_unknown_keys_io_stat() {
        let data = "\
8:0 foo=100 rbytes=1024 bar=999
";
        let stat = IoStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.rbytes, 1024);
        assert_eq!(stat.wbytes, 0);
        assert_eq!(stat.rios, 0);
        assert_eq!(stat.wios, 0);
    }

    #[test]
    fn test_ignore_malformed_key_value_pairs() {
        let data = "\
8:0 rbytes=1024 malformedpair wios=24
";
        let stat = IoStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.rbytes, 1024);
        assert_eq!(stat.wios, 24);
        assert_eq!(stat.wbytes, 0);
        assert_eq!(stat.rios, 0);
    }

    #[test]
    fn test_parse_with_extra_whitespace() {
        let data = "\
8:0    rbytes=1000    wbytes=2000
    ";
        let stat = IoStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.rbytes, 1000);
        assert_eq!(stat.wbytes, 2000);
    }
}
