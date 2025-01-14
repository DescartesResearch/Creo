use std::io::Write;

use crate::graph::ApplicationGraph;
use crate::load::http_loadgenerator;
use crate::{application, handler};

pub fn create_load_generator_file(
    application: &ApplicationGraph,
    service: &application::MicroService,
    registry: &handler::FunctionRegistry,
) -> (http_loadgenerator::Script, http_loadgenerator::Script) {
    let mut user_requests = Vec::default();
    let mut frontend_requests = Vec::default();

    for endpoint in application.iter_service_endpoints(service.id) {
        let handler = registry.get_function(endpoint.id);
        let user_request = http_loadgenerator::Request {
            endpoint_id: endpoint.id.0,
            service_id: service.id.0,
            path: application.get_endpoint_path(endpoint.id),
            data: handler.into(),
        };
        if application.is_user_frontend(endpoint.id) {
            frontend_requests.push(user_request.clone());
        }
        user_requests.push(user_request);
    }

    let load_service = http_loadgenerator::Service {
        id: service.id.0,
        url: format!("http://{{{{APPLICATION_HOST}}}}:{}", service.port),
    };
    let services = vec![load_service];

    let all = http_loadgenerator::Script {
        services: services.clone(),
        requests: user_requests,
    };
    let user_only = http_loadgenerator::Script {
        services,
        requests: frontend_requests,
    };

    (all, user_only)
}

pub fn write_load_generator_file(
    script: &http_loadgenerator::Script,
    path: impl AsRef<std::path::Path>,
) -> std::io::Result<()> {
    let mut file = std::fs::File::create(path.as_ref())?;
    file.write_all(script.as_lua_source().as_bytes())
}

pub fn create_application_load_file(
    data: Vec<http_loadgenerator::Script>,
) -> http_loadgenerator::Script {
    let mut services = Vec::default();
    let mut user_requests = Vec::default();
    for script in data {
        services.extend(script.services);
        user_requests.extend(script.requests);
    }
    http_loadgenerator::Script {
        services,
        requests: user_requests,
    }
}
