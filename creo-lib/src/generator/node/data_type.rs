use crate::{
    generator::core::{self, LanguageDataType},
};

pub struct DataTypeMapper;

impl core::DataTypeMapper for DataTypeMapper {
    fn get_string_type(&self) -> &'static str {
        "String"
    }

    fn get_date_type(&self) -> LanguageDataType {
        LanguageDataType {
            type_name: "Date".into(),
            import: None,
        }
    }

    fn get_date_time_type(&self) -> LanguageDataType {
        LanguageDataType {
            type_name: "Date".into(),
            import: None,
        }
    }

    fn get_floating_point_number_type(&self) -> &'static str {
        "Number"
    }

    fn get_double_type(&self) -> &'static str {
        "Number"
    }

    fn get_signed_32_bit_integer_type(&self) -> &'static str {
        "Number"
    }

    fn get_signed_64_bit_integer_type(&self) -> &'static str {
        "Number"
    }

    fn get_boolean_type(&self) -> &'static str {
        "Boolean"
    }
}
