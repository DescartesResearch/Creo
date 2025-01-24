use creo_lib::handler;

use crate::{Error, Result};

pub fn create_handler_function_registry(
    app: &creo_lib::graph::ApplicationGraph,
) -> Result<handler::FunctionRegistry> {
    creo_lib::handler::FunctionRegistry::new(app.iter_endpoints()).map_err(|err| {
        Error::new(format!(
            "failed to parse handler function!\n\tReason: {}",
            err
        ))
    })
}
