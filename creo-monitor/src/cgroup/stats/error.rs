//! Defines structured error types for parsing cgroup statistics.
//!
//! This module provides the [`StatParseError`] enum, which encapsulates detailed
//! error reporting for parsing failures encountered while processing cgroup stat files.
//!
//! # Error Types
//!
//! - [`StatParseError::InvalidKeyValue`] — Indicates a key-value pair could not be parsed as expected.
//! - [`StatParseError::InvalidValue`] — Indicates a single numeric value (e.g., in `memory.current`) failed to parse.
//! - [`StatParseError::DuplicateField`] — Indicates a duplicate field was found where disallowed.
//! - [`StatParseError::Io`] — Wraps underlying I/O errors during file reads.
//!
//! # Integration
//!
//! `StatParseError` automatically converts to [`std::io::Error`] for seamless use with I/O APIs
//! and can be extracted in tests using the [`extract_stat_parse_error`] helper.
//!
//! # Example
//!
//! ```rust
//! use std::io;
//! use creo_monitor::cgroup::stats::StatParseError;
//!
//! fn parse_line(val: &str) -> io::Result<()> {
//!     let value = val.parse::<u64>().map_err(|e| {
//!         StatParseError::InvalidValue {
//!             value: val.to_string(),
//!             line: 1,
//!             source: e,
//!         }
//!     })?;
//!     Ok(())
//! }
//!
//! parse_line("not-a-number").unwrap_err();
//! ```

use std::num::ParseIntError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StatParseError {
    #[error("duplicate field '{field}' at line {line}")]
    DuplicateField { field: String, line: usize },

    #[error("invalid value for '{key}' at line {line}: '{value}': {source}")]
    InvalidKeyValue {
        key: String,
        value: String,
        line: usize,
        #[source]
        source: ParseIntError,
    },

    #[error("invalid value at line {line}: '{value}': {source}")]
    InvalidValue {
        value: String,
        line: usize,
        #[source]
        source: ParseIntError,
    },

    #[error("error during I/O: {0}")]
    Io(#[from] std::io::Error),
}

impl From<StatParseError> for std::io::Error {
    fn from(err: StatParseError) -> Self {
        match err {
            StatParseError::Io(e) => e,
            StatParseError::InvalidKeyValue { .. } => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, err)
            }
            StatParseError::InvalidValue { .. } => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, err)
            }
            StatParseError::DuplicateField { .. } => {
                std::io::Error::new(std::io::ErrorKind::InvalidData, err)
            }
        }
    }
}

/// Extracts a `StatParseError` from an `std::io::Error` assuming it was wrapped.
///
/// Panics if the inner error is not a `StatParseError`. Intended for use in test assertions only.
#[cfg(test)]
pub(super) fn extract_stat_parse_error(err: &std::io::Error) -> &StatParseError {
    err.get_ref()
        .and_then(|e| e.downcast_ref::<StatParseError>())
        .unwrap()
}
