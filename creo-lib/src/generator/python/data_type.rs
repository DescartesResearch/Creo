use crate::{
    generator::core::{self, LanguageDataType},
    template::Import,
};

pub struct DataTypeMapper;

impl core::DataTypeMapper for DataTypeMapper {
    fn get_string_type(&self) -> &'static str {
        "str"
    }

    fn get_date_type(&self) -> LanguageDataType {
        LanguageDataType {
            type_name: "date".into(),
            import: Some(Import::new("from datetime import date".into())),
        }
    }

    fn get_date_time_type(&self) -> LanguageDataType {
        LanguageDataType {
            type_name: "datetime".into(),
            import: Some(Import::new("from datetime import datetime".into())),
        }
    }

    fn get_floating_point_number_type(&self) -> &'static str {
        "float"
    }

    fn get_double_type(&self) -> &'static str {
        "float"
    }

    fn get_signed_32_bit_integer_type(&self) -> &'static str {
        "int"
    }

    fn get_signed_64_bit_integer_type(&self) -> &'static str {
        "int"
    }

    fn get_boolean_type(&self) -> &'static str {
        "bool"
    }
}
