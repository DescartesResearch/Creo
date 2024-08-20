use std::collections::HashSet;

use crate::{
    generator::core::SymbolGenerator,
    graph::{ApplicationGraph, MicroServiceIndex},
    handler,
    http_method::HTTPMethod,
    schema,
};

use super::faker::{FakeFunction, Fakeable};

pub(super) mod models;
pub use models::ServiceCallFileData;

fn to_fake_function(
    name: &str,
    schema_kind: &schema::SchemaKind,
    faker: &dyn Fakeable,
) -> FakeFunction {
    match schema_kind {
        schema::SchemaKind::Type(type_schema) => match type_schema {
            schema::Type::String(string_validation) => faker.get_string_fake(string_validation),
            schema::Type::Number(number_validation) => faker.get_number_fake(number_validation),
            schema::Type::Integer(integer_validation) => faker.get_integer_fake(integer_validation),
            schema::Type::Object(_) => faker.get_object_fake(name),
            schema::Type::Array(_) => faker.get_array_fake(name),
            schema::Type::Boolean(boolean_validation) => faker.get_boolean_fake(boolean_validation),
        },
    }
}

fn create_array_fake_function(
    name: String,
    array_type: &schema::ArrayType,
    object_funcs: &mut Vec<models::ObjectFakeFunction>,
    array_funcs: &mut Vec<models::ArrayFakeFunction>,
    symbol_generator: &dyn SymbolGenerator,
    faker: &dyn Fakeable,
) -> models::ArrayFakeFunction {
    let item_name = symbol_generator.generate_array_item_function_name(&name);
    if let Some(object_type) = array_type.items.get_object_schema() {
        let obj_fn = create_object_fake_function(
            item_name.clone(),
            object_type,
            object_funcs,
            array_funcs,
            symbol_generator,
            faker,
        );
        object_funcs.push(obj_fn);
    }

    if let Some(array_type) = array_type.items.get_array_schema_type() {
        let arr_fn = create_array_fake_function(
            item_name.clone(),
            array_type,
            object_funcs,
            array_funcs,
            symbol_generator,
            faker,
        );
        array_funcs.push(arr_fn);
    }

    let fake_func = to_fake_function(&item_name, &array_type.items.schema_kind, faker);
    models::ArrayFakeFunction::new(
        name,
        fake_func,
        array_type.min_items.unwrap_or_default(),
        array_type.max_items.unwrap_or(30),
    )
}

fn create_query_data_function(
    name: String,
    parameters: &[handler::Param],
    faker: &dyn Fakeable,
) -> models::QueryDataFunction {
    let mut params: Vec<models::QueryParamFakeFunction> = Vec::default();
    for param in parameters {
        let param_name = param.as_name();
        let fake_func = to_fake_function("", &param.schema.schema_kind, faker);
        params.push(models::QueryParamFakeFunction {
            name: param_name,
            fake_func,
            nullable: param.schema.schema_data.nullable,
            exclude_probability: crate::constants::DEFAULT_EXCLUDE_PROBABILITY,
        });
    }

    models::QueryDataFunction { name, params }
}

fn create_object_fake_function(
    name: String,
    object_type: &schema::ObjectType,
    object_funcs: &mut Vec<models::ObjectFakeFunction>,
    array_funcs: &mut Vec<models::ArrayFakeFunction>,
    symbol_generator: &dyn SymbolGenerator,
    faker: &dyn Fakeable,
) -> models::ObjectFakeFunction {
    let mut props: Vec<models::PropertyFakeFunction> =
        Vec::with_capacity(object_type.properties.len());
    let required_prop_names: HashSet<&String> = HashSet::from_iter(&object_type.required);
    for (prop_name, prop) in &object_type.properties {
        let prop_fake_name =
            symbol_generator.generate_object_property_function_name(&name, prop_name);
        if let Some(object_type) = prop.get_object_schema() {
            let obj_fn = create_object_fake_function(
                prop_fake_name.clone(),
                object_type,
                object_funcs,
                array_funcs,
                symbol_generator,
                faker,
            );
            object_funcs.push(obj_fn);
        }

        if let Some(array_type) = prop.get_array_schema_type() {
            let arr_fn = create_array_fake_function(
                prop_fake_name.clone(),
                array_type,
                object_funcs,
                array_funcs,
                symbol_generator,
                faker,
            );
            array_funcs.push(arr_fn);
        }

        let fake_func = to_fake_function(&prop_fake_name, &prop.schema_kind, faker);
        props.push(models::PropertyFakeFunction {
            name: prop_name.to_string(),
            fake_func,
            required: required_prop_names.contains(prop_name),
            exclude_probability: crate::constants::DEFAULT_EXCLUDE_PROBABILITY,
        })
    }

    models::ObjectFakeFunction { name, props }
}

pub(crate) fn create_service_call_file_data(
    service: MicroServiceIndex,
    graph: &ApplicationGraph,
    registry: &handler::FunctionRegistry,
    faker: &dyn Fakeable,
    symbol_generator: &dyn SymbolGenerator,
) -> models::ServiceCallFileData {
    let mut object_fake_functions: Vec<models::ObjectFakeFunction> = Vec::default();
    let mut array_fake_functions: Vec<models::ArrayFakeFunction> = Vec::default();
    let mut query_data_functions: Vec<models::QueryDataFunction> = Vec::default();
    let mut service_call_functions: Vec<models::ServiceCallFunction> = Vec::default();

    for endpoint in graph.iter_service_endpoints(service) {
        let function_name = symbol_generator.generate_service_calls_function_name(endpoint.id);
        let mut service_call_function = models::ServiceCallFunction::new(function_name.clone());
        let mut did_iter = false;
        for call in graph.iter_service_calls(endpoint.id) {
            did_iter = true;
            let handler_func = registry.get_function(call.target);
            let path = graph.get_endpoint_path(call.target);
            let host_env_var = graph.get_host_env_var(graph.get_service(call.target));
            let method = handler_func.get_http_method();
            let function_name =
                symbol_generator.generate_individual_service_call_function_name(call);

            match method {
                HTTPMethod::Get => {
                    if handler_func.signature.parameters.is_empty() {
                        service_call_function
                            .get_service_calls
                            .push(models::GetServiceCall {
                                name: function_name,
                                requires_data: false,
                                query_data_func: String::new(),
                                path,
                                host_env_var,
                            })
                    } else {
                        let query_func_name =
                            symbol_generator.generate_query_data_function_name(call);
                        let query_func = create_query_data_function(
                            query_func_name.clone(),
                            &handler_func.signature.parameters,
                            faker,
                        );

                        query_data_functions.push(query_func);
                        service_call_function
                            .get_service_calls
                            .push(models::GetServiceCall {
                                name: function_name,
                                requires_data: true,
                                query_data_func: query_func_name,
                                path,
                                host_env_var,
                            });
                    }
                }
                HTTPMethod::Post => {
                    let query_func_name = symbol_generator.generate_query_data_function_name(call);
                    let primitive_params: Vec<handler::Param> = handler_func
                        .signature
                        .parameters
                        .iter()
                        .filter(|p| p.is_primitive_type())
                        .cloned()
                        .collect();
                    let query_func =
                        create_query_data_function(query_func_name, &primitive_params, faker);
                    for param in &handler_func.signature.parameters {
                        if let Some(object_type) = param.schema.get_object_schema() {
                            let unique_object_func_name = symbol_generator
                                .generate_parameter_function_name(call, &param.as_name());

                            let body_func = create_object_fake_function(
                                unique_object_func_name,
                                object_type,
                                &mut object_fake_functions,
                                &mut array_fake_functions,
                                symbol_generator,
                                faker,
                            );
                            service_call_function.post_service_calls.push(
                                models::PostServiceCall {
                                    name: function_name,
                                    requires_query_data: !query_func.params.is_empty(),
                                    query_data_func: {
                                        if !query_func.params.is_empty() {
                                            query_func.name.clone()
                                        } else {
                                            String::new()
                                        }
                                    },
                                    body_data_func: body_func.name.clone(),
                                    path,
                                    host_env_var,
                                },
                            );
                            object_fake_functions.push(body_func);
                            break;
                        }

                        if let Some(array_type) = param.schema.get_array_schema_type() {
                            let unique_array_func_name = symbol_generator
                                .generate_parameter_function_name(call, &param.as_name());

                            let body_func = create_array_fake_function(
                                unique_array_func_name,
                                array_type,
                                &mut object_fake_functions,
                                &mut array_fake_functions,
                                symbol_generator,
                                faker,
                            );
                            service_call_function.post_service_calls.push(
                                models::PostServiceCall {
                                    name: function_name,
                                    requires_query_data: !query_func.params.is_empty(),
                                    query_data_func: {
                                        if !query_func.params.is_empty() {
                                            query_func.name.clone()
                                        } else {
                                            String::new()
                                        }
                                    },
                                    body_data_func: body_func.name.clone(),
                                    path,
                                    host_env_var,
                                },
                            );
                            array_fake_functions.push(body_func);
                            break;
                        }
                    }
                    query_data_functions.push(query_func);
                }
            }
        }

        if did_iter {
            service_call_functions.push(service_call_function);
        }
    }

    models::ServiceCallFileData {
        object_fake_functions,
        array_fake_functions,
        query_data_functions,
        service_call_functions,
    }
}

pub trait ServiceCallGenerator {
    fn create_service_call_template(&self) -> ServiceCallTemplate;
}

pub struct ServiceCallTemplate {
    pub template_dir: &'static str,
    pub root_template_name: &'static str,
}
