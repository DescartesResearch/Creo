use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

type Parser<T> = fn(&mut dyn BufRead) -> std::io::Result<T>;

/// Reads from a file, applies the given parser function, and rewinds the file cursor to the start.
///
/// Returns `Ok(None)` if the file is `None`.
pub fn read_and_rewind<T, R>(file: Option<&mut R>, parser: Parser<T>) -> std::io::Result<Option<T>>
where
    R: Read + Seek,
{
    if let Some(f) = file {
        let mut buf = BufReader::new(&mut *f);
        let result = parser(&mut buf)?;
        f.seek(SeekFrom::Start(0))?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

/// Reads from all provided files using the given parser function, rewinds them, and sums the results.
///
/// Returns `Ok(None)` if the list of files is empty.
pub fn read_all_and_rewind<T, R>(files: &mut [R], parser: Parser<T>) -> std::io::Result<Option<T>>
where
    T: std::ops::AddAssign + Default,
    R: Read + Seek,
{
    if files.is_empty() {
        return Ok(None);
    }

    let mut sum = T::default();

    for file in files {
        let mut buf = BufReader::new(&mut *file);
        let value = parser(&mut buf)?;
        file.seek(SeekFrom::Start(0))?;
        sum += value;
    }

    Ok(Some(sum))
}

#[inline]
pub fn open_file(path: impl AsRef<std::path::Path>) -> Option<std::fs::File> {
    Some(std::fs::File::open(path).ok()?)
}
