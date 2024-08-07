use indexmap::IndexMap;

use crate::{handler, http_method::HTTPMethod};

use super::Expression;

#[derive(serde::Serialize, Debug, Clone)]
pub struct LoadGeneratorFile {
    pub services: Vec<LoadService>,
    pub user_requests: Vec<UserRequest>,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct LoadService {
    common_headers: IndexMap<String, String>,
    hosts: Vec<String>,
    protocol: String,
    response_content_type: String,
    service_name: String,
}

impl LoadService {
    pub fn new(hosts: Vec<String>, service_name: String) -> Self {
        Self {
            common_headers: IndexMap::from([("Content-Type".into(), "application/json".into())]),
            hosts,
            protocol: "http".into(),
            response_content_type: "json".into(),
            service_name,
        }
    }
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct UserRequest {
    service_name: String,
    uri: Expression,
    method: HTTPMethod,
    expected_response_codes: Vec<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    body: Option<Expression>,
}

impl UserRequest {
    pub fn new(function: &handler::Function, service_name: String, path: String) -> Self {
        let method = function.get_http_method();
        let load_function: super::LoadGeneratorFunction = function.into();
        let mut uri_children = Vec::with_capacity(2);
        uri_children.push(Expression::Const { text: path });
        if let Some(uri_expression) = load_function.uri_expression {
            uri_children.push(uri_expression);
        }

        Self {
            service_name,
            uri: super::Expression::Composite {
                children: uri_children,
            },
            method,
            expected_response_codes: vec![200, 201, 203, 204],
            body: load_function.body_expression,
        }
    }
}

impl AsRef<UserRequest> for UserRequest {
    fn as_ref(&self) -> &UserRequest {
        self
    }
}
