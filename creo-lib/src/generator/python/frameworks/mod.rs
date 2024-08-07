use rand_derive::Rand;

use crate::{generator::core::FrameworkGenerator, template};

pub mod fastapi;

#[derive(Rand)]
pub enum Frameworks {
    FastAPI,
}
use Frameworks::*;

impl FrameworkGenerator for Frameworks {
    fn to_faker(&self) -> &dyn template::Fakeable {
        match self {
            FastAPI => &fastapi::Faker,
        }
    }

    fn to_router_generator(&self) -> &dyn template::RouterGenerator {
        match self {
            FastAPI => &fastapi::RouterGenerator,
        }
    }

    fn to_service_calls_generator(&self) -> &dyn template::ServiceCallGenerator {
        match self {
            FastAPI => &fastapi::ServiceCallGenerator,
        }
    }

    fn to_main_generator(&self) -> &dyn template::MainGenerator {
        match self {
            FastAPI => &fastapi::MainGenerator,
        }
    }

    fn get_framework_requirements(&self) -> Vec<&'static str> {
        match self {
            FastAPI => fastapi::get_framework_dependencies(),
        }
    }

    fn get_docker_entrypoint(&self) -> &'static str {
        match self {
            FastAPI => fastapi::DOCKER_ENTRYPOINT,
        }
    }
}
