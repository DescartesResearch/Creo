//! Environment detection module.
//!
//! Determines whether the program is running on the host or inside a container.
mod checks;
mod detect;
mod error;

pub use detect::{RuntimeEnvironment, detect_runtime_environment};
pub use error::{Error, Result};
