//! Provides generic traits for parsing Linux cgroup statistics files into structured types.
//!
//! This module defines reusable parsing abstractions for extracting metrics from files such as
//! `cpu.stat`, `memory.stat`, `io.stat`, and others commonly found under `/sys/fs/cgroup` or `/proc`.
//!
//! # Traits
//!
//! - [`KeyValueStat`]: A trait for parsing multi-line, key-value formatted stat files. Configurable parsing behavior
//!   allows support for varying formats (e.g., space-separated vs. equals-sign-separated).
//! - [`SingleLineStat`]: A trait for parsing single-line statistics containing a single numeric value, such as `memory.current` or `memory.max`.
//!
//! # Key Features
//!
//! - Skips arbitrary lines or fields before parsing begins.
//! - Supports detection of duplicate keys and optional enforcement of uniqueness.
//! - Gracefully handles unknown keys via customizable hooks.
//! - Consolidates handler-based field population for robust extensibility.
//!
//! # Example: Implementing `KeyValueStat`
//!
//! ```rust
//! use std::collections::HashMap;
//! use creo_monitor::cgroup::stats::KeyValueStat;
//! use std::sync::OnceLock;
//!
//! #[derive(Default)]
//! struct MyStat {
//!     foo: u64,
//!     bar: u64,
//! }
//!
//! // or use [`std::sync::LazyLock`] instead of [`std::sync::OnceLock`]
//! static HANDLERS: OnceLock<HashMap<&'static str, fn(&mut MyStat, u64)>> = OnceLock::new();
//!
//! impl MyStat {
//!     fn set_foo(&mut self, foo: u64) {
//!         self.foo = foo;
//!     }
//!
//!     fn set_bar(&mut self, bar: u64) {
//!         self.bar = bar;
//!     }
//! }
//!
//! impl KeyValueStat for MyStat {
//!     const SPLIT_CHAR: Option<char> = Some('=');
//!     const SKIP_LINES: usize = 0;
//!     const SKIP_VALUES: usize = 0;
//!     const ALLOW_DUPLICATE_KEYS: bool = false;
//!     const ALLOW_MULTIPLE_KV_PER_LINE: bool = true;
//!
//!     fn field_handlers() -> &'static HashMap<&'static str, fn(&mut Self, u64)> {
//!         HANDLERS.get_or_init(|| {
//!             let mut map = HashMap::new();
//!             map.insert("foo", MyStat::set_foo as fn(&mut MyStat, u64));
//!             map.insert("bar", MyStat::set_bar as fn(&mut MyStat, u64));
//!             map
//!         })
//!     }
//! }
//! ```

use std::collections::{HashMap, HashSet};
use std::io::BufRead;

use super::StatParseError;

/// A trait for parsing structured key-value style `*.stat` files such as
/// `cpu.stat`, `memory.stat`, `io.stat`, etc., commonly found in Linux `/sys/fs/cgroup` or `/proc`.
///
/// Implementors define a set of known keys and how to set values for them.
/// This trait provides a generic implementation to read and parse these files
/// line by line with configurable parsing behavior.
pub trait KeyValueStat: Default
where
    Self: 'static,
{
    /// If set to `Some(char)`, each key-value pair is expected to be joined by that character.
    ///
    /// For example, a line like `"somekey=123"` would be parsed using `SPLIT_CHAR = Some('=')`.
    ///
    /// If `None`, the parser assumes each key and value are separated by whitespace,
    /// like `"somekey 123 otherkey 456"`.
    const SPLIT_CHAR: Option<char>;

    /// The number of lines at the start of the file to skip before parsing begins.
    const SKIP_LINES: usize;

    /// The number of whitespace-separated values to skip at the start of *each line*.
    const SKIP_VALUES: usize;

    /// If `true`, repeated keys are allowed in the file (e.g., multiple entries for the same field).
    /// If `false`, encountering the same key more than once will cause an error.
    const ALLOW_DUPLICATE_KEYS: bool;

    /// If `true`, the parser will attempt to consume multiple key-value pairs per line.
    /// If `false`, only the first key-value pair on each line is parsed.
    const ALLOW_MULTIPLE_KV_PER_LINE: bool;

    /// Returns a map of known field names and corresponding handler functions
    /// that apply parsed values (e.g., set or accumulate) to the struct's fields.
    ///
    /// # Returns
    /// A reference to a map where keys are field names and values are functions
    /// that mutate the implementing struct based on the parsed value.
    fn field_handlers() -> &'static HashMap<&'static str, fn(&mut Self, u64)>;

    /// Parses a key-value formatted buffer into a struct implementing `KeyValueStat`.
    ///
    /// This will skip the first `SKIP_LINES` lines, then process each line using
    /// the configured split behavior and handler mapping. Unknown fields are ignored
    /// by default (see `on_unknown_key()`).
    ///
    /// # Arguments
    /// * `buf` - A buffered reader for the input stream.
    ///
    /// # Returns
    /// A populated instance of the struct implementing `KeyValueStat`.
    ///
    /// # Errors
    /// Returns an `io::Error` if reading fails, or a `StatParseError` wrapped in `io::Error` if parsing fails.
    fn from_reader<R: BufRead>(buf: &mut R) -> std::io::Result<Self> {
        let mut stat = Self::default();
        let handlers = Self::field_handlers();
        let field_count = handlers.len();
        let mut seen_keys = HashSet::with_capacity(field_count);

        let mut line = String::new();
        let mut lineno = 0;
        for _ in 0..Self::SKIP_LINES {
            buf.read_line(&mut line)?;
            line.clear();
        }

        while buf.read_line(&mut line)? != 0 {
            lineno += 1;
            Self::parse_line(&mut stat, &line, lineno, handlers, &mut seen_keys)?;
            if !Self::ALLOW_DUPLICATE_KEYS && seen_keys.len() == field_count {
                break;
            }

            line.clear();
        }

        Ok(stat)
    }

    /// Parses a single line into one or more key-value pairs based on the trait configuration.
    ///
    /// # Arguments
    /// * `stat` - Mutable reference to the struct being populated.
    /// * `line` - The current line from the input.
    /// * `lineno` - The current line number (1-based, after `SKIP_LINES`).
    /// * `handlers` - Map of known keys and associated handler functions.
    /// * `seen_keys` - Tracks which keys have already been parsed (used for duplication check).
    ///
    /// # Errors
    /// Returns an error if parsing fails or duplicate keys are found and disallowed.
    fn parse_line<'a>(
        stat: &mut Self,
        line: &'a str,
        lineno: usize,
        handlers: &HashMap<&'static str, fn(&mut Self, u64)>,
        seen_keys: &mut HashSet<&'static str>,
    ) -> std::io::Result<()> {
        let mut parts = line.split_whitespace().skip(Self::SKIP_VALUES);

        if let Some(split_char) = Self::SPLIT_CHAR {
            Self::parse_split_pairs(&mut parts, split_char, stat, lineno, handlers, seen_keys)
        } else {
            Self::parse_flat_pairs(&mut parts, stat, lineno, handlers, seen_keys)
        }
    }

    /// Parses a line with space-separated alternating key/value tokens (e.g., `key1 123 key2 456`).
    ///
    /// # Arguments
    /// * `parts` - An iterator over the split tokens in the line.
    /// * `stat` - The struct to populate.
    /// * `lineno` - Line number used for error reporting.
    /// * `handlers` - Map of field handlers.
    /// * `seen_keys` - Set of already-seen keys.
    ///
    /// # Errors
    /// Returns an error if a key-value pair fails to parse or a duplicate key is found and not allowed.
    fn parse_flat_pairs<'a>(
        parts: &mut impl Iterator<Item = &'a str>,
        stat: &mut Self,
        lineno: usize,
        handlers: &HashMap<&'static str, fn(&mut Self, u64)>,
        seen_keys: &mut HashSet<&'static str>,
    ) -> std::io::Result<()> {
        while let (Some(key), Some(val)) = (parts.next(), parts.next()) {
            Self::parse_and_set(key, val, stat, lineno, handlers, seen_keys)?;
            if !Self::ALLOW_MULTIPLE_KV_PER_LINE {
                break;
            }
        }
        Ok(())
    }

    /// Parses a line with `key<split_char>value` format tokens (e.g., `key1=123 key2=456`).
    ///
    /// # Arguments
    /// * `parts` - An iterator over the tokens split by whitespace.
    /// * `split_char` - Character used to split keys from values.
    /// * `stat` - The struct being populated.
    /// * `lineno` - Line number used for error reporting.
    /// * `handlers` - Map of field handlers.
    /// * `seen_keys` - Set of already-seen keys.
    ///
    /// # Errors
    /// Returns an error if parsing a value fails or a duplicate key is found and not allowed.
    fn parse_split_pairs<'a>(
        parts: &mut impl Iterator<Item = &'a str>,
        split_char: char,
        stat: &mut Self,
        lineno: usize,
        handlers: &HashMap<&'static str, fn(&mut Self, u64)>,
        seen_keys: &mut HashSet<&'static str>,
    ) -> std::io::Result<()> {
        for part in parts {
            if let Some((key, val)) = part.split_once(split_char) {
                Self::parse_and_set(key, val, stat, lineno, handlers, seen_keys)?;
            }
            if !Self::ALLOW_MULTIPLE_KV_PER_LINE {
                break;
            }
        }
        Ok(())
    }

    /// Parses a single key-value pair and updates the target struct via the field handler.
    ///
    /// If the key is unknown, `on_unknown_key` is called.
    ///
    /// # Arguments
    /// * `key` - The key to match against known field handlers.
    /// * `val` - The value string to parse into a `u64`.
    /// * `stat` - The struct to populate.
    /// * `lineno` - Line number for contextual error reporting.
    /// * `handlers` - Map of keys to handler functions.
    /// * `seen_keys` - Set of already-seen keys for duplication check.
    ///
    /// # Returns
    /// `Ok(())` if the value was successfully parsed and applied.
    ///
    /// # Errors
    /// Returns a `StatParseError::InvalidKeyValue` if the value cannot be parsed as `u64`,
    /// or `StatParseError::DuplicateField` if the key appears more than once and duplicates are disallowed.
    fn parse_and_set(
        key: &str,
        val: &str,
        stat: &mut Self,
        lineno: usize,
        handlers: &HashMap<&'static str, fn(&mut Self, u64)>,
        seen_keys: &mut HashSet<&'static str>,
    ) -> std::io::Result<()> {
        if let Some((k, handler)) = handlers.get_key_value(key) {
            let parsed = val
                .parse::<u64>()
                .map_err(|source| StatParseError::InvalidKeyValue {
                    key: key.to_string(),
                    value: val.to_string(),
                    line: lineno,
                    source,
                })?;
            if !Self::ALLOW_DUPLICATE_KEYS && !seen_keys.insert(k) {
                return Err(StatParseError::DuplicateField {
                    field: key.to_string(),
                    line: lineno,
                }
                .into());
            }
            handler(stat, parsed);
            return Ok(());
        }

        Self::on_unknown_key(key, val, lineno)
    }

    /// Called when a key in the input is not found in the `field_handlers()` map.
    ///
    /// Override this method to implement custom behavior for unknown keys
    /// (e.g., logging, filtering, or collecting metrics).
    ///
    /// By default, unknown keys are silently ignored.
    ///
    /// # Arguments
    /// * `key` - The unknown field name.
    /// * `val` - The associated value string.
    /// * `lineno` - Line number in the file where the unknown field was found.
    ///
    /// # Returns
    /// Default implementation returns `Ok(())`. Override to log, error, or collect unknown keys.
    #[inline]
    fn on_unknown_key(_key: &str, _val: &str, _lineno: usize) -> std::io::Result<()> {
        Ok(())
    }
}

/// A trait for parsing single-line, single-value statistics, such as
/// `memory.current` or `memory.max` files.
///
/// Implementors provide a method to parse from a buffered reader,
/// returning the strongly typed structure.
pub trait SingleLineStat: Sized + Default {
    /// Parses a single-line statistic from the provided buffered reader.
    ///
    /// # Arguments
    ///
    /// * `buf` - A mutable reference to a type implementing `BufRead` containing
    ///
    /// # Returns
    ///
    /// * `Ok(Self)` if parsing succeeds.
    /// * `Err(std::io::Error)` if reading or parsing fails.
    fn from_reader<R: BufRead>(buf: &mut R) -> std::io::Result<Self>;
}
