//! Container identity utilities for validated IDs and structured identifiers.
//!
//! This module defines strong types for identifying containers and pods using
//! fixed-length, validated ASCII strings, typically derived from runtime
//! orchestrators like Docker or Kubernetes. These types ensure correctness
//! by enforcing strict format requirements at construction time, enabling
//! safe and predictable usage throughout the monitoring stack.
//!
//! The primary types in this module are:
//!
//! - [`ContainerID`]: a 64-byte lowercase alphanumeric identifier used to uniquely
//!   identify a container.
//!
//! These identifiers are opaque and should not be parsed or manipulated as
//! structured strings. Consumers should use the provided constructors to
//! ensure validity, and the `as_str()` methods for display or logging purposes.
//!
//! # Examples
//!
//! ```
//! use creo_monitor::container::{ContainerID, MachineID};
//!
//! let container_id = ContainerID::new(*b"abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd").unwrap();
//! assert_eq!(container_id.as_str(), "abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd");
//!
//! let machine_id = MachineID::new(*b"abc123abc123abc1").unwrap();
//! assert_eq!(machine_id.to_string(), String::from("61626331323361626331323361626331"));
//! ```

use std::fmt;
use std::str::FromStr;

mod error;
mod utils;

pub use error::{Error, Result};

/// A validated container identifier consisting of exactly 64 lowercase ASCII alphanumeric bytes.
///
/// `ContainerID` ensures that all bytes in the ID are either ASCII digits (`0-9`) or lowercase
/// ASCII letters (`a-z`). This invariant is enforced at construction time via [`ContainerID::new`],
/// and consumers can safely assume that all instances are valid.
///
/// # Examples
///
/// ```
/// # use creo_monitor::container::{ContainerID, Error};
/// let raw_id = *b"abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd";
/// let container_id = ContainerID::new(raw_id).unwrap();
/// assert_eq!(container_id.as_str(), "abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ContainerID([u8; 64]);

impl ContainerID {
    /// Creates a new `ContainerID` from the given byte array.
    ///
    /// Returns an error if the input contains any non-lowercase alphanumeric ASCII characters.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidContainerID`] if the input contains characters
    /// other than lowercase letters (`a-z`) or digits (`0-9`).
    ///
    /// # Examples
    ///
    /// ```
    /// # use creo_monitor::container::{ContainerID, Error};
    /// let valid = *b"abcdef012345abcdef012345abcdef012345abcdef012345abcdef012345abcd";
    /// let id = ContainerID::new(valid);
    /// assert!(id.is_ok());
    ///
    /// let invalid = *b"ABCDEF012345ABCDEF012345ABCDEF012345ABCDEF012345ABCDEF012345ABCD";
    /// assert!(matches!(
    ///     ContainerID::new(invalid),
    ///     Err(Error::InvalidContainerID(_))
    /// ));
    /// ```
    pub fn new(src: [u8; 64]) -> Result<Self> {
        if !utils::is_lowercase_alpha_numeric(&src) {
            return Err(Error::InvalidContainerID(
                String::from_utf8_lossy(&src).to_string(),
            ));
        }

        Ok(Self(src))
    }

    /// Returns the container ID as a string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use creo_monitor::container::ContainerID;
    /// let raw = *b"abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd";
    /// let id = ContainerID::new(raw).unwrap();
    /// assert_eq!(id.as_str(), "abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd");
    /// ```
    pub fn as_str(&self) -> &str {
        // SAFETY: we check in `new()` that all bytes are lowercase ascii characters or ascii digits
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }

    pub fn as_raw(&self) -> [u8; 64] {
        self.0
    }
}

impl FromStr for ContainerID {
    type Err = Error;

    /// Attempts to parse a `ContainerID` from a string slice.
    ///
    /// Returns an error if the input is not exactly 64 characters long
    /// or contains characters other than lowercase letters (`a-z`) or digits (`0-9`).
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let bytes: [u8; 64] = utils::create_array_from_iter(s.as_bytes().iter().copied())
            .ok_or_else(|| Error::InvalidContainerID(s.to_owned()))?;

        ContainerID::new(bytes)
    }
}

impl fmt::Display for ContainerID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MachineID([u8; 16]);

impl MachineID {
    pub fn new(src: [u8; 16]) -> Result<Self> {
        Ok(Self(src))
    }

    pub fn as_raw(&self) -> [u8; 16] {
        self.0
    }
}

impl FromStr for MachineID {
    type Err = Error;

    /// Attempts to parse a `MachineID` from a string slice.
    ///
    /// Returns an error if the input is not exactly  characters long
    /// or contains characters other than lowercase letters (`a-z`) or digits (`0-9`).
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        if s.len() != 32 {
            return Err(Error::InvalidMachineID(s.to_owned()));
        }
        let mut bytes = [0u8; 16];
        for i in (0..s.len()).step_by(2) {
            bytes[i / 2] = u8::from_str_radix(&s[i..i + 2], 16)
                .map_err(|_| Error::InvalidMachineID(s.to_owned()))?;
        }

        MachineID::new(bytes)
    }
}

impl fmt::Display for MachineID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            write!(f, "{:02x}", b)?;
        }
        Ok(())
    }
}
