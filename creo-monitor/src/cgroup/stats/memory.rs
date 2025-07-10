//! This module provides parsing utilities for memory statistics as reported in Linux cgroup files.
//!
//! It supports parsing of various memory-related statistics, including:
//!
//! - **Key-value pair style statistics** from `memory.stat` files.
//!   These files contain multiple lines with whitespace-separated keys and values,
//!   representing detailed memory usage categories. The parsing enforces unique keys,
//!   robust error handling, and converts the data into a structured [`MemoryStat`] type.
//!
//! - **Single-line scalar statistics** from files like `memory.current` and `memory.max`.
//!   These contain either a single numeric value representing current memory usage or
//!   memory limits, or special values such as `"max"` indicating unlimited memory.
//!   These are parsed into dedicated types [`MemoryUsage`] and [`MemoryLimit`] respectively.
//!
//! # Parsing assumptions
//!
//! - For multi-field stats (`memory.stat`), the format is expected to be
//!   whitespace-separated key-value pairs with one key-value pair per line.
//! - For single-line stats (`memory.current`, `memory.max`), the file contains exactly
//!   one line with a single value or keyword.
//!
//! # Error handling
//!
//! Parsing provides clear error reporting, including detection of invalid keys or values,
//! duplicate keys in multi-field stats, and invalid numeric formats.
//!
//! # Examples
//!
//! ```rust
//! use std::io::BufReader;
//! use creo_monitor::cgroup::stats::{MemoryStat, MemoryUsage, MemoryLimit, KeyValueStat, SingleLineStat};
//!
//! let data = "anon 1000\nfile 2000\n";
//! let mut reader = BufReader::new(data.as_bytes());
//! let mem_stat = MemoryStat::from_reader(&mut reader).unwrap();
//!
//! let usage_data = "8192\n";
//! let mut usage_reader = BufReader::new(usage_data.as_bytes());
//! let mem_usage = MemoryUsage::from_reader(&mut usage_reader).unwrap();
//!
//! let limit_data = "max\n";
//! let mut limit_reader = BufReader::new(limit_data.as_bytes());
//! let mem_limit = MemoryLimit::from_reader(&mut limit_reader).unwrap();
//! ```

use std::collections::HashMap;
use std::io::BufRead;
use std::sync::LazyLock;

use super::parser::KeyValueStat;
use super::{SingleLineStat, StatParseError};

/// Represents memory usage statistics from `memory.stat`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MemoryStat {
    /// Anonymous memory.
    pub anon: u64,
    /// File-backed memory.
    pub file: u64,
    /// Kernel stack memory.
    pub kernel_stack: u64,
    /// Slab memory (used for kernel object caches).
    pub slab: u64,
    /// Socket memory usage.
    pub sock: u64,
    /// Shared memory.
    pub shmem: u64,
    /// Mapped file memory.
    pub file_mapped: u64,
}

impl MemoryStat {
    /// Sets the `anon` field.
    fn set_anon(&mut self, v: u64) {
        self.anon = v;
    }

    /// Sets the `file` field.
    fn set_file(&mut self, v: u64) {
        self.file = v;
    }

    /// Sets the `kernel_stack` field.
    fn set_kernel_stack(&mut self, v: u64) {
        self.kernel_stack = v;
    }

    /// Sets the `slab` field.
    fn set_slab(&mut self, v: u64) {
        self.slab = v;
    }

    /// Sets the `sock` field.
    fn set_sock(&mut self, v: u64) {
        self.sock = v;
    }

    /// Sets the `shmem` field.
    fn set_shmem(&mut self, v: u64) {
        self.shmem = v;
    }

    /// Sets the `file_mapped` field.
    fn set_file_mapped(&mut self, v: u64) {
        self.file_mapped = v;
    }
}

type Setter = fn(&mut MemoryStat, u64);

static SETTERS: LazyLock<HashMap<&'static str, Setter>> = LazyLock::new(|| {
    let mut m: HashMap<&'static str, Setter> = HashMap::with_capacity(7);

    m.insert("anon", MemoryStat::set_anon);
    m.insert("file", MemoryStat::set_file);
    m.insert("kernel_stack", MemoryStat::set_kernel_stack);
    m.insert("slab", MemoryStat::set_slab);
    m.insert("sock", MemoryStat::set_sock);
    m.insert("shmem", MemoryStat::set_shmem);
    m.insert("file_mapped", MemoryStat::set_file_mapped);

    m
});

impl KeyValueStat for MemoryStat {
    const SPLIT_CHAR: Option<char> = None;
    const SKIP_LINES: usize = 0;
    const SKIP_VALUES: usize = 0;
    const ALLOW_DUPLICATE_KEYS: bool = false;
    const ALLOW_MULTIPLE_KV_PER_LINE: bool = false;

    fn field_handlers() -> &'static HashMap<&'static str, fn(&mut Self, u64)> {
        &SETTERS
    }
}

/// Represents memory usage statistics from `memory.current`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MemoryUsage {
    /// Total memory usage in bytes.
    pub usage_bytes: u64,
}

impl SingleLineStat for MemoryUsage {
    /// Parses a `memory.current`-style file from a buffered reader into a `MemoryUsage` structure.
    ///
    /// The input is expected to contain a single numeric value representing the current memory usage in bytes.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable reference to a type implementing `BufRead`, containing the `memory.current` data.
    ///
    /// # Returns
    ///
    /// * `Ok(MemoryUsage)` if the value is successfully parsed.
    /// * `Err(std::io::Error)` if the value fails to parse.
    ///
    /// # Errors
    ///
    /// This function returns an error of kind `std::io::ErrorKind::InvalidData` if the value cannot be parsed as a `u64`.
    fn from_reader<R: BufRead>(buf: &mut R) -> std::io::Result<Self> {
        let mut stat = MemoryUsage::default();
        let mut line = String::new();

        buf.read_line(&mut line)?;
        let line = line.trim();
        stat.usage_bytes = line
            .parse::<u64>()
            .map_err(|source| StatParseError::InvalidValue {
                value: line.to_string(),
                line: 1,
                source,
            })?;

        Ok(stat)
    }
}

/// Represents memory limits from `memory.max`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct MemoryLimit {
    /// Memory usage limit in bytes.
    ///
    /// A value of `None` represents "max", meaning no memory limit is set.
    pub limit_bytes: Option<u64>,
}

impl SingleLineStat for MemoryLimit {
    /// Parses a `memory.max`-style file from a buffered reader into a `MemoryLimit` structure.
    ///
    /// The input is expected to be either a numeric value representing the memory limit in bytes,
    /// or the string "max" to indicate no memory limit.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable reference to a type implementing `BufRead`, containing the `memory.max` data.
    ///
    /// # Returns
    ///
    /// * `Ok(MemoryLimit)` with `Some(limit)` if a numeric value is provided.
    /// * `Ok(MemoryLimit)` with `None` if the value is "max".
    fn from_reader<R: BufRead>(buf: &mut R) -> std::io::Result<Self> {
        let mut line = String::new();
        buf.read_line(&mut line)?;
        let limit_bytes = match line.trim() {
            "max" => None,
            value => value.parse::<u64>().ok(),
        };

        Ok(MemoryLimit { limit_bytes })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cgroup::stats::error::extract_stat_parse_error;

    #[test]
    fn test_parse_empty_memory_stat() {
        let data = "";
        let stat = MemoryStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat, MemoryStat::default());
    }

    #[test]
    fn test_parse_complete_memory_stat() {
        let data = "\
anon 1000
file 2000
kernel_stack 300
slab 400
sock 500
shmem 600
file_mapped 700
";
        let stat = MemoryStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.anon, 1000);
        assert_eq!(stat.file, 2000);
        assert_eq!(stat.kernel_stack, 300);
        assert_eq!(stat.slab, 400);
        assert_eq!(stat.sock, 500);
        assert_eq!(stat.shmem, 600);
        assert_eq!(stat.file_mapped, 700);
    }

    #[test]
    fn test_parse_partial_memory_stat() {
        let data = "\
anon 1000
file 2000
kernel_stack 300
";
        let stat = MemoryStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.anon, 1000);
        assert_eq!(stat.file, 2000);
        assert_eq!(stat.kernel_stack, 300);
        assert_eq!(stat.slab, 0);
        assert_eq!(stat.sock, 0);
        assert_eq!(stat.shmem, 0);
        assert_eq!(stat.file_mapped, 0);
    }

    #[test]
    fn test_parse_invalid_memory_stat() {
        let data = "\
invalid line
anon abc
file 2000
kernel_stack 300
";
        let err = MemoryStat::from_reader(&mut data.as_bytes()).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        let err = extract_stat_parse_error(&err);
        match err {
            StatParseError::InvalidKeyValue {
                key, value, line, ..
            } => {
                assert_eq!(key, "anon");
                assert_eq!(value, "abc");
                assert_eq!(*line, 2);
            }
            _ => panic!("Expected InvalidKeyValue error"),
        }
    }

    #[test]
    fn test_duplicate_memory_stat_field() {
        let data = "\
anon 1000
anon 2000
";
        let err = MemoryStat::from_reader(&mut data.as_bytes()).unwrap_err();
        let err = extract_stat_parse_error(&err);
        match err {
            StatParseError::DuplicateField { field, line } => {
                assert_eq!(field, "anon");
                assert_eq!(*line, 2);
            }
            _ => panic!("Expected DuplicateField error"),
        }
    }

    #[test]
    fn test_ignore_unknown_keys() {
        let data = "\
anon 1000
unknown_key 12345
file 2000
";
        let stat = MemoryStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.anon, 1000);
        assert_eq!(stat.file, 2000);
    }

    #[test]
    fn test_extra_whitespace() {
        let data = "\
    anon     1000
file     2000
    kernel_stack     300
";
        let stat = MemoryStat::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat.anon, 1000);
        assert_eq!(stat.file, 2000);
        assert_eq!(stat.kernel_stack, 300);
    }

    #[test]
    fn test_parse_empty_memory_usage() {
        let data = "";
        let err = MemoryUsage::from_reader(&mut data.as_bytes()).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        let err = extract_stat_parse_error(&err);
        match err {
            StatParseError::InvalidValue { value, line, .. } => {
                assert_eq!(value, "");
                assert_eq!(*line, 1);
            }
            _ => panic!("Expected InvalidValue Error"),
        }
    }

    #[test]
    fn test_parse_complete_memory_usage() {
        let data = "\
8192
";

        let stat = MemoryUsage::from_reader(&mut data.as_bytes()).unwrap();

        assert_eq!(stat.usage_bytes, 8192);
    }

    #[test]
    fn test_parse_invalid_memory_usage() {
        let data = "\
abcd
";

        let err = MemoryUsage::from_reader(&mut data.as_bytes()).unwrap_err();
        assert_eq!(err.kind(), std::io::ErrorKind::InvalidData);
        let err = extract_stat_parse_error(&err);
        match err {
            StatParseError::InvalidValue { value, line, .. } => {
                assert_eq!(value, "abcd");
                assert_eq!(*line, 1);
            }
            _ => panic!("Expected InvalidValue error"),
        }
    }

    #[test]
    fn test_parse_empty_memory_limit() {
        let data = "";
        let stat = MemoryLimit::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(stat, MemoryLimit::default());
    }

    #[test]
    fn test_parse_complete_memory_limit() {
        let data = "\
max
";
        let limit = MemoryLimit::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(limit.limit_bytes, None);

        let data = "\
104857600
";
        let limit = MemoryLimit::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(limit.limit_bytes, Some(104857600));
    }

    #[test]
    fn test_invalid_memory_limit() {
        let data = "\
abc
";
        let limit = MemoryLimit::from_reader(&mut data.as_bytes()).unwrap();
        assert_eq!(limit.limit_bytes, None);
    }
}
