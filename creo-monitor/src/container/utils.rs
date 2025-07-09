/// Checks whether all bytes in the given slice are lowercase alphanumeric ASCII characters.
///
/// This function returns `true` if every byte in the input slice is either an ASCII
/// digit (`'0'..='9'`) or a lowercase ASCII letter (`'a'..='z'`). It returns `false`
/// if any byte falls outside of these ranges, including uppercase letters, symbols,
/// or non-ASCII characters.
///
/// # Arguments
///
/// * `src` - A byte slice to check.
///
/// # Returns
///
/// `true` if all bytes are lowercase alphanumeric ASCII characters, otherwise `false`.
pub(super) fn is_lowercase_alpha_numeric(src: &[u8]) -> bool {
    src.iter()
        .all(|b| b.is_ascii_digit() || b.is_ascii_lowercase())
}

/// Collects exactly `N` items from an iterator into an array.
///
/// Returns None if the iterator did not yield exactly `N` elements.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_lowercase_alpha_numeric() {
        let valid = b"abc123";
        assert!(is_lowercase_alpha_numeric(valid));

        let with_upper = b"abcXYZ123";
        assert!(!is_lowercase_alpha_numeric(with_upper));

        let with_symbol = b"abc_123";
        assert!(!is_lowercase_alpha_numeric(with_symbol));

        let empty: &[u8] = b"";
        assert!(is_lowercase_alpha_numeric(empty));
    }
}
