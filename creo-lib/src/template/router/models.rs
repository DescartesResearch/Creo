use indexmap::IndexSet;

use crate::{schema, template::import::Import};

#[derive(serde::Serialize, Debug)]
/// [`RouterFileData`] contains the data passed to the router file template.
pub struct RouterFileData {
    /// contains the import statements needed for data types, e.g. dates, maps, etc.
    pub type_imports: IndexSet<Import>,
    /// contains the import statements for the service call functions from the service call file.
    pub service_call_imports: IndexSet<Import>,
    /// contains the import statements for the handler function of each endpoint.
    pub handler_func_imports: IndexSet<Import>,
    /// contains all HTTP GET operations.
    pub http_get_operations: Vec<HTTPGetOperation>,
    /// contains all HTTP POST operations.
    pub http_post_operations: Vec<HTTPPostOperation>,
}

#[derive(serde::Serialize, Debug)]
/// [`HTTPPostOperation`] contains the information for a HTTP POST operation.
pub struct HTTPPostOperation {
    /// the function name of the operation.
    pub name: String,
    /// the optional description for the operation.
    pub description: Option<String>,
    /// the flag indicating whether the endpoint function is asynchronous or not.
    pub is_async: bool,
    /// the unique endpoint index
    pub endpoint_index: usize,
    /// the path string of the operation.
    pub path: String,
    /// the list of arguments to pass to the handler function.
    pub handler_args: Vec<Argument>,
    /// the name of the handler function, that is called by this operation.
    pub handler_func_name: String,
    /// the name of the handler function argument, that expects the body bytes string.
    pub body_param_name: String,
    /// the list of query parameters.
    pub query_params: Vec<QueryParameter>,
    /// the flag indicating whether this endpoint calls other endpoints in the generated
    /// application via service calls.
    pub has_service_calls: bool,
    /// the function name of the service call function. If this endpoint has no service calls, this
    /// will be an empty string.
    pub service_call_function_name: String,
    /// the flag indicating whether the handler function of this operation returns a value or not.
    pub has_return_type: bool,
}

#[derive(serde::Serialize, Debug)]
pub struct HTTPGetOperation {
    /// the function name of the operation.
    pub name: String,
    /// the optional description for the operation.
    pub description: Option<String>,
    /// the flag indicating whether the endpoint function is asynchronous or not.
    pub is_async: bool,
    /// the unique endpoint index
    pub endpoint_index: usize,
    /// the path string of the operation.
    pub path: String,
    /// the list of arguments to pass to the handler function.
    pub handler_args: Vec<Argument>,
    /// the name of the handler function, that is called by this operation.
    pub handler_func_name: String,
    /// the list of query parameters.
    pub query_params: Vec<QueryParameter>,
    /// the flag indicating whether this endpoint calls other endpoints in the generated
    /// application via service calls.
    pub has_service_calls: bool,
    /// the function name of the service call function. If this endpoint has no service calls, this
    /// will be an empty string.
    pub service_call_function_name: String,
    /// the flag indicating whether the handler function of this operation returns a value or not.
    pub has_return_type: bool,
}

#[derive(serde::Serialize, Debug)]
/// [`Argument`] represents a single input argument to a handler function.
pub struct Argument {
    /// the name of input argument.
    pub name: String,
    /// the flag indicating whether this argument is a positional or key-word argument.
    pub is_pos_arg: bool,
    /// the flag indicating whether this argument is included in the body
    pub is_body_arg: bool,
}

#[derive(serde::Serialize, Debug)]
/// [`QueryParameter`] represents a single query parameter.
pub struct QueryParameter {
    /// the name (or key) of the query parameter.
    name: String,
    /// the primitive data type of the query parameter.
    param_type: String,
    /// the description of the query parameter.
    description: String,
    /// the openapi string validation of the query parameter, if the parameter is of type string.
    string_validation: Option<openapiv3::StringType>,
    /// the openapi number validation of the query parameter, if the parameter is of type float or double.
    number_validation: Option<openapiv3::NumberType>,
    /// the openapi integer validation of the query parameter, if the parameter is of type integer.
    integer_validation: Option<openapiv3::IntegerType>,
}

impl QueryParameter {
    pub fn new(name: String, param_type: String, schema: &schema::Schema) -> Self {
        let description = if let Some(description) = schema.schema_data.description.as_ref() {
            description.clone()
        } else {
            format!("Query Parameter {}", name.clone())
        };

        Self {
            name,
            param_type,
            description,
            string_validation: match &schema.schema_kind {
                schema::SchemaKind::Type(type_schema) => match type_schema {
                    schema::Type::String(string_validation) => Some(string_validation.clone()),
                    _ => None,
                },
            },
            number_validation: match &schema.schema_kind {
                schema::SchemaKind::Type(type_schema) => match type_schema {
                    schema::Type::Number(number_validation) => Some(number_validation.clone()),
                    _ => None,
                },
            },
            integer_validation: match &schema.schema_kind {
                schema::SchemaKind::Type(type_schema) => match type_schema {
                    schema::Type::Integer(interger_validation) => Some(interger_validation.clone()),
                    _ => None,
                },
            },
        }
    }
}
