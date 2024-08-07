use crate::Error;

pub fn digits(n: usize) -> usize {
    (n.checked_ilog10().unwrap_or_default() + 1) as usize
}

pub fn cleanup_dir(path: impl AsRef<std::path::Path>) {
    let path = path.as_ref();
    if path.exists() {
        log::info!("Trying to clean up directory at path {}...", path.display());
        match std::fs::remove_dir_all(path).map_err(|err| {
            Error::with_log(
                format!("failed to clean up directory at path: {}", path.display()),
                err,
            )
        }) {
            Ok(_) => log::info!(
                "Successfully cleaned up directory at path: {}",
                path.display()
            ),
            Err(err) => log::error!("{}", err),
        }
    }
}
