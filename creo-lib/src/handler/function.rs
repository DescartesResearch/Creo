use crate::{dependencies::Dependency, http_method::HTTPMethod};

use super::signature::Signature;

#[derive(serde::Deserialize, Clone, Debug)]
pub struct Function {
    pub import_path: String,
    #[serde(default)]
    pub description: Option<String>,
    pub signature: Signature,
    pub is_async: bool,
    #[serde(default)]
    pub depends_on: Vec<Dependency>,
    #[serde(default)]
    pub returns: bool,
}

impl Function {
    pub fn get_http_method(&self) -> HTTPMethod {
        for param in &self.signature.parameters {
            if param.schema.get_object_schema().is_some() {
                return HTTPMethod::Post;
            }
            if param.schema.get_array_schema_type().is_some() {
                return HTTPMethod::Post;
            }
        }

        HTTPMethod::Get
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::de::FromYamlStr;

    #[test]
    fn test_valid_handler_definition() {
        const INPUT: &str = "
            import_path: register_user.register
            is_async: true
            signature:
              function: register_user
              parameters:
                - type: string
                  arg: username
                - type: string
                  arg: password
                - type: string
                  arg: email
        ";

        Function::from_yaml_str(INPUT).unwrap();
    }

    #[test]
    fn test_login_handler_definition() {
        const INPUT: &str = "
            import_path: login_user.login
            is_async: true
            signature:
              function: login_with_username_or_email
              parameters:
                - arg: username_or_email
                  type: string
                  nullable: false
                  minLength: 3
                  maxLength: 64
                - arg: password
                  type: string
                  nullable: false
                  minLength: 6
                  maxLength: 48
              depends_on:
                - name: mongo_db
                  service_yaml: mongo.yaml
                  environment:
                    - MONGO_HOST={{service_name}}
                    - MONGO_PORT=27017
                    - MONGO_USER=root
                    - MONGO_PASSWORD=supers3cret";
        Function::from_yaml_str(INPUT).unwrap();
    }
}
