use std::collections::HashSet;

use crate::{
    application::{self, get_host},
    graph, handler,
};
use docker_compose_types as dct;
use indexmap::IndexMap;

pub fn create_service_compose_with_build(
    application: &graph::ApplicationGraph,
    service: &application::MicroService,
    registry: &handler::FunctionRegistry,
    app_name: impl AsRef<str>,
    timestamp_tag: i64,
) -> super::Compose {
    let mut services = IndexMap::new();
    let mut environment = Vec::default();
    let mut volumes = Vec::default();
    let service_host = get_host(service.id);
    let mut depends_on = Vec::default();
    let mut seen_deps_types = HashSet::new();
    let mut seen_init_services = HashSet::new();
    for dependency in registry.get_service_dependencies(application, service.id) {
        let dc_service = dependency.name.as_docker_compose_service(&service_host);
        if let Some(init) = &dependency.init {
            if seen_init_services.insert(init) {
                let init_service = dct::Service {
                    image: Some(format!(
                        "{}-{}-{}:{}",
                        app_name.as_ref(),
                        &dc_service.0,
                        init,
                        timestamp_tag
                    )),
                    build_: Some(dct::BuildStep::Simple(format!("./init-services/{}", init))),
                    profiles: vec!["init".into()],
                    environment: {
                        let mut env = vec!["MG_SEED_COUNT=$MG_SEED_COUNT".into()];
                        env.extend(dependency.name.as_docker_compose_environment(&service_host));
                        dct::Environment::List(env)
                    },
                    depends_on: dct::DependsOnOptions::Simple(vec![dc_service.0.clone()]),
                    ..Default::default()
                };
                services.insert(format!("{}-{}", dc_service.0, init), Some(init_service));
            }
        }
        if seen_deps_types.insert(&dependency.name) {
            services.insert(dc_service.0.clone(), Some(dc_service.1));
            depends_on.push(dc_service.0.clone());
            environment.extend(dependency.name.as_docker_compose_environment(&service_host));
            if let Some(volume_name) = dependency.name.as_volume_name(&service_host) {
                volumes.push(volume_name)
            }
        }
    }

    let mut call_services = IndexMap::new();
    for endpoint in application.iter_service_endpoints(service.id) {
        for service_call in application.iter_service_calls(endpoint.id) {
            let call_service = application.get_service(service_call.target);
            call_services.insert(
                application.get_host_env_var(call_service),
                get_host(call_service),
            );
        }
    }
    environment.extend(call_services.iter().map(|(k, v)| format!("{}={}", k, v)));

    let image_name = format!("{}-{}:{}", app_name.as_ref(), service_host, timestamp_tag);
    dct::Compose {
        services: {
            services.insert(
                service_host,
                Some(dct::Service {
                    build_: Some(dct::BuildStep::Simple("./".into())),
                    image: Some(image_name),
                    environment: dct::Environment::List(environment),
                    ports: dct::Ports::Short(vec![format!("{}:80", service.port)]),
                    restart: Some("unless-stopped".into()),
                    depends_on: dct::DependsOnOptions::Simple(depends_on),
                    ..Default::default()
                }),
            );
            dct::Services(services)
        },
        volumes: dct::TopLevelVolumes(IndexMap::from_iter(
            volumes
                .into_iter()
                .map(|name| (name, dct::MapOrEmpty::Empty)),
        )),
        ..Default::default()
    }
    .into()
}

pub fn create_service_compose_with_image(
    compose: &mut super::Compose,
    service: &application::MicroService,
    service_name: impl AsRef<str>,
) {
    let mut dcs = compose
        .0
        .services
        .0
        .get(&get_host(service.id))
        .expect("service in compose")
        .clone()
        .expect("service");
    dcs.build_ = None;
    dcs.image = Some(format!("{}:v1.0.0", service_name.as_ref()));
    compose.0.services.0.insert(get_host(service.id), Some(dcs));
}
