use std::io::{BufRead, BufReader, Read};

type Parser<T> = fn(&mut dyn BufRead) -> std::io::Result<T>;

/// Reads from a file, applies the given parser function.
///
/// Returns `Ok(None)` if the file is `None`.
pub fn read<T, R>(file: Option<&mut R>, parser: Parser<T>) -> std::io::Result<Option<T>>
where
    R: Read,
{
    if let Some(f) = file {
        let mut buf = BufReader::new(&mut *f);
        let result = parser(&mut buf)?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

/// Reads from all provided files using the given parser function, and sums the results.
///
/// Returns `Ok(None)` if the list of files is empty.
pub fn read_all<T, R>(files: &mut [R], parser: Parser<T>) -> std::io::Result<Option<T>>
where
    T: std::ops::AddAssign + Default,
    R: Read,
{
    if files.is_empty() {
        return Ok(None);
    }

    let mut sum = T::default();

    for file in files {
        let mut buf = BufReader::new(&mut *file);
        let value = parser(&mut buf)?;
        sum += value;
    }

    Ok(Some(sum))
}

#[inline]
pub fn open_file(path: impl AsRef<std::path::Path>) -> Option<std::fs::File> {
    std::fs::File::open(path).ok()
}
