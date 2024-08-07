#[derive(serde::Serialize, Debug)]
pub struct ServiceInfo {
    /// The title of the microservice.
    pub title: String,
    /// The description of the microservice.
    pub description: String,
    /// The version of the microservice.
    pub version: String,
    /// The openapi contact information for the microservice.
    pub contact: openapiv3::Contact,
}
