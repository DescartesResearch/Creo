use rand_derive::Rand;

use crate::{generator::core::FrameworkGenerator, template};

pub mod axum;

#[derive(Rand)]
pub enum Frameworks {
    Axum,
}
use Frameworks::*;

impl FrameworkGenerator for Frameworks {
    fn to_faker(&self) -> &dyn template::Fakeable {
        match self {
            Axum => &axum::Faker,
        }
    }

    fn to_router_generator(&self) -> &dyn template::RouterGenerator {
        match self {
            Axum => &axum::RouterGenerator,
        }
    }

    fn to_service_calls_generator(&self) -> &dyn template::ServiceCallGenerator {
        match self {
            Axum => &axum::ServiceCallGenerator,
        }
    }

    fn to_main_generator(&self) -> &dyn template::MainGenerator {
        match self {
            Axum => &axum::MainGenerator,
        }
    }

    fn get_framework_requirements(&self) -> Vec<&'static str> {
        match self {
            Axum => axum::get_framework_dependencies(),
        }
    }

    fn get_docker_entrypoint(&self) -> &'static str {
        match self {
            Axum => axum::DOCKER_ENTRYPOINT,
        }
    }
}
