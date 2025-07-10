//! This module provides parsing utilities for CPU statistics as reported in Linux cgroup files.
//!
//! It supports parsing of two primary types of CPU-related statistics:
//!
//! - **Key-value pair style statistics** from `cpu.stat` files.
//!   These files consist of multiple lines with whitespace-separated keys and values,
//!   describing detailed CPU usage metrics such as usage time, throttling counts, and bursts.
//!   The parsing enforces unique keys, strict error handling, and maps fields to a strongly
//!   typed [`CpuStat`] structure.
//!
//! - **Single-line statistics** from `cpu.max` files.
//!   These files contain either a quota and period pair, or a special `"max"` value indicating
//!   unlimited CPU quota. The data is parsed into the [`CpuLimit`] struct with clear semantics
//!   for quota and enforcement period.
//!
//! # Parsing assumptions
//!
//! - For multi-field stats (`cpu.stat`), the format is expected as one key-value pair per line,
//!   separated by whitespace, with no duplicate keys allowed.
//! - For single-line stats (`cpu.max`), the line contains either one or two whitespace-separated
//!   fields, where quota can be a numeric value or `"max"`.
//!
//! # Error handling
//!
//! The parsers provide explicit error reporting on invalid keys, malformed values, duplicate fields,
//! and other common data issues encountered in cgroup stat files.
//!
//! # Examples
//!
//! ```rust
//! use std::io::BufReader;
//! use creo_monitor::cgroup::stats::{CpuStat, CpuLimit, KeyValueStat, SingleLineStat};
//!
//! let data = "\
//! usage_usec 1000000
//! user_usec 600000
//! system_usec 400000
//! nr_periods 10
//! nr_throttled 2
//! throttled_usec 50000
//! nr_bursts 1
//! burst_usec 10000
//! ";
//! let mut reader = BufReader::new(data.as_bytes());
//! let cpu_stat = CpuStat::from_reader(&mut reader).unwrap();
//!
//! let limit_data = "max 100000\n";
//! let mut limit_reader = BufReader::new(limit_data.as_bytes());
//! let cpu_limit = CpuLimit::from_reader(&mut limit_reader).unwrap();
//! ```

use std::collections::HashMap;
use std::io::BufRead;
use std::sync::LazyLock;

use super::{KeyValueStat, SingleLineStat};

/// Represents parsed data from a cgroup `cpu.stat` file.
///
/// All fields correspond to values provided by the Linux kernel in microseconds (`_usec`)
/// or counts (`nr_*`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CpuStat {
    /// Total time (in microseconds) that the cgroup used CPU (user + system).
    pub usage_usec: u64,
    /// Time (in microseconds) spent in user space.
    pub user_usec: u64,
    /// Time (in microseconds) spent in kernel (system) space.
    pub system_usec: u64,
    /// Number of scheduling periods in which the cgroup was eligible to run.
    pub nr_periods: u64,
    /// Number of periods in which the cgroup was throttled.
    pub nr_throttled: u64,
    /// Total time (in microseconds) the cgroup was throttled.
    pub throttled_usec: u64,
    /// Number of CPU burst periods.
    pub nr_bursts: u64,
    /// Total time (in microseconds) spent in bursts.
    pub burst_usec: u64,
}

impl CpuStat {
    /// Sets the `usage_usec` field.
    fn set_usage_usec(&mut self, usage_usec: u64) {
        self.usage_usec = usage_usec;
    }
    /// Sets the `user_usec` field.
    fn set_user_usec(&mut self, user_usec: u64) {
        self.user_usec = user_usec;
    }

    /// Sets the `system_usec` field.
    fn set_system_usec(&mut self, system_usec: u64) {
        self.system_usec = system_usec;
    }

    /// Sets the `nr_periods` field.
    fn set_nr_periods(&mut self, nr_periods: u64) {
        self.nr_periods = nr_periods;
    }

    /// Sets the `nr_throttled` field.
    fn set_nr_throttled(&mut self, nr_throttled: u64) {
        self.nr_throttled = nr_throttled;
    }

    /// Sets the `throttled_usec` field.
    fn set_throttled_usec(&mut self, throttled_usec: u64) {
        self.throttled_usec = throttled_usec;
    }

    /// Sets the `nr_bursts` field.
    fn set_nr_bursts(&mut self, nr_bursts: u64) {
        self.nr_bursts = nr_bursts;
    }

    /// Sets the `burst_usec` field.
    fn set_burst_usec(&mut self, burst_usec: u64) {
        self.burst_usec = burst_usec;
    }
}

type Setter = fn(&mut CpuStat, u64);

static SETTERS: LazyLock<HashMap<&'static str, Setter>> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, Setter> = HashMap::with_capacity(8);

    m.insert("usage_usec", CpuStat::set_usage_usec);
    m.insert("user_usec", CpuStat::set_user_usec);
    m.insert("system_usec", CpuStat::set_system_usec);
    m.insert("nr_periods", CpuStat::set_nr_periods);
    m.insert("nr_throttled", CpuStat::set_nr_throttled);
    m.insert("throttled_usec", CpuStat::set_throttled_usec);
    m.insert("nr_bursts", CpuStat::set_nr_bursts);
    m.insert("burst_usec", CpuStat::set_burst_usec);

    m
});

impl KeyValueStat for CpuStat {
    const SPLIT_CHAR: Option<char> = None;
    const SKIP_LINES: usize = 0;
    const SKIP_VALUES: usize = 0;
    const ALLOW_DUPLICATE_KEYS: bool = false;
    const ALLOW_MULTIPLE_KV_PER_LINE: bool = false;

    fn field_handlers() -> &'static HashMap<&'static str, fn(&mut Self, u64)> {
        &SETTERS
    }
}

/// Represents CPU limits from `cpu.max`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CpuLimit {
    /// Maximum allowed CPU time in microseconds over each period.
    ///
    /// A value of `None` represents no quota (i.e., unlimited, as specified by `"max"` in the file).
    pub quota: Option<u64>,
    /// Duration (in microseconds) of each enforcement period.
    ///
    /// The Linux kernel uses this value to determine the interval for applying the CPU quota.
    /// If omitted in the input, this defaults to 100,000µs (100ms).
    pub period: u64,
}

const DEFAULT_PERIOD: u64 = 100_000;
impl Default for CpuLimit {
    fn default() -> Self {
        Self {
            quota: None,
            period: DEFAULT_PERIOD,
        }
    }
}

impl SingleLineStat for CpuLimit {
    /// Parses a `cpu.max`-style file from a buffered reader into a `CpuLimit` structure.
    ///
    /// The input is expected to contain either:
    /// - Two whitespace-separated values: `<quota> <period>`, or
    /// - A single value `"max"` followed optionally by a period.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable reference to a type implementing `BufRead`, containing the `cpu.max` line.
    ///
    /// # Returns
    ///
    /// * `Ok(CpuLimit)` if parsing succeeds. If quota is `"max"`, `quota` will be `None`.
    ///
    /// # Errors
    ///
    /// This function returns an `Ok` even if quota or period parsing fails,
    /// falling back to default period of `100_000` and `None` for `quota` on `"max"`.
    fn from_reader<R: BufRead>(buf: &mut R) -> std::io::Result<Self> {
        let mut line = String::new();
        buf.read_line(&mut line)?;
        let mut parts = line.split_whitespace();
        let quota_str = parts.next().unwrap_or("max");
        let period = parts
            .next()
            .and_then(|p| p.parse::<u64>().ok())
            .unwrap_or(DEFAULT_PERIOD);

        let quota = if quota_str == "max" {
            None
        } else {
            quota_str.parse::<u64>().ok()
        };

        Ok(CpuLimit { quota, period })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cgroup::stats::error::{StatParseError, extract_stat_parse_error};

    #[test]
    fn test_parse_empty_cpu_stat() {
        let data = "";
        let stat = CpuStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat, CpuStat::default());
    }

    #[test]
    fn test_parse_complete_cpu_stat() {
        let data = "\
usage_usec 623932088000
user_usec 421230248000
system_usec 202701840000
nr_periods 0
nr_throttled 0
throttled_usec 0
nr_bursts 0
burst_usec 0
";
        let stat = CpuStat::from_reader(&mut data.as_bytes()).unwrap();

        assert_eq!(stat.usage_usec, 623_932_088_000);
        assert_eq!(stat.user_usec, 421_230_248_000);
        assert_eq!(stat.system_usec, 202_701_840_000);
        assert_eq!(stat.nr_periods, 0);
        assert_eq!(stat.nr_throttled, 0);
        assert_eq!(stat.throttled_usec, 0);
        assert_eq!(stat.nr_bursts, 0);
        assert_eq!(stat.burst_usec, 0);
    }

    #[test]
    fn test_parse_partial_cpu_stat() {
        let data = "\
usage_usec 100
user_usec 60
system_usec 40
";
        let stat = CpuStat::from_reader(&mut data.as_bytes()).unwrap();

        assert_eq!(stat.usage_usec, 100);
        assert_eq!(stat.user_usec, 60);
        assert_eq!(stat.system_usec, 40);
        assert_eq!(stat.nr_periods, 0); // defaults
        assert_eq!(stat.burst_usec, 0);
    }

    #[test]
    fn test_parse_invalid_cpu_stat() {
        let data = "\
invalid_line
usage_usec abc
user_usec 42
";
        let err = CpuStat::from_reader(&mut data.as_bytes()).unwrap_err();

        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        let err = extract_stat_parse_error(&err);
        match err {
            StatParseError::InvalidKeyValue {
                key, value, line, ..
            } => {
                assert_eq!(key, "usage_usec");
                assert_eq!(value, "abc");
                assert_eq!(*line, 2);
            }
            _ => panic!("Expected InvalidKeyValue error"),
        }
    }
    #[test]
    fn test_duplicate_field_errors() {
        let data = "\
usage_usec 100
usage_usec 200
";
        let err = CpuStat::from_reader(&mut data.as_bytes()).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        let err = extract_stat_parse_error(&err);
        match err {
            StatParseError::DuplicateField { field, line } => {
                assert_eq!(field, "usage_usec");
                assert_eq!(*line, 2);
            }
            _ => panic!("Expected DuplicateField error"),
        }
    }

    #[test]
    fn test_parse_empty_cpu_limit() {
        let data = "";
        let stat = CpuLimit::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat, CpuLimit::default());
    }

    #[test]
    fn test_parse_complete_cpu_limit() {
        let data = b"\
50000 100000
";
        let limit = CpuLimit::from_reader(&mut &data[..]).unwrap();
        assert_eq!(limit.quota, Some(50000));
        assert_eq!(limit.period, 100000);
    }

    #[test]
    fn test_parse_cpu_limit_max_quota() {
        let data = b"max 250000";
        let limit = CpuLimit::from_reader(&mut &data[..]).unwrap();
        assert_eq!(limit.quota, None);
        assert_eq!(limit.period, 250000);
    }

    #[test]
    fn test_parse_cpu_limit_missing_period() {
        let data = b"max";
        let limit = CpuLimit::from_reader(&mut &data[..]).unwrap();
        assert_eq!(limit.quota, None);
        assert_eq!(limit.period, 100_000);
    }
}
