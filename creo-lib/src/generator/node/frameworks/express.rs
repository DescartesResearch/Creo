use crate::template;

pub const DOCKER_ENTRYPOINT: &str = r#"[ "node", "src/index.js" ]"#;

pub struct Faker;

impl template::Fakeable for Faker {
    fn get_string_fake(&self, string_validation: &openapiv3::StringType) -> template::FakeFunction {
        template::FakeFunction::new(
            "faker.string.alphanumeric".into(),
            format!(
                "{{ length: {{ min: {}, max: {} }} }}",
                string_validation.min_length.unwrap_or_default(),
                string_validation.max_length.unwrap_or(20)
            ),
        )
    }

    fn get_number_fake(&self, number_validation: &openapiv3::NumberType) -> template::FakeFunction {
        template::FakeFunction::new(
            "faker.number.float".into(),
            format!(
                " {{ min: {}, max: {} }}",
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
            "faker.number.int".into(),
            format!(
                " {{ min: {}, max: {} }}",
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
        template::FakeFunction::new("faker.datatype.boolean".into(), String::new())
    }
}

pub struct RouterGenerator;

impl template::RouterGenerator for RouterGenerator {
    fn create_router_template(&self) -> template::RouterTemplate {
        template::RouterTemplate {
            template_dir: "node/express/router",
            root_template_name: "router",
        }
    }
}

pub struct ServiceCallGenerator;

impl template::ServiceCallGenerator for ServiceCallGenerator {
    fn create_service_call_template(&self) -> template::ServiceCallTemplate {
        template::ServiceCallTemplate {
            template_dir: "node/express/service-calls",
            root_template_name: "service-calls",
        }
    }
}

pub struct MainGenerator;

impl template::MainGenerator for MainGenerator {
    fn create_main_template(&self) -> template::MainTemplate {
        template::MainTemplate {
            template_dir: "node/express",
            root_template_name: "index",
            auxiliry_template_names: &[]
        }
    }
}

pub fn get_framework_dependencies() -> Vec<&'static str> {
    vec![
        r#""express": "^5""#,
        r#""@faker-js/faker": "^9""#,
        r#""body-parser": "^2""#,
        r#""joi": "^17""#,
    ]
}
