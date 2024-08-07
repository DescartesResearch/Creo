use indexmap::IndexSet;

pub(crate) fn to_data_type(
    schema_kind: &crate::schema::SchemaKind,
    imports: &mut IndexSet<crate::template::Import>,
    data_type_mapper: &dyn DataTypeMapper,
) -> String {
    match schema_kind {
        crate::schema::SchemaKind::Type(type_schema) => match type_schema {
            crate::schema::Type::String(string_type) => {
                let openapiv3::VariantOrUnknownOrEmpty::Item(format) = string_type.format else {
                    return data_type_mapper.get_string_type().into();
                };
                match format {
                    openapiv3::StringFormat::Date => {
                        let dt = data_type_mapper.get_date_type();
                        if let Some(import) = dt.import {
                            imports.insert(import);
                        };
                        dt.type_name
                    }
                    openapiv3::StringFormat::DateTime => {
                        let dt = data_type_mapper.get_date_time_type();
                        if let Some(import) = dt.import {
                            imports.insert(import);
                        }
                        dt.type_name
                    }
                    _ => data_type_mapper.get_string_type().into(),
                }
            }
            crate::schema::Type::Number(number_type) => {
                let openapiv3::VariantOrUnknownOrEmpty::Item(format) = number_type.format else {
                    return data_type_mapper.get_floating_point_number_type().into();
                };
                match format {
                    openapiv3::NumberFormat::Float => {
                        data_type_mapper.get_floating_point_number_type().into()
                    }
                    openapiv3::NumberFormat::Double => data_type_mapper.get_double_type().into(),
                }
            }
            crate::schema::Type::Integer(integer_type) => {
                let openapiv3::VariantOrUnknownOrEmpty::Item(format) = integer_type.format else {
                    return data_type_mapper.get_signed_64_bit_integer_type().into();
                };

                match format {
                    openapiv3::IntegerFormat::Int32 => {
                        data_type_mapper.get_signed_32_bit_integer_type().into()
                    }
                    openapiv3::IntegerFormat::Int64 => {
                        data_type_mapper.get_signed_64_bit_integer_type().into()
                    }
                }
            }
            crate::schema::Type::Boolean(_) => data_type_mapper.get_boolean_type().into(),
            crate::schema::Type::Object(_) => {
                unreachable!("should not be called with object type")
            }
            crate::schema::Type::Array(_) => unreachable!("should not be called with array type"),
        },
    }
}

/// [`LanguageDataType`] represents a specific data type of a programming language.
pub struct LanguageDataType {
    /// The name of the data type.
    pub type_name: String,
    /// If needed, `import` contains the import statement for making the data type accessible.
    pub import: Option<crate::template::Import>,
}

pub(crate) trait DataTypeMapper {
    /// Gets the primitive string type.
    fn get_string_type(&self) -> &'static str;

    /// Gets the date datatype.
    fn get_date_type(&self) -> LanguageDataType;

    /// Gets the date time datatype.
    fn get_date_time_type(&self) -> LanguageDataType;

    /// Gets the primitive floating point number type.
    fn get_floating_point_number_type(&self) -> &'static str;

    /// Gets the primitive double number type.
    fn get_double_type(&self) -> &'static str;

    /// Gets the primitive signed 32-bit integer number type.
    fn get_signed_32_bit_integer_type(&self) -> &'static str;

    /// Gets the primitive signed 64-bit integer number type.
    fn get_signed_64_bit_integer_type(&self) -> &'static str;

    /// Gets the primitive boolean type.
    fn get_boolean_type(&self) -> &'static str;
}
