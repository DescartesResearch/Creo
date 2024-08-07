use crate::template;

pub const DOCKER_ENTRYPOINT: &str =
    r#"[ "uvicorn", "main:app", "--host", "0.0.0.0", "--port", "80" ]"#;

pub struct Faker;

impl template::Fakeable for Faker {
    fn get_string_fake(&self, string_validation: &openapiv3::StringType) -> template::FakeFunction {
        template::FakeFunction::new(
            "fake.pystr".into(),
            format!(
                "{}, {}",
                string_validation.min_length.unwrap_or_default(),
                string_validation.max_length.unwrap_or(20)
            ),
        )
    }

    fn get_number_fake(&self, number_validation: &openapiv3::NumberType) -> template::FakeFunction {
        template::FakeFunction::new(
            "fake.pyfloat".into(),
            format!(
                "min_value={}, max_value={}",
                number_validation.minimum.unwrap_or_default(),
                number_validation.maximum.unwrap_or(5000.0)
            ),
        )
    }

    fn get_integer_fake(
        &self,
        integer_validation: &openapiv3::IntegerType,
    ) -> template::FakeFunction {
        template::FakeFunction::new(
            "fake.pyint".into(),
            format!(
                "{}, {}",
                integer_validation.minimum.unwrap_or_default(),
                integer_validation.maximum.unwrap_or(9999)
            ),
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
        template::FakeFunction::new("fake.pybool".into(), String::new())
    }
}

pub struct RouterGenerator;

impl template::RouterGenerator for RouterGenerator {
    fn create_router_template(&self) -> template::RouterTemplate {
        template::RouterTemplate {
            template_dir: "python/fastapi/router",
            root_template_name: "router",
        }
    }
}

pub struct ServiceCallGenerator;

impl template::ServiceCallGenerator for ServiceCallGenerator {
    fn create_service_call_template(&self) -> template::ServiceCallTemplate {
        template::ServiceCallTemplate {
            template_dir: "python/fastapi/service_calls",
            root_template_name: "service_calls",
        }
    }
}

pub struct MainGenerator;

impl template::MainGenerator for MainGenerator {
    fn create_main_template(&self) -> template::MainTemplate {
        template::MainTemplate {
            template_dir: "python/fastapi",
            root_template_name: "main",
            auxiliry_template_names: &[template::AuxiliryTemplate {
                template_name: "http_client",
                file_name: "src/http_client.py",
            }],
        }
    }
}

pub fn get_framework_dependencies() -> Vec<&'static str> {
    vec![
        "fastapi==0.109.*",
        "uvicorn[standard]==0.26.*",
        "httpx==0.26.*",
        "Faker==22.2.*",
    ]
}
