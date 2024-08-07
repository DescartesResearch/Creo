use std::collections::{HashMap, HashSet};

use crate::{
    application,
    dependencies::Dependency,
    graph::{ApplicationGraph, MicroServiceIndex},
    handler,
};

pub struct FunctionRegistry {
    funcs: HashMap<std::path::PathBuf, handler::Function>,
    paths: Vec<std::path::PathBuf>,
    dependencies: HashMap<std::path::PathBuf, HashSet<Dependency>>,
}

impl FunctionRegistry {
    pub fn new<'a>(
        endpoints: impl Iterator<Item = application::Endpoint<'a>>,
    ) -> std::io::Result<Self> {
        let mut unique_defs = HashSet::new();
        let mut paths = Vec::default();
        for endpoint in endpoints {
            unique_defs.insert(endpoint.handler_dir.clone());
            paths.push(endpoint.handler_dir.clone());
        }
        let mut funcs = HashMap::with_capacity(unique_defs.len());
        let mut dependencies = HashMap::with_capacity(unique_defs.len());
        for def in unique_defs {
            let func = crate::io::parse_handler_function(&def).map_err(|err| {
                std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    format!(
                        "invalid definition file at path {}! \n\tReason: {}",
                        def.display(),
                        err
                    ),
                )
            })?;
            dependencies.insert(def.clone(), HashSet::from_iter(func.depends_on.clone()));
            funcs.insert(def, func);
        }
        Ok(Self {
            funcs,
            paths,
            dependencies,
        })
    }
    pub fn get_function(&self, endpoint: crate::graph::EndpointIndex) -> &crate::handler::Function {
        &self.funcs[&self.paths[endpoint.0]]
    }

    pub fn get_service_dependencies(
        &self,
        application: &ApplicationGraph,
        service: MicroServiceIndex,
    ) -> HashSet<&Dependency> {
        application
            .iter_service_endpoints(service)
            .flat_map(|endpoint| &self.dependencies[&self.paths[endpoint.id.0]])
            .collect()
    }
}
