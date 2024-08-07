mod dependencies;
pub mod docker;
mod error;
mod faker;
mod import;
mod info;
mod main_file;
mod router;
mod service_calls;
mod writer;

pub use dependencies::DependencyData;
pub use error::{Error, Result};
pub use faker::{FakeFunction, Fakeable};
pub use import::Import;
pub use info::ServiceInfo;
pub(crate) use main_file::{AuxiliryTemplate, MainGenerator, MainTemplate};
pub(crate) use router::{create_router_file_data, RouterFileData, RouterGenerator, RouterTemplate};
pub(crate) use service_calls::{
    create_service_call_file_data, ServiceCallFileData, ServiceCallGenerator, ServiceCallTemplate,
};

pub use writer::{
    register_template_file, register_templates, write_dependency_file, write_docker_file,
    write_main_file, write_router_file, write_service_call_file,
};
