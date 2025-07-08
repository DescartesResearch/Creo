use std::io::{BufRead, BufReader, Seek, SeekFrom};

/// Reads from a file, applies the given reader function, and rewinds the file cursor to the start.
///
/// Returns `Ok(None)` if the file is `None`.
pub fn read_and_rewind<T, R>(
    file: Option<&mut R>,
    reader: impl FnOnce(&mut R) -> std::io::Result<T>,
) -> std::io::Result<Option<T>>
where
    R: BufRead + Seek,
{
    if let Some(f) = file {
        let result = reader(f)?;
        f.seek(SeekFrom::Start(0))?;
        Ok(Some(result))
    } else {
        Ok(None)
    }
}

/// Reads from all provided files using the given reader function, rewinds them, and sums the results.
///
/// Returns `Ok(None)` if the list of files is empty.
pub fn read_all_and_rewind<T, F, R>(files: &mut [R], reader: F) -> std::io::Result<Option<T>>
where
    T: std::ops::AddAssign + Default,
    F: Fn(&mut R) -> std::io::Result<T>,
    R: BufRead + Seek,
{
    if files.is_empty() {
        return Ok(None);
    }

    let mut sum = T::default();

    for file in files {
        let value = reader(file)?;
        file.seek(SeekFrom::Start(0))?;
        sum += value;
    }

    Ok(Some(sum))
}

#[inline]
pub fn open_file(path: impl AsRef<std::path::Path>) -> Option<BufReader<std::fs::File>> {
    Some(BufReader::new(std::fs::File::open(path).ok()?))
}
