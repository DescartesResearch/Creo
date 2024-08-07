use super::{Error, Result};

pub fn get_local_prefix(path: &impl AsRef<std::path::Path>) -> &std::path::Path {
    path.as_ref()
        .parent()
        .unwrap_or_else(|| std::path::Path::new(""))
}

pub fn remote_path_from_prefix(
    local: impl AsRef<std::path::Path>,
    prefix: impl AsRef<std::path::Path>,
) -> Result<String> {
    let remote = local.as_ref().strip_prefix(prefix.as_ref()).map_err(|_| {
        Error::InvalidArgument(format!(
            "expected {} to be a prefix of {}",
            prefix.as_ref().display(),
            local.as_ref().display()
        ))
    })?;
    Ok(path_to_str(&remote)?.into())
}

pub fn path_to_str(path: &impl AsRef<std::path::Path>) -> Result<&str> {
    path.as_ref().to_str().ok_or_else(|| {
        Error::InvalidArgument(format!(
            "expected only valid UTF-8 in path {}",
            path.as_ref().display()
        ))
    })
}
