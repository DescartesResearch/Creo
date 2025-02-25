use crate::generator::core;

pub struct SymbolGenerator;

impl core::SymbolGenerator for SymbolGenerator {
    fn generate_array_item_function_name(&self, name: &str) -> String {
        format!("{}_item", name)
    }

    fn generate_object_property_function_name(&self, name: &str, prop_name: &str) -> String {
        format!("{}_prop_{}", name, prop_name)
    }

    fn generate_service_calls_function_name(
        &self,
        endpoint: crate::graph::EndpointIndex,
    ) -> String {
        format!("service_calls_endpoint_{}", endpoint.0)
    }

    fn generate_service_call_function_import(
        &self,
        file_path: &str,
        function_name: &str,
    ) -> String {
        let split = file_path.rsplit_once('/');
        let file_name = if let Some(split) = split {
            split.1
        } else {
            file_path
        };
        format!("from {} import {}", file_name, function_name)
    }

    fn generate_individual_service_call_function_name(
        &self,
        call: crate::application::ServiceCallEdge,
    ) -> String {
        format!(
            "service_call_endpoint_{}_to_endpoint_{}",
            call.source.0, call.target.0
        )
    }

    fn generate_operation_function_name(&self, endpoint: crate::graph::EndpointIndex) -> String {
        format!("operation_endpoint_{}", endpoint.0)
    }

    fn generate_handler_function_import(&self, import_path: &str, function_name: &str) -> String {
        format!("from {} import {}", import_path, function_name)
    }

    fn generate_query_data_function_name(
        &self,
        service_call: crate::application::ServiceCallEdge,
    ) -> String {
        format!(
            "query_data_call_endpoint_{}_to_endpoint_{}",
            service_call.source.0, service_call.target.0
        )
    }

    fn generate_parameter_function_name(
        &self,
        service_call: crate::application::ServiceCallEdge,
        param_name: &str,
    ) -> String {
        format!(
            "call_endpoint_{}_to_endpoint_{}_param_{}",
            service_call.source.0, service_call.target.0, param_name
        )
    }
}
