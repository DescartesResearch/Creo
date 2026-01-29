use super::ProgrammingLanguage::{self, *};
use crate::generator::{self, core::FileNameGenerator};

impl ProgrammingLanguage {
    pub(crate) fn as_file_name_generator(&self) -> Box<dyn FileNameGenerator> {
        match self {
            Python(_) => Box::new(generator::python::FileNameGenerator),
            Rust(_) => Box::new(generator::rust::FileNameGenerator),
            Node(_) => Box::new(generator::node::FileNameGenerator),
        }
    }
}
