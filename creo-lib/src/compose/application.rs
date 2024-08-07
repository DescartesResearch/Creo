use indexmap::IndexMap;

pub fn create_application_compose(
    service_composes: Vec<(String, super::Compose)>,
) -> super::Compose {
    let mut services = IndexMap::new();
    let mut volumes = IndexMap::new();
    docker_compose_types::Compose {
        volumes: {
            for (_, service_compose) in &service_composes {
                volumes.extend(service_compose.0.volumes.0.clone())
            }
            docker_compose_types::TopLevelVolumes(volumes)
        },
        services: {
            for (dir_name, service_compose) in service_composes {
                for (service_name, mut service) in service_compose.0.services.0 {
                    if let Some(ref mut service) = service {
                        if let Some(service_build) = &service.build_ {
                            let build = match service_build {
                                docker_compose_types::BuildStep::Simple(step) => {
                                    let step = step.strip_prefix(".").unwrap_or_else(|| step);
                                    let step = step.strip_prefix("/").unwrap_or(step);
                                    docker_compose_types::BuildStep::Simple(format!(
                                        "{}/{}",
                                        dir_name, step
                                    ))
                                }
                                docker_compose_types::BuildStep::Advanced(step) => {
                                    let mut step = step.clone();
                                    step.context = step
                                        .context
                                        .strip_prefix(".")
                                        .unwrap_or_else(|| &step.context)
                                        .into();
                                    step.context = step
                                        .context
                                        .strip_prefix("/")
                                        .unwrap_or_else(|| &step.context)
                                        .into();
                                    docker_compose_types::BuildStep::Advanced(step)
                                }
                            };
                            service.build_ = Some(build)
                        }
                    }
                    services.insert(service_name, service);
                }
            }
            docker_compose_types::Services(services)
        },
        ..Default::default()
    }
    .into()
}
