use crate::template;

pub const DOCKER_ENTRYPOINT: &str = r#"["myapp"]"#;

pub struct Faker;

impl template::Fakeable for Faker {
    fn get_string_fake(&self, string_validation: &openapiv3::StringType) -> template::FakeFunction {
        let min_length = string_validation.min_length.unwrap_or(0);
        let max_length = string_validation.max_length.unwrap_or(20);

        template::FakeFunction::new(
            "get_random_string".into(),
            format!("{}, {}", min_length, max_length),
        )
    }

    fn get_number_fake(&self, number_validation: &openapiv3::NumberType) -> template::FakeFunction {
        let min_value = number_validation.minimum.unwrap_or_default();
        let max_value = number_validation.maximum.unwrap_or(5000.0);
        template::FakeFunction::new(
            "get_random_numeric_value".into(),
            format!("{}, {}", min_value, max_value),
        )
    }

    fn get_integer_fake(
        &self,
        integer_validation: &openapiv3::IntegerType,
    ) -> template::FakeFunction {
        let min_value = integer_validation.minimum.unwrap_or_default();
        let max_value = integer_validation.maximum.unwrap_or(9999);
        template::FakeFunction::new(
            "get_random_numeric_value".into(),
            format!("{}, {}", min_value, max_value),
        )
    }

    fn get_object_fake(&self, function_name: &str) -> template::FakeFunction {
        template::FakeFunction::new(function_name.into(), String::new())
    }

    fn get_array_fake(&self, function_name: &str) -> template::FakeFunction {
        template::FakeFunction::new(function_name.into(), String::new())
    }

    fn get_boolean_fake(
        &self,
        _boolean_validation: &openapiv3::BooleanType,
    ) -> template::FakeFunction {
        template::FakeFunction::new("get_random_bool".into(), String::new())
    }
}

pub struct RouterGenerator;

impl template::RouterGenerator for RouterGenerator {
    fn create_router_template(&self) -> template::RouterTemplate {
        template::RouterTemplate {
            template_dir: "rust/axum/router",
            root_template_name: "router",
        }
    }
}

pub struct ServiceCallGenerator;

impl template::ServiceCallGenerator for ServiceCallGenerator {
    fn create_service_call_template(&self) -> template::ServiceCallTemplate {
        template::ServiceCallTemplate {
            template_dir: "rust/axum/service_calls",
            root_template_name: "service_calls",
        }
    }
}

pub struct MainGenerator;

impl template::MainGenerator for MainGenerator {
    fn create_main_template(&self) -> template::MainTemplate {
        template::MainTemplate {
            template_dir: "rust/axum",
            root_template_name: "main",
            auxiliry_template_names: &[],
        }
    }
}

pub fn get_framework_dependencies() -> Vec<&'static str> {
    vec![
        r#"axum = {version = "0.6.20", features = ["headers"]}"#,
        r#"serde = { version = "1.0", features = ["derive"] }"#,
        r#"tokio = { version = "1", features = ["full"] }"#,
        r#"hyper = { version = "0.14", features = ["client"] }"#,
        r#"rand = "0.8.5""#,
        r#"serde_json = "1.0""#,
        r#"serde_urlencoded = "0.7.1""#,
        r#"serde_valid = "0.24.0""#,
    ]
}
