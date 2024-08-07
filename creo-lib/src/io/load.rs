use crate::{
    application,
    graph::ApplicationGraph,
    handler,
    load::{self, LoadGeneratorFile},
};

pub fn create_load_generator_file(
    application: &ApplicationGraph,
    service: &application::MicroService,
    registry: &handler::FunctionRegistry,
    service_name: impl AsRef<str>,
) -> (load::LoadGeneratorFile, load::LoadGeneratorFile) {
    let mut user_requests = Vec::default();
    let mut is_user_front_end = Vec::default();
    for endpoint in application.iter_service_endpoints(service.id) {
        let handler = registry.get_function(endpoint.id);
        let user_request = load::UserRequest::new(
            handler,
            service_name.as_ref().into(),
            application.get_endpoint_path(endpoint.id),
        );
        user_requests.push(user_request);
        is_user_front_end.push(application.is_user_frontend(endpoint.id));
    }
    let load_service = load::LoadService::new(
        vec![format!("{{{{APPLICATION_HOST}}}}:{}", service.port)],
        service_name.as_ref().into(),
    );
    let services = vec![load_service];
    let user_file = load::LoadGeneratorFile {
        services: services.clone(),
        user_requests: user_requests
            .iter()
            .zip(is_user_front_end.iter())
            .filter(|(_, p)| **p)
            .map(|(ur, _)| ur)
            .cloned()
            .collect(),
    };
    let load_generator_file = load::LoadGeneratorFile {
        services,
        user_requests,
    };
    (load_generator_file, user_file)
}

pub fn write_load_generator_file(
    load_generator_file: &LoadGeneratorFile,
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    serde_yaml::to_writer(std::fs::File::create(path.as_ref())?, load_generator_file).map_err(
        |err| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "failed to write file for path {}!\n\tReason: {}",
                    path.as_ref().display(),
                    err
                ),
            )
        },
    )?;

    Ok(())
}

pub fn create_application_load_file(data: Vec<load::LoadGeneratorFile>) -> load::LoadGeneratorFile {
    let mut services = Vec::default();
    let mut user_requests = Vec::default();
    for file in data {
        services.extend(file.services);
        user_requests.extend(file.user_requests)
    }
    load::LoadGeneratorFile {
        services,
        user_requests,
    }
}
