#[derive(serde::Deserialize, PartialEq, Debug, Clone)]
pub struct Param {
    pub arg: super::PassingType,
    #[serde(flatten)]
    pub schema: crate::schema::Schema,
}

impl PartialOrd for Param {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Param {}

impl Ord for Param {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.arg.cmp(&other.arg)
    }
}

impl Param {
    pub fn is_primitive_type(&self) -> bool {
        match &self.schema.schema_kind {
            crate::schema::SchemaKind::Type(type_schema) => !matches!(
                type_schema,
                crate::schema::Type::Object(_) | crate::schema::Type::Array(_)
            ),
        }
    }

    pub fn as_name(&self) -> String {
        match &self.arg {
            super::PassingType::Kw(name) => name.clone(),
            super::PassingType::Pos(pos) => match &self.schema.schema_data.title {
                Some(title) => title.clone(),
                None => format!("positional{}", pos),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::de::FromYamlStr;

    use super::*;

    #[test]
    fn test_postional_string_param() {
        const INPUT: &str = "
            type: string
            format: email
            arg: 0
        ";

        let param = Param::from_yaml_str(INPUT).unwrap();
        match param.arg {
            crate::handler::PassingType::Pos(pos) => assert_eq!(pos, 0),
            _ => panic!("expected Positional `PassingType`"),
        }
    }

    #[test]
    fn test_keyword_string_param() {
        const INPUT: &str = "
            type: string
            title: name
            arg: name
        ";
        let param = Param::from_yaml_str(INPUT).unwrap();
        match param.arg {
            crate::handler::PassingType::Kw(keyword) => assert_eq!(keyword, "name"),
            _ => panic!("expected Keyword `PassingType`"),
        }
    }

    #[test]
    fn test_optional_param() {
        const INPUT: &str = "
            type: integer
            nullable: true
            arg: 0
        ";

        let param = Param::from_yaml_str(INPUT).unwrap();
        assert!(param.schema.schema_data.nullable)
    }
}
