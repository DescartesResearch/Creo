use crate::template::faker::FakeFunction;

/// [`ServiceCallFileData`] represents the dynamic information of the source code file comprising the
/// individual service calls.
#[derive(serde::Serialize, Debug)]
/// all object fake functions
pub struct ServiceCallFileData {
    pub object_fake_functions: Vec<ObjectFakeFunction>,
    /// all array fake functions
    pub array_fake_functions: Vec<ArrayFakeFunction>,
    /// all data functions
    pub query_data_functions: Vec<QueryDataFunction>,
    /// all service call functions
    pub service_call_functions: Vec<ServiceCallFunction>,
}

/// [`ObjectFakeFunction`] represents the dynamic information for an object fake function.
#[derive(serde::Serialize, Debug)]
pub struct ObjectFakeFunction {
    /// the name of the fake function
    pub name: String,
    /// the name and corresponding fake function names of every property of the object
    pub props: Vec<PropertyFakeFunction>,
}

/// [`PropertyFakeFunction`] represents a object property with its corresponding fake
/// function name.
#[derive(serde::Serialize, Debug)]
pub struct PropertyFakeFunction {
    /// the object property name
    pub name: String,
    /// the fake function for the object property
    pub fake_func: FakeFunction,
    /// flag indicating whether the property is required or not
    pub required: bool,
    /// the probability with which to exclude the property if it is not required
    pub exclude_probability: f64,
}

/// [`ArrayFakeFunction`] represents the dynamic information for an array fake function.
#[derive(serde::Serialize, Debug)]
pub struct ArrayFakeFunction {
    /// the name of the fake function
    pub name: String,
    /// the fake function for the array items
    pub fake_func: FakeFunction,
    /// the inclusive minimum number of items in the array
    inclusive_min_items: usize,
    /// the exclusive minimum number of items in the array
    exclusive_min_items: isize,
    /// the inclusive maximum number of items in the array
    inclusive_max_items: usize,
    /// the exclusive maximum number of items in the array
    exclusive_max_items: usize,
}

impl ArrayFakeFunction {
    pub fn new(
        name: String,
        fake_func: FakeFunction,
        inclusive_min_items: usize,
        inclusive_max_items: usize,
    ) -> Self {
        Self {
            name,
            fake_func,
            inclusive_min_items,
            exclusive_min_items: (inclusive_min_items as isize) - 1,
            inclusive_max_items,
            exclusive_max_items: inclusive_max_items + 1,
        }
    }
}

/// [`QueryDataFunction`] represents the dynamic information for the query data generation function
/// for a single service call.
#[derive(serde::Serialize, Debug)]
pub struct QueryDataFunction {
    /// the name of the function
    pub name: String,
    /// the expected input parameters
    pub params: Vec<QueryParamFakeFunction>,
}

/// [`QueryParamFakeFunction`] represents a query parameter with its corresponding fake
/// function name.
#[derive(serde::Serialize, Debug)]
pub struct QueryParamFakeFunction {
    /// the parameter name
    pub name: String,
    /// the fake function for the parameter
    pub fake_func: FakeFunction,
    /// flag indicating whether the parameter is nullable
    pub nullable: bool,
    /// the probability with which to exclude the parameter if it is nullable
    pub exclude_probability: f64,
}

/// [`PostServiceCall`] represents the dynamic information for a single service call to another
/// endpoint.
#[derive(serde::Serialize, Debug)]
pub struct PostServiceCall {
    /// the name of the function
    pub name: String,
    /// flag indicating whether the service call requires query data from a fake function
    pub requires_query_data: bool,
    /// the data function name if the service call requires query data
    pub query_data_func: String,
    /// the data function name if the service call requires body payload data
    pub body_data_func: String,
    /// the path of the service call
    pub path: String,
    /// the name of environment variable containing the host name for the service call
    pub host_env_var: String,
}

#[derive(serde::Serialize, Debug)]
pub struct GetServiceCall {
    /// the name of the function
    pub name: String,
    /// flag indicating whether the service call requires data from a fake function
    pub requires_data: bool,
    /// the data function name if the service call requires query data
    pub query_data_func: String,
    /// the path of the service call
    pub path: String,
    /// the name of environment variable containing the host name for the service call
    pub host_env_var: String,
}

/// [`ServiceCallFunction`] represents the dynamic information for all outgoing service
/// calls of an endpoint.
#[derive(serde::Serialize, Debug)]
pub struct ServiceCallFunction {
    /// the name of the function
    pub name: String,
    /// the individual service calls with a HTTP POST method
    pub post_service_calls: Vec<PostServiceCall>,
    /// the individual service calls with a HTTP GET method
    pub get_service_calls: Vec<GetServiceCall>,
}

impl ServiceCallFunction {
    /// constructs a new service call function
    pub fn new(name: String) -> Self {
        Self {
            name,
            post_service_calls: Vec::default(),
            get_service_calls: Vec::default(),
        }
    }
}
