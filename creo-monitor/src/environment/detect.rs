use std::path::Path;

use super::checks::{
    contains_proc_mount, has_container_indicators, is_pid_namespace_isolated,
    matches_container_cgroup,
};

/// Available runtime environments for the monitoring tool.
#[derive(Debug, PartialEq, Eq)]
pub enum RuntimeEnvironment {
    /// Running directly on the host.
    Host,
    /// Running inside a containerized environment (e.g., Docker, Kubernetes, Podman).
    Container,
}

/// Detects whether the current system is running in a container or on the host.
///
/// This function performs a series of heuristic checks to determine the runtime context:
///
/// 1. Checks if `/proc` exists in the rootfs and whether the init PID namespace differs.
/// 2. Checks the content of `/proc/self/cgroup` for container-related patterns.
/// 3. Checks for known container-specific marker files or environment variables.
///
/// All individual errors are logged as warnings and do **not** cause this function to fail.
///
/// # Arguments
///
/// * `rootfs` - A path to the root filesystem to inspect.
///
/// # Returns
///
/// A [`RuntimeEnvironment`] indicating whether the environment is a [`Host`] or [`Container`].
///
/// [`Host`]: RuntimeEnvironment::Host
/// [`Container`]: RuntimeEnvironment::Container
pub fn detect_runtime_environment(rootfs: impl AsRef<Path>) -> RuntimeEnvironment {
    let rootfs = rootfs.as_ref();
    match contains_proc_mount(rootfs) {
        Ok(true) => match is_pid_namespace_isolated(rootfs) {
            Ok(true) => return RuntimeEnvironment::Container,
            Ok(false) => {}
            Err(err) => log::warn!(
                "Namespace check failed when detecting runtime environment: {}",
                err
            ),
        },
        Ok(false) => {}
        Err(err) => log::warn!("Failed to determine presence of /proc in rootfs: {}", err),
    }

    match matches_container_cgroup() {
        Ok(true) => return RuntimeEnvironment::Container,
        Ok(false) => {}
        Err(err) => log::warn!("Cgroup analysis failed during runtime detection: {}", err),
    }

    if has_container_indicators() {
        return RuntimeEnvironment::Container;
    }

    RuntimeEnvironment::Host
}
