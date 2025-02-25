/// PassingType represents the expected type of argument passing of a particular parameter.
#[derive(serde::Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
#[serde(untagged, remote = "Self")]
pub enum PassingType {
    /// The Pos variant refers to passing a positional argument.
    /// The integer indicates the index of the parameter in the function signature.
    Pos(u32),
    /// The Kw variant refers to passing a keyword argument.
    /// The string indicates the name of the parameter in the function signature.
    Kw(String),
}

impl<'de> serde::Deserialize<'de> for PassingType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let this = Self::deserialize(deserializer)?;
        match this {
            PassingType::Pos(_) => {}
            PassingType::Kw(ref name) => {
                if name.is_empty() {
                    return Err(serde::de::Error::custom(
                        "expected non-empty string for keyword name",
                    ));
                }
            }
        }

        Ok(this)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::FromYamlStr;

    #[test]
    fn test_zero_pos() {
        const INPUT: &str = "0";

        PassingType::from_yaml_str(INPUT).expect("should parse");
    }

    #[test]
    fn test_negative_pos() {
        const INPUT: &str = "-1";

        PassingType::from_yaml_str(INPUT).expect_err("should not allow negative pos");
    }

    #[test]
    fn test_positive_pos() {
        const INPUT: &str = "1";

        PassingType::from_yaml_str(INPUT).expect("should parse");
    }

    #[test]
    fn test_non_empty_kw() {
        const INPUT: &str = "some_name";

        PassingType::from_yaml_str(INPUT).expect("should parse");
    }

    #[test]
    fn test_empty_kw() {
        const INPUT: &str = "";

        PassingType::from_yaml_str(INPUT).expect_err("should not allow empty keyword name");
    }

    #[test]
    fn test_ordering() {
        let pos_0 = PassingType::Pos(0);
        let pos_1 = PassingType::Pos(1);
        let kw_a = PassingType::Kw("a".to_string());
        let kw_b = PassingType::Kw("b".to_string());

        let mut unordered = vec![kw_b, kw_a, pos_1, pos_0];
        unordered.sort();
        assert_eq!(
            unordered,
            vec![
                PassingType::Pos(0),
                PassingType::Pos(1),
                PassingType::Kw("a".to_string()),
                PassingType::Kw("b".to_string())
            ]
        )
    }
}
