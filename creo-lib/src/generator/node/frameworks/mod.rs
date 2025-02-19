use rand_derive::Rand;

use crate::{generator::core::FrameworkGenerator, template};

pub mod express;

#[derive(Rand)]
pub enum Frameworks {
    Express,
}

use Frameworks::*;

impl FrameworkGenerator for Frameworks {
    fn to_faker(&self) -> &dyn template::Fakeable {
        match self {
            Express => &express::Faker,
        }
    }

    fn to_router_generator(&self) -> &dyn template::RouterGenerator {
        match self {
            Express => &express::RouterGenerator,
        }
    }

    fn to_service_calls_generator(&self) -> &dyn template::ServiceCallGenerator {
        match self {
            Express => &express::ServiceCallGenerator,
        }
    }

    fn to_main_generator(&self) -> &dyn template::MainGenerator {
        match self {
            Express => &express::MainGenerator,
        }
    }

    fn get_framework_requirements(&self) -> Vec<&'static str> {
        match self {
            Express => express::get_framework_dependencies(),
        }
    }

    fn get_docker_entrypoint(&self) -> &'static str {
        match self {
            Express => express::DOCKER_ENTRYPOINT,
        }
    }
}
