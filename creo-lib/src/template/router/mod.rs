use indexmap::IndexSet;

use crate::generator::core::{DataTypeMapper, SymbolGenerator};
use crate::graph::{ApplicationGraph, MicroServiceIndex};
use crate::{generator, handler};

use super::Import;

mod models;
pub use models::RouterFileData;

pub(crate) fn create_router_file_data(
    service: MicroServiceIndex,
    graph: &ApplicationGraph,
    registry: &handler::FunctionRegistry,
    symbol_generator: &dyn SymbolGenerator,
    data_type_mapper: &dyn DataTypeMapper,
    service_call_file_name: &str,
) -> models::RouterFileData {
    let mut service_call_imports: IndexSet<Import> = IndexSet::default();
    let mut handler_function_imports: IndexSet<Import> = IndexSet::default();
    let mut http_get_operations: Vec<models::HTTPGetOperation> = Vec::default();
    let mut http_post_operations: Vec<models::HTTPPostOperation> = Vec::default();
    let mut type_imports: IndexSet<Import> = IndexSet::default();

    for endpoint in graph.iter_service_endpoints(service) {
        let handler_func = registry.get_function(endpoint.id);
        handler_function_imports.insert(Import {
            import: symbol_generator.generate_handler_function_import(
                &handler_func.import_path,
                &handler_func.signature.function,
            ),
        });
        let name = symbol_generator.generate_operation_function_name(endpoint.id);
        let path = graph.get_endpoint_path(endpoint.id);
        let n_args = handler_func.signature.parameters.len();
        let mut query_params: Vec<models::QueryParameter> = Vec::with_capacity(n_args);
        let mut handler_args: Vec<models::Argument> = Vec::with_capacity(n_args);
        let mut body_param_name: Option<String> = None;
        for param in &handler_func.signature.parameters {
            handler_args.push(models::Argument {
                name: param.as_name(),
                is_pos_arg: match param.arg {
                    handler::PassingType::Kw(_) => false,
                    handler::PassingType::Pos(_) => true,
                },
                is_body_arg: !param.is_primitive_type(),
            });

            if param.schema.get_object_schema().is_some()
                || param.schema.get_array_schema_type().is_some()
            {
                body_param_name = Some(param.as_name())
            } else {
                let param_type = generator::core::to_data_type(
                    &param.schema.schema_kind,
                    &mut type_imports,
                    data_type_mapper,
                );
                query_params.push(models::QueryParameter::new(
                    param.as_name(),
                    param_type,
                    &param.schema,
                ));
            }
        }

        let has_service_calls = graph.iter_service_calls(endpoint.id).next().is_some();
        let service_call_function_name = if has_service_calls {
            symbol_generator.generate_service_calls_function_name(endpoint.id)
        } else {
            String::new()
        };
        if has_service_calls {
            service_call_imports.insert(Import {
                import: symbol_generator.generate_service_call_function_import(
                    service_call_file_name,
                    &service_call_function_name,
                ),
            });
        }
        if let Some(body_param_name) = body_param_name {
            http_post_operations.push(models::HTTPPostOperation {
                name,
                description: handler_func.description.clone(),
                is_async: handler_func.is_async,
                endpoint_index: endpoint.id.0,
                path,
                handler_args,
                handler_func_name: handler_func.signature.function.clone(),
                body_param_name,
                query_params,
                has_service_calls,
                service_call_function_name,
                has_return_type: handler_func.returns,
            });
        } else {
            http_get_operations.push(models::HTTPGetOperation {
                name,
                description: handler_func.description.clone(),
                is_async: handler_func.is_async,
                endpoint_index: endpoint.id.0,
                path,
                handler_args,
                handler_func_name: handler_func.signature.function.clone(),
                query_params,
                has_service_calls,
                service_call_function_name,
                has_return_type: handler_func.returns,
            });
        }
    }

    models::RouterFileData {
        service_call_imports,
        handler_func_imports: handler_function_imports,
        type_imports,
        http_get_operations,
        http_post_operations,
    }
}

pub trait RouterGenerator {
    fn create_router_template(&self) -> RouterTemplate;
}

pub struct RouterTemplate {
    pub template_dir: &'static str,
    pub root_template_name: &'static str,
}
