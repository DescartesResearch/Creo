use crate::generator::core;

pub struct SymbolGenerator;

fn capitalize_first(s: &str) -> String {
    let mut chars = s.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

impl core::SymbolGenerator for SymbolGenerator {
    fn generate_array_item_function_name(&self, name: &str) -> String {
        format!("{}Item", name)
    }

    fn generate_object_property_function_name(&self, name: &str, prop_name: &str) -> String {
        format!("{}Property{}", name.to_lowercase(), capitalize_first(prop_name))
    }

    fn generate_service_calls_function_name(
        &self,
        endpoint: crate::graph::EndpointIndex,
    ) -> String {
        format!("serviceCallsEndpoint{}", endpoint.0)
    }

    fn generate_service_call_function_import(
        &self,
        file_name: &str,
        function_name: &str,
    ) -> String {
        let split = file_name.rsplit_once('/');
        let file_name = if let Some(split) = split {
            split.1
        } else {
            file_name
        };
        format!("import {{ {} }} from './{}.js'", function_name, file_name)
    }

    fn generate_individual_service_call_function_name(
        &self,
        call: crate::application::ServiceCallEdge,
    ) -> String {
        format!(
            "serviceCallEndpoint{}ToEndpoint{}",
            call.source.0, call.target.0
        )
    }

    fn generate_operation_function_name(&self, endpoint: crate::graph::EndpointIndex) -> String {
        format!("operationEndpoint{}", endpoint.0)
    }

    fn generate_handler_function_import(&self, import_path: &str, function_name: &str) -> String {
        format!("import {{ {} }} from '{}'", function_name, import_path)
    }

    fn generate_query_data_function_name(
        &self,
        service_call: crate::application::ServiceCallEdge,
    ) -> String {
        format!(
            "queryDataCallEndpoint{}ToEndpoint{}",
            service_call.source.0, service_call.target.0
        )
    }

    fn generate_parameter_function_name(
        &self,
        service_call: crate::application::ServiceCallEdge,
        param_name: &str,
    ) -> String {
        format!(
            "callEndpoint{}ToEndpoint{}Param{}",
            service_call.source.0, service_call.target.0, param_name
        )
    }
}
