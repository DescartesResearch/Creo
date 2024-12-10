use std::{collections::HashSet, hash::Hash};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[serde(remote = "Self")]
/// A non-empty vector.
///
/// *Note*: This is only checked once during deserialization. If the vector is modified afterwards,
/// it may empty.
pub struct NonEmptyVec<T>(Vec<T>);

impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for NonEmptyVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = NonEmptyVec::deserialize(deserializer)?;

        if this.is_empty() {
            return Err(serde::de::Error::custom("expected non-empty vector"));
        }

        Ok(this)
    }
}

impl<T> std::ops::Deref for NonEmptyVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for NonEmptyVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Deserialize, serde::Serialize)]
#[serde(remote = "Self")]
/// A vector of unique elemets.
///
/// *Note*: This is only checked once during deserialization. If the vector is modified afterwards,
/// it may contain duplicate Elements.
pub struct UniqueVec<T>(Vec<T>);

impl<'de, T: serde::Deserialize<'de> + Hash + Eq> serde::Deserialize<'de> for UniqueVec<T> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = UniqueVec::deserialize(deserializer)?;

        let mut seen = HashSet::new();
        let mut duplicates = Vec::with_capacity(this.len());
        for (index, element) in this.0.iter().enumerate() {
            if !seen.insert(element) {
                duplicates.push(format!("#{}", index));
            }
        }
        if !duplicates.is_empty() {
            return Err(serde::de::Error::custom(format!(
                "duplicate element(s) in unique vector at position(s) {}",
                duplicates.join(", ")
            )));
        }

        Ok(this)
    }
}

impl<T> std::ops::Deref for UniqueVec<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> std::ops::DerefMut for UniqueVec<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> From<Vec<T>> for UniqueVec<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::de::FromYamlStr;

    use super::*;

    const NON_EMPTY_VALID_INPUT: &str = "[1, 2, 3]";
    const NON_EMPTY_INVALID_INPUT: &str = "[]";

    const UNIQUE_VALID_INPUT: &str = "[1, 2, 3]";
    const UNIQUE_INVALID_INPUT: &str = "[1, 2, 3, 1, 2]";

    #[test]
    fn test_non_empyt_valid_input() {
        let v = NonEmptyVec::<usize>::from_yaml_str(NON_EMPTY_VALID_INPUT).unwrap();
        assert_eq!(&v[..], &[1, 2, 3]);
    }

    #[test]
    fn test_non_empty_invalid_input() {
        let e = NonEmptyVec::<usize>::from_yaml_str(NON_EMPTY_INVALID_INPUT).unwrap_err();
        let msg = e.to_string();
        assert!(
            msg.contains("expected non-empty vector"),
            "unexpected message: {msg}"
        );
    }

    #[test]
    fn test_unique_valid_input() {
        let v = UniqueVec::<usize>::from_yaml_str(UNIQUE_VALID_INPUT).unwrap();
        assert_eq!(&v[..], &[1, 2, 3]);
    }

    #[test]
    fn test_unique_invalid_input() {
        let e = UniqueVec::<usize>::from_yaml_str(UNIQUE_INVALID_INPUT).unwrap_err();
        let msg = e.to_string();
        assert!(msg.contains("#3"), "unexpected message: {msg}");
        assert!(msg.contains("#4"), "unexpected message: {msg}");
    }
}
