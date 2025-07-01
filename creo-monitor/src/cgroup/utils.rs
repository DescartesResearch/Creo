use std::io::{BufRead, Seek, SeekFrom};

/// Collects exactly `N` items from an iterator into an array.
pub(super) fn create_array_from_iter<T, const N: usize>(
    iter: impl Iterator<Item = T>,
) -> Option<[T; N]>
where
    T: Copy + Sized,
{
    let mut out: [std::mem::MaybeUninit<T>; N] = [const { std::mem::MaybeUninit::uninit() }; N];
    let mut iter = iter.into_iter();
    for elem in out.iter_mut() {
        let val = iter.next()?;
        elem.write(val);
    }

    if iter.next().is_some() {
        return None;
    }

    // SAFETY: We initialized the entire array with elements from the iterator and ensured the
    // iterator and the array have the same length.
    let out = unsafe {
        let ptr = &out as *const _ as *const [T; N];
        ptr.read()
    };

    Some(out)
}

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
    T: std::iter::Sum<T>,
    F: Fn(&mut R) -> std::io::Result<T>,
    R: BufRead + Seek,
{
    if files.is_empty() {
        return Ok(None);
    }

    let mut results = Vec::with_capacity(files.len());

    for f in files {
        let value = reader(f)?;
        f.seek(SeekFrom::Start(0))?;
        results.push(value);
    }

    Ok(Some(results.into_iter().sum()))
}
