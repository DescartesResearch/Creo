pub trait ResultOkLogExt<T, E> {
    fn ok_log(self) -> Option<T>;
}

impl<T, E> ResultOkLogExt<T, E> for std::result::Result<T, E>
where
    E: std::error::Error,
{
    fn ok_log(self) -> Option<T> {
        match self {
            Ok(ok) => Some(ok),
            Err(err) => {
                log::error!("{err}");
                None
            }
        }
    }
}
