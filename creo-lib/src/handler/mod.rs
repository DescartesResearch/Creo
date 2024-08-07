mod arg_passing;
mod definition;
mod function;
mod param;
mod registry;
mod signature;

pub use arg_passing::PassingType;
pub use definition::Definition;
pub use function::Function;
pub use param::Param;
pub use registry::FunctionRegistry;
pub use signature::Signature;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Validation(#[from] serde_yaml::Error),
}
