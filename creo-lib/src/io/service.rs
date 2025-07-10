use std::collections::HashSet;

use crate::{
    application::{self, get_host},
    graph, handler, template,
};

pub fn create_service_folder<R: rand::Rng>(
    application_dir: impl AsRef<std::path::Path>,
    service_name: impl AsRef<str>,
    template_dir: impl AsRef<std::path::Path>,
    service: &application::MicroService,
    application: &graph::ApplicationGraph,
    registry: &handler::FunctionRegistry,
    rng: &mut R,
) -> std::io::Result<()> {
    let service_dir = application_dir.as_ref().join(service_name.as_ref());
    crate::io::create_dir_all(&service_dir)?;

    let data_type_mapper = service.language.as_data_type_mapper();
    let symbol_generator = service.language.as_symbol_generator();
    let file_name_generator = service.language.as_file_name_generator();
    let framework = service.language.choose_random_framework(rng);
    let faker = framework.to_faker();

    // Router File
    let router_data = template::create_router_file_data(
        service.id,
        application,
        registry,
        symbol_generator,
        data_type_mapper,
        file_name_generator.generate_service_call_file_name().path,
    );
    let router_template = framework.to_router_generator().create_router_template();
    template::write_router_file(
        &service_dir,
        &file_name_generator
            .generate_router_file_name()
            .as_complete_file_name(),
        &template_dir,
        router_data,
        router_template,
    )?;

    // Service Call File
    if application
        .iter_service_endpoints(service.id)
        .any(|endpoint| application.iter_service_calls(endpoint.id).next().is_some())
    {
        let service_call_data = template::create_service_call_file_data(
            service.id,
            application,
            registry,
            faker,
            symbol_generator,
        );
        let service_call_template = framework
            .to_service_calls_generator()
            .create_service_call_template();
        template::write_service_call_file(
            &service_dir,
            &file_name_generator
                .generate_service_call_file_name()
                .as_complete_file_name(),
            &template_dir,
            service_call_data,
            service_call_template,
        )?;
    }
    let info = template::ServiceInfo {
        title: format!("Service {}", service.id.0),
        description: String::new(),
        version: "1.0.0".into(),
        contact: openapiv3::Contact {
            name: Some("Creo".into()),
            url: Some("https://github.com/DescartesResearch/creo".into()),
            email: Some("yannik.lubas@uni-wuerzburg.de".into()),
            extensions: Default::default(),
        },
    };
    template::write_main_file(
        &service_dir,
        &file_name_generator
            .generate_main_file_name()
            .as_complete_file_name(),
        &template_dir,
        info,
        framework.to_main_generator().create_main_template(),
    )?;

    let lib_dir = service_dir.join("lib");
    crate::io::create_dir_all(&lib_dir)?;
    let mut unique_handler_dirs = HashSet::new();
    for endpoint in application.iter_service_endpoints(service.id) {
        if unique_handler_dirs.insert(endpoint.handler_dir) {
            crate::io::copy_dir_all(
                endpoint.handler_dir,
                lib_dir.join(
                    endpoint
                        .handler_dir
                        .file_name()
                        .expect("should have a dir name"),
                ),
            )?;
        }
    }

    let docker_file = template::docker::DockerfileData {
        entrypoint: framework.get_docker_entrypoint(),
    };
    template::write_docker_file(
        &service_dir,
        template_dir
            .as_ref()
            .join(service.language.as_docker_template_path()),
        docker_file,
    )?;

    let mut dependencies: Vec<String> = framework
        .get_framework_requirements()
        .into_iter()
        .map(|s| s.into())
        .collect();
    dependencies.extend(service.language.get_local_handler_dependencies(&lib_dir)?);
    let dependency_file_data = template::DependencyData {
        service_name: &get_host(service.id),
        dependencies,
    };

    template::write_dependency_file(
        &service_dir,
        template_dir
            .as_ref()
            .join(service.language.as_dependency_file_template_path()),
        service.language.dependency_file_name(),
        dependency_file_data,
    )?;

    Ok(())
}
