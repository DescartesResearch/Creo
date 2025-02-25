use crate::{application::ServiceCallEdge, graph::EndpointIndex};

/// [`SymbolGenerator`] enables the generation of various dynamic names used during the microservice
/// source code generation.
pub(crate) trait SymbolGenerator {
    /// Generates a unique function name for the given array name.
    ///
    /// This function can assume that the given array name is unique and thus a unique function
    /// name can be derived of it. The implementation should simply create a string following the
    /// programming language's function naming convention, e.g. "{name}_array_item" for languages
    /// following a snake_case function naming convention.
    fn generate_array_item_function_name(&self, name: &str) -> String;

    /// Generates a unique function name for the given object and property name.
    ///
    /// This function can assume that the given object name in combination with the property name
    /// is unique and thus a unique function name can be derived of it. The implementation should
    /// simply create a string following the programming language's function naming convention,
    /// e.g. "{name}_property_{prop_name}" for languages following a snake_case function naming
    /// convention.
    fn generate_object_property_function_name(&self, name: &str, prop_name: &str) -> String;

    /// Generates a unique function name based on the given endpoint.
    ///
    /// The returned function name will be used for the function calling all other service
    /// endpoints for the given endpoint. The implementation should return a string following the
    /// programming language's function naming convention, e.g. "service_calls_endpoint_{endpoint.0}"
    /// for languages following a snake_case function naming convention.
    fn generate_service_calls_function_name(&self, endpoint: EndpointIndex) -> String;

    /// Generates the function import statement for the given service call function name.
    ///
    /// The [`crate::generator::core::FileName::path`] returned by the language-specific
    /// implementation of the the
    /// [`crate::generator::core::FileNameGenerator::generate_service_call_file_name`] is passed as
    /// the `file_path` argument. In general, this will be `src/[file_name]` without the
    /// language-specific extension.
    ///
    /// # Arguments
    ///
    /// - *file_path* the file path of the service call file relative to the microservice root
    ///   directory
    /// - *function_name* the function name to import from the service call file
    fn generate_service_call_function_import(&self, file_path: &str, function_name: &str)
        -> String;

    /// Generates the function import statement for the given handler function.
    ///
    /// # Arguments
    ///
    /// - *import_path* the import path of the module/file, in which the handler function is
    ///   defined
    /// - *function_name* the name of the handler function
    fn generate_handler_function_import(&self, import_path: &str, function_name: &str) -> String;

    /// Generates the unique function name for the given, individual service call.
    ///
    /// The returned function name will be used for the function calling the target endpoint of the
    /// service call for the given source endpoint. The implementation should return a string following
    /// the programming language's function naming convention, e.g.
    /// "service_call_from_endpoint_{call.source}_to_endpoint_{call.target}" for languages following a
    /// snake_case function naming convention.
    fn generate_individual_service_call_function_name(&self, call: ServiceCallEdge) -> String;

    /// Generates the unique function name for the given endpoint.
    ///
    /// The returned function name will be used for the operation function of the given endpoint.
    fn generate_operation_function_name(&self, endpoint: EndpointIndex) -> String;

    /// Generates the unique function name for the query data of the given service call.
    ///
    /// The returned function name will be used for the function generating the required query data
    /// for the given service call. The implementation should return a string following the
    /// programming language's function naming convention, e.g.
    /// "query_data_for_service_call_from_endpoint_{service_call.source}_to_endpoint_{service_call.target}"
    /// for languages following a snake_case naming convention.
    fn generate_query_data_function_name(&self, service_call: ServiceCallEdge) -> String;

    /// Generates the unique function name for a complex-typed paramater of the given service call.
    ///
    /// The implementation should return a string following the programming language's function
    /// naming convention, e.g.
    /// "service_call_from_endpoint_{service_call.source}_to_endpoint_{service_call.target}_parameter_{param_name}"
    /// for languages following a snake_case naming convention.
    fn generate_parameter_function_name(
        &self,
        service_call: ServiceCallEdge,
        param_name: &str,
    ) -> String;
}
