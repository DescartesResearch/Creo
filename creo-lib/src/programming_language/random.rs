use super::ProgrammingLanguage::{self, *};
use crate::generator::{self, core::FrameworkGenerator};

impl ProgrammingLanguage {
    pub fn choose_random_framework<R: rand::Rng>(
        &self,
        rng: &mut R,
    ) -> Box<dyn FrameworkGenerator> {
        match self {
            Python(_) => Box::new(rng.gen::<generator::python::Frameworks>()),
            Rust(_) => Box::new(rng.gen::<generator::rust::Frameworks>()),
        }
    }
}
