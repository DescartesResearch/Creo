use super::param::Param;

#[derive(serde::Deserialize, Clone, Debug)]
#[serde(remote = "Self")]
pub struct Signature {
    pub function: String,
    #[serde(alias = "params", default)]
    pub parameters: Vec<Param>,
    #[serde(default)]
    pub returns: Option<crate::schema::Schema>,
}

impl<'de> serde::Deserialize<'de> for Signature {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let mut this = Self::deserialize(deserializer)?;
        if this.function.is_empty() {
            return Err(serde::de::Error::custom(
                "expected non-empty string for function",
            ));
        }
        this.parameters.sort();
        let mut params_with_complex_types = 0;

        let mut param_index = 0u32;
        for param in &this.parameters {
            if let super::PassingType::Pos(pos) = param.arg {
                if pos != param_index {
                    return Err(serde::de::Error::custom(format!(
                        "expected next positional argument to have index {}, but was {}",
                        param_index, pos
                    )));
                }
                param_index += 1;
            }

            if param.schema.get_object_schema().is_some()
                || param.schema.get_array_schema_type().is_some()
            {
                params_with_complex_types += 1;
            }

            if params_with_complex_types > 1 {
                return Err(serde::de::Error::custom(
                    "only exactly one parameter with type object or array is allowed",
                ));
            }
        }

        Ok(this)
    }
}

#[cfg(test)]
mod tests {
    use crate::de::FromYamlStr;

    use super::*;

    #[test]
    fn test_signature_defaults() {
        const INPUT: &str = "function: some_name";

        Signature::from_yaml_str(INPUT).unwrap();
    }

    #[test]
    fn test_signature_with_params() {
        const INPUT: &str = "
            function: function_name
            parameters:
                - type: string
                  format: email
                  minLength: 8
                  maxLength: 48
                  arg: 0
        ";
        Signature::from_yaml_str(INPUT).unwrap();
    }

    #[test]
    fn test_signature_with_params_and_return_type() {
        const INPUT: &str = "
            function: function_name
            parameters:
                - type: string
                  format: email
                  minLength: 8
                  maxLength: 48
                  arg: 0
            returns:
                type: string
                format: email";

        Signature::from_yaml_str(INPUT).unwrap();
    }

    #[test]
    fn test_signature_with_gap_in_positional_params() {
        const INPUT: &str = "
            function: function_name
            parameters:
                - type: number
                  format: i32
                  arg: 2
                - type: string
                  arg: 0";
        let err = Signature::from_yaml_str(INPUT);
        assert!(err.is_err_and(
            |e| e.to_string() == "expected next positional argument to have index 1, but was 2"
        ));
    }

    #[test]
    fn test_multiple_object_parameters() {
        const INPUT: &str = "
            function: function_name
            parameters:
                - type: object
                  arg: 0
                - type: object
                  arg: 1";
        let err = Signature::from_yaml_str(INPUT);
        assert!(err
            .is_err_and(|e| e.to_string()
                == "only exactly one parameter with type object or array is allowed"));
    }
}
