mod detect;
mod error;
mod parser;

pub use detect::{detect_cgroup2_mount_point, detect_validated_cgroup2_mount_point};
pub use error::{Error, Result};
