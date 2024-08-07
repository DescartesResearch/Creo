use super::ProgrammingLanguage::{self, *};
use crate::generator;

impl ProgrammingLanguage {
    pub fn get_local_handler_dependencies(
        &self,
        lib_dir: impl AsRef<std::path::Path>,
    ) -> std::io::Result<Vec<String>> {
        match self {
            Python(_) => generator::python::get_local_handler_dependencies(lib_dir),
            Rust(_) => generator::rust::get_local_handler_dependencies(lib_dir),
        }
    }

    pub fn dependency_file_name(&self) -> &'static str {
        match self {
            Python(_) => generator::python::DEPENDENCY_FILE_NAME,
            Rust(_) => generator::rust::DEPENDENCY_FILE_NAME,
        }
    }

    pub fn as_dependency_file_template_path(&self) -> &'static str {
        match self {
            Python(_) => generator::python::DEPENDENCY_FILE_TEMPLATE_PATH,
            Rust(_) => generator::rust::DEPENDENCY_FILE_TEMPLATE_PATH,
        }
    }
}
