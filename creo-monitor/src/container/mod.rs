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
//! - [`PodID`]: a 32-byte lowercase alphanumeric identifier representing a logical pod,
//!   grouping multiple containers under a single scheduling unit.
//!
//! These identifiers are opaque and should not be parsed or manipulated as
//! structured strings. Consumers should use the provided constructors to
//! ensure validity, and the `as_str()` methods for display or logging purposes.
//!
//! # Examples
//!
//! ```
//! use creo_monitor::container::{ContainerID, PodID};
//!
//! let container_id = ContainerID::new(*b"abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd").unwrap();
//! assert_eq!(container_id.as_str(), "abc123abc123abc123abc123abc123abc123abc123abc123abc123abc123abcd");
//!
//! let pod_id = PodID::new(*b"abc123abc123abc123abc123abc123ab").unwrap();
//! assert_eq!(pod_id.as_str(), "abc123abc123abc123abc123abc123ab");
//! ```

use std::fmt;

mod error;
mod metadata;
mod utils;

pub use error::{Error, Result};
pub use metadata::{ContainerDMetaDataProvider, ContainerMeta, PodMeta};

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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
}

impl fmt::Display for ContainerID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A validated pod identifier consisting of exactly 32 lowercase ASCII alphanumeric bytes.
///
/// `PodID` ensures that all bytes are either lowercase ASCII letters (`a-z`) or digits (`0-9`).
/// This constraint is checked at creation time via [`PodID::new`], making it safe to assume
/// validity throughout the lifetime of a `PodID` instance.
///
/// # Examples
///
/// ```
/// # use creo_monitor::container::{PodID, Error};
/// let raw_id = *b"abc123abc123abc123abc123abc123ab";
/// let pod_id = PodID::new(raw_id).unwrap();
/// assert_eq!(pod_id.as_str(), "abc123abc123abc123abc123abc123ab");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PodID([u8; 32]);

impl PodID {
    /// Creates a new `PodID` from a 32-byte array.
    ///
    /// Validates that all bytes are lowercase alphanumeric ASCII characters (i.e., `'0'..='9'` or `'a'..='z'`).
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidPodID`] if the input contains any non-lowercase ASCII letter or digit.
    ///
    /// # Examples
    ///
    /// ```
    /// # use creo_monitor::container::{PodID, Error};
    /// let valid = *b"abc123abc123abc123abc123abc123ab";
    /// let id = PodID::new(valid);
    /// assert!(id.is_ok());
    ///
    /// let invalid = *b"ABC123abc123abc123abc123abc123AB";
    /// assert!(matches!(
    ///     PodID::new(invalid),
    ///     Err(Error::InvalidPodID(_))
    /// ));
    /// ```
    pub fn new(src: [u8; 32]) -> Result<Self> {
        if !utils::is_lowercase_alpha_numeric(&src) {
            return Err(Error::InvalidPodID(
                String::from_utf8_lossy(&src).to_string(),
            ));
        }

        Ok(Self(src))
    }

    /// Returns the pod ID as a UTF-8 string slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use creo_monitor::container::PodID;
    /// let raw = *b"abc123abc123abc123abc123abc123ab";
    /// let id = PodID::new(raw).unwrap();
    /// assert_eq!(id.as_str(), "abc123abc123abc123abc123abc123ab");
    /// ```
    pub fn as_str(&self) -> &str {
        // SAFETY: we check in `new()` that all bytes are lowercase ascii characters or ascii digits
        unsafe { std::str::from_utf8_unchecked(&self.0) }
    }
}

impl fmt::Display for PodID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
