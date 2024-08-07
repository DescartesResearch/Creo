use crate::{
    generator::core::{self, LanguageDataType},
    template::Import,
};

pub struct DataTypeMapper;

impl core::DataTypeMapper for DataTypeMapper {
    fn get_string_type(&self) -> &'static str {
        "String"
    }

    fn get_date_type(&self) -> LanguageDataType {
        LanguageDataType {
            type_name: "NaiveDate".into(),
            import: Some(Import::new("use chrono::NaiveDate;".into())),
        }
    }

    fn get_date_time_type(&self) -> LanguageDataType {
        LanguageDataType {
            type_name: "NaiveDateTime".into(),
            import: Some(Import::new("use chrono::NaiveDateTime".into())),
        }
    }

    fn get_floating_point_number_type(&self) -> &'static str {
        "f32"
    }

    fn get_double_type(&self) -> &'static str {
        "f64"
    }

    fn get_signed_32_bit_integer_type(&self) -> &'static str {
        "i32"
    }

    fn get_signed_64_bit_integer_type(&self) -> &'static str {
        "i64"
    }

    fn get_boolean_type(&self) -> &'static str {
        "bool"
    }
}
