mod expressions;
mod models;

pub use expressions::Expression;
use rand::Rng;

use crate::{handler, schema};
pub use models::{LoadGeneratorFile, LoadService, UserRequest};

pub(crate) fn to_load_generator_type(schema_kind: &schema::SchemaKind) -> Expression {
    match schema_kind {
        schema::SchemaKind::Type(type_schema) => match type_schema {
            schema::Type::String(string_type) => {
                let openapiv3::VariantOrUnknownOrEmpty::Item(format) = string_type.format else {
                    let min = string_type.min_length.unwrap_or_default();
                    let max = string_type.max_length.unwrap_or(64);
                    return Expression::RandomString {
                        text: (max + min) / 2,
                    };
                };
                match format {
                    openapiv3::StringFormat::Date => Expression::LocalNow {
                        text: "Y-M-d".into(),
                    },
                    openapiv3::StringFormat::DateTime => Expression::LocalNow {
                        text: "Y-M-d'T'H:m:s.n'Z'".into(),
                    },
                    _ => {
                        let min = string_type.min_length.unwrap_or_default();
                        let max = string_type.max_length.unwrap_or(64);
                        Expression::RandomString {
                            text: (max + min) / 2,
                        }
                    }
                }
            }
            schema::Type::Number(number_type) => {
                // FIXME: Float might be out of range if fraction_min > fraction_max

                let min_str = number_type.minimum.unwrap_or_default().to_string();
                let max_str = number_type.maximum.unwrap_or(1.0).to_string();
                let (integer_min, fraction_min) = min_str.split_once(".").unwrap();
                let (integer_max, fraction_max) = max_str.split_once(".").unwrap();

                let width = fraction_min.len().max(fraction_max.len());

                let fraction_max: i64 = fraction_max.parse::<i64>().unwrap() + 1;
                let fraction_min: i64 = fraction_min.parse().unwrap();

                let integer_min: i64 = integer_min.parse().unwrap();
                let integer_max: i64 = integer_max.parse::<i64>().unwrap()
                    + if number_type.exclusive_maximum { 0 } else { 1 };
                Expression::Composite {
                    children: vec![
                        Expression::RandomInt {
                            text: expressions::RandomIntText::new(integer_min, integer_max),
                        },
                        Expression::Const { text: ".".into() },
                        Expression::RandomInt {
                            text: expressions::RandomIntText::new_with_digits(
                                fraction_min,
                                fraction_max,
                                width,
                            ),
                        },
                    ],
                }
            }
            schema::Type::Integer(integer_type) => {
                let min = {
                    integer_type.minimum.unwrap_or_default()
                        + if integer_type.exclusive_minimum { 1 } else { 0 }
                };
                let max = {
                    integer_type.maximum.unwrap_or(100)
                        + if integer_type.exclusive_maximum { 0 } else { 1 }
                };
                Expression::RandomInt {
                    text: expressions::RandomIntText::new(min, max),
                }
            }
            schema::Type::Boolean(boolean_type) => {
                let mut choices = Vec::with_capacity(boolean_type.enumeration.len());
                for el in boolean_type.enumeration.iter().flatten() {
                    choices.push(el);
                }
                if choices.is_empty() {
                    choices.push(&true);
                    choices.push(&false);
                }
                Expression::RandomOf {
                    text: expressions::RandomOfText {
                        delimiter: ",".into(),
                        elements: choices.into_iter().map(bool::to_string).collect(),
                    },
                }
            }
            schema::Type::Object(object_type) => {
                let mut children = Vec::with_capacity(2 * object_type.properties.len() + 1);
                children.push(Expression::Const { text: "{".into() });
                for (index, (name, property)) in object_type.properties.iter().enumerate() {
                    let mut property_children = Vec::with_capacity(5);
                    property_children.push(Expression::Const {
                        text: format!(r#""{}": "#, name),
                    });
                    property_children.extend(expression_with_quotation(&property.schema_kind));
                    children.push(Expression::Composite {
                        children: property_children,
                    });
                    if index < object_type.properties.len() - 1 {
                        children.push(Expression::Const { text: ", ".into() });
                    }
                }
                children.push(Expression::Const { text: "}".into() });

                Expression::Composite { children }
            }
            schema::Type::Array(array_type) => {
                let item_expr = expression_with_quotation(&array_type.items.schema_kind);
                let min = array_type.min_items.unwrap_or_default();
                let max = array_type.max_items.unwrap_or(15);
                let n_elements = rand::thread_rng().gen_range(min..max);
                let mut children = Vec::with_capacity(2 * n_elements + 1);
                children.push(Expression::Const { text: "[".into() });
                for index in 0..n_elements {
                    children.extend(item_expr.clone());
                    if index < n_elements - 1 {
                        children.push(Expression::Const { text: ", ".into() });
                    }
                }
                children.push(Expression::Const { text: "]".into() });
                Expression::Composite { children }
            }
        },
    }
}

pub fn expression_with_quotation(schema_kind: &schema::SchemaKind) -> Vec<Expression> {
    let mut out = Vec::with_capacity(3);
    //check if value must be wrapped in quotation marks
    match schema_kind {
        schema::SchemaKind::Type(type_schema) => match type_schema {
            // String type requires quotation
            schema::Type::String(_) => {
                out.push(Expression::Const {
                    text: r#"""#.into(),
                });
                out.push(to_load_generator_type(schema_kind));
                out.push(Expression::Const {
                    text: r#"""#.into(),
                });
            }
            // All other types (number, integer, boolean, object, array) do not require quotation
            _ => {
                out.push(to_load_generator_type(schema_kind));
            }
        },
    }
    out
}

pub struct LoadGeneratorFunction {
    pub uri_expression: Option<Expression>,
    pub body_expression: Option<Expression>,
}

impl From<&handler::Function> for LoadGeneratorFunction {
    fn from(value: &handler::Function) -> Self {
        let length = value.signature.parameters.len();
        let mut query_children = Vec::with_capacity(3 * length);
        let mut body_expression = Option::None;
        for param in &value.signature.parameters {
            if param.schema.get_object_schema().is_some()
                || param.schema.get_array_schema_type().is_some()
            {
                body_expression = Some(to_load_generator_type(&param.schema.schema_kind))
            } else {
                query_children.push(Expression::Const {
                    text: format!("{}=", param.as_name()),
                });
                query_children.push(to_load_generator_type(&param.schema.schema_kind));
                query_children.push(Expression::Const { text: "&".into() });
            }
        }
        // Remove last unnessecary "&"
        if !query_children.is_empty() {
            query_children.pop();
        }

        Self {
            uri_expression: if !query_children.is_empty() {
                Some(Expression::Composite {
                    children: vec![
                        Expression::Const { text: "?".into() },
                        Expression::Composite {
                            children: query_children,
                        },
                    ],
                })
            } else {
                None
            },
            body_expression,
        }
    }
}
