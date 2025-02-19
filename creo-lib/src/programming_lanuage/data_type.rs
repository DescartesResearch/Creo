use super::ProgrammingLanguage::{self, *};
use crate::generator::{self, core::DataTypeMapper};

impl ProgrammingLanguage {
    pub(crate) fn as_data_type_mapper(&self) -> &dyn DataTypeMapper {
        match self {
            Python(_) => &generator::python::DataTypeMapper,
            Rust(_) => &generator::rust::DataTypeMapper,
            Node(_) => &generator::node::DataTypeMapper
        }
    }
}
