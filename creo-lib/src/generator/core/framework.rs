use crate::template;

pub trait FrameworkGenerator {
    fn to_faker(&self) -> &dyn template::Fakeable;
    fn to_router_generator(&self) -> &dyn template::RouterGenerator;
    fn to_service_calls_generator(&self) -> &dyn template::ServiceCallGenerator;
    fn to_main_generator(&self) -> &dyn template::MainGenerator;
    fn get_framework_requirements(&self) -> Vec<&'static str>;
    fn get_docker_entrypoint(&self) -> &'static str;
}
