use super::ProgrammingLanguage::{self, *};
use crate::generator;

impl ProgrammingLanguage {
    pub fn as_docker_template_path(&self) -> &'static str {
        match self {
            Python(_) => generator::python::DOCKERFILE_TEMPLATE_PATH,
            Rust(_) => generator::rust::DOCKERFILE_TEMPLATE_PATH,
        }
    }
}
