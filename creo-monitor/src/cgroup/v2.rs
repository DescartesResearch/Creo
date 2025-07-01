use std::collections::VecDeque;
use std::ffi::OsStr;
use std::io::BufRead;
use std::os::unix::ffi::OsStrExt;
use std::path::Path;

use crate::container::{ContainerDMetaDataProvider, ContainerID, PodID};

use super::{ContainerRuntime, ContainerSlice, Monitor};

/// Default implementation of [`super::ContainerScanner`] for cgroup v2.
#[derive(Debug, Default)]
pub struct Scanner;

impl super::ContainerScanner for Scanner {
    async fn scan_path(
        &self,
        path: &std::path::Path,
        monitor: &mut Monitor,
        containerd_meta_provider: &mut ContainerDMetaDataProvider,
    ) -> std::io::Result<()> {
        scan_cgroup_tree(path, None, monitor, containerd_meta_provider).await
    }
}

/// Recursively scans the cgroup v2 hierarchy from a root path,
/// identifying container and pod slices by naming convention.
async fn scan_cgroup_tree(
    path: impl AsRef<std::path::Path>,
    pod_id: Option<crate::container::PodID>,
    monitor: &mut Monitor,
    containerd_meta_provider: &mut ContainerDMetaDataProvider,
) -> std::io::Result<()> {
    let mut stack = VecDeque::new();
    stack.push_back((path.as_ref().to_path_buf(), pod_id));
    while let Some((path, pod_id)) = stack.pop_back() {
        let mut entries = tokio::fs::read_dir(&path).await?;
        while let Some(entry) = entries.next_entry().await? {
            let ft = entry.file_type().await?;
            if !ft.is_dir() {
                continue;
            }

            let path = entry.path();

            if monitor.is_tracking_path(&path) {
                continue;
            }

            let file_name = entry.file_name();
            if let Some(pod_id) = extract_pod_id(&file_name) {
                stack.push_back((path, Some(pod_id)));
                continue;
            }

            if let Some(slice) =
                try_build_container_slice(&path, &file_name, pod_id, containerd_meta_provider).await
            {
                monitor.register_container(&path, slice);
            }

            stack.push_back((path, pod_id));
        }
    }

    Ok(())
}

#[inline]
async fn try_build_container_slice(
    path: &Path,
    file_name: &OsStr,
    pod_id: Option<PodID>,
    containerd_meta_provider: &mut ContainerDMetaDataProvider,
) -> Option<ContainerSlice> {
    let (container_id, runtime) = extract_container_id(file_name)?;
    let (container_meta, pod_meta) = match runtime {
        ContainerRuntime::Docker => (None, None),
        ContainerRuntime::ContainerD => {
            match containerd_meta_provider
                .request_metadata(container_id)
                .await
            {
                Ok(Some((container_meta, pod_meta))) => (Some(container_meta), Some(pod_meta)),
                Ok(None) => (None, None),
                Err(_) => (None, None),
            }
        }
        ContainerRuntime::Podman => (None, None),
    };
    let pids = read_pids_from(path)?;
    Some(match pod_id {
        Some(pod_id) => {
            ContainerSlice::new_pod(container_id, pod_id, pids, path, container_meta, pod_meta)
        }
        None => ContainerSlice::new_standalone(container_id, pids, path, container_meta),
    })
}

#[inline]
fn read_pids_from(path: &Path) -> Option<Vec<u32>> {
    let file = std::fs::File::open(path.join("cgroup.procs")).ok()?;
    let reader = std::io::BufReader::new(file);
    let mut pids = Vec::new();

    for line in reader.lines().map_while(Result::ok) {
        if let Ok(pid) = line.parse::<u32>() {
            pids.push(pid);
        }
    }

    Some(pids)
}

/// Tries to extract a [`crate::container::ContainerID`] from the given file name.
///
/// Recognizes Docker, Podman, and containerd prefixes.
#[inline]
fn extract_container_id(name: &OsStr) -> Option<(crate::container::ContainerID, ContainerRuntime)> {
    const ID_LENGTH_IN_PATH: usize = 64;
    const CONTAINER_ID_LENGTH: usize = 64;

    let suffix = b".scope";

    let attempts: &[(&'static [u8], ContainerRuntime)] = &[
        (b"cri-containerd-", ContainerRuntime::ContainerD),
        (b"docker-", ContainerRuntime::Docker),
        (b"libpod-", ContainerRuntime::Podman),
    ];

    let name = name.as_bytes();
    for &(prefix, runtime) in attempts {
        if let Some(id_bytes) = extract_id_from_path_bytes(name, prefix, suffix, ID_LENGTH_IN_PATH)
        {
            let id_array = super::utils::create_array_from_iter::<_, CONTAINER_ID_LENGTH>(
                id_bytes.iter().copied(),
            )?;
            let container_id = ContainerID::new(id_array).ok()?;
            return Some((container_id, runtime));
        }
    }
    None
}

/// Tries to extract a Kubernetes [`crate::container::PodID`] from the given file name.
///
/// Recognizes raw, best-effort, burstable, and guaranteed pod slices.
#[inline]
fn extract_pod_id(name: &std::ffi::OsStr) -> Option<crate::container::PodID> {
    /// Length with "_", i.e. 32 alphanumeric characters + 4 "_"
    const ID_LENGTH_IN_PATH: usize = 36;
    const POD_ID_LENGTH: usize = 32;

    let raw_prefix = b"kubepods-pod";
    let burstable_prefix = b"kubepods-burstable-pod";
    let best_effort_prefix = b"kubepods-besteffort-pod";
    let guaranteed_prefix = b"kubepods-guaranteed-pod";
    let suffix = b".slice";

    let name = name.as_bytes();

    let id_bytes = extract_id_from_path_bytes(name, raw_prefix, suffix, POD_ID_LENGTH)
        .or_else(|| extract_id_from_path_bytes(name, burstable_prefix, suffix, ID_LENGTH_IN_PATH))
        .or_else(|| extract_id_from_path_bytes(name, best_effort_prefix, suffix, ID_LENGTH_IN_PATH))
        .or_else(|| {
            extract_id_from_path_bytes(name, guaranteed_prefix, suffix, ID_LENGTH_IN_PATH)
        })?;

    let clean_bytes = id_bytes.iter().filter(|&&b| b != b'_').copied();
    let id_array = super::utils::create_array_from_iter::<_, POD_ID_LENGTH>(clean_bytes)?;
    crate::container::PodID::new(id_array).ok()
}

/// Extracts an ID from the given path, if it has the given prefix and suffix, and has the given
/// expected length if the prefix and suffix are stripped.
#[inline]
fn extract_id_from_path_bytes<'a>(
    path_bytes: &'a [u8],
    prefix: &[u8],
    suffix: &[u8],
    expected_length: usize,
) -> Option<&'a [u8]> {
    if path_bytes.starts_with(prefix)
        && path_bytes.ends_with(suffix)
        && path_bytes.len() == prefix.len() + expected_length + suffix.len()
    {
        return Some(&path_bytes[prefix.len()..(path_bytes.len() - suffix.len())]);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_extract_valid_container_id() {
        let name = OsStr::new(
            "docker-0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef.scope",
        );
        assert!(extract_container_id(name).is_some());
    }

    #[test]
    fn test_extract_invalid_container_id() {
        let name = OsStr::new("docker-invalid.scope");
        assert!(extract_container_id(name).is_none());
    }

    #[test]
    fn test_extract_valid_pod_id() {
        let name = OsStr::new("kubepods-guaranteed-pod12345678_90ab_cdef_1234_567890abcdef.slice");
        assert!(extract_pod_id(name).is_some());
    }

    #[test]
    fn test_extract_invalid_pod_id() {
        let name = OsStr::new("kubepods-unknown.slice");
        assert!(extract_pod_id(name).is_none());
    }
}
