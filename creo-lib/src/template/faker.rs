#[derive(serde::Serialize, Debug)]
pub struct FakeFunction {
    name: String,
    args: String,
}

impl FakeFunction {
    pub fn new(name: String, args: String) -> Self {
        Self { name, args }
    }
}

pub trait Fakeable {
    fn get_string_fake(&self, string_validation: &openapiv3::StringType) -> FakeFunction;

    fn get_number_fake(&self, number_validation: &openapiv3::NumberType) -> FakeFunction;

    fn get_integer_fake(&self, integer_validation: &openapiv3::IntegerType) -> FakeFunction;

    fn get_object_fake(&self, function_name: &str) -> FakeFunction;

    fn get_array_fake(&self, function_name: &str) -> FakeFunction;

    fn get_boolean_fake(&self, boolean_validation: &openapiv3::BooleanType) -> FakeFunction;
}
