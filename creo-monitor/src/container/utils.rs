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
