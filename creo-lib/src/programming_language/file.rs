use super::ProgrammingLanguage::{self, *};
use crate::generator::{self, core::FileNameGenerator};

impl ProgrammingLanguage {
    pub(crate) fn as_file_name_generator(&self) -> &dyn FileNameGenerator {
        match self {
            Python(_) => &generator::python::FileNameGenerator,
            Rust(_) => &generator::rust::FileNameGenerator,
            Node(_) => &generator::node::FileNameGenerator,
        }
    }
}
