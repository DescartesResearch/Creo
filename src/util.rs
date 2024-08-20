use crate::Error;

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

pub trait DigitExt {
    /// Returns the number of digits in `self`.
    fn digits(&self) -> usize;
}

impl DigitExt for usize {
    fn digits(&self) -> usize {
        (self.checked_ilog10().unwrap_or_default() + 1) as usize
    }
}
