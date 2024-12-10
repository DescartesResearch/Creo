#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("invalid port range: port range must be in [{min}..{max}], but was [{start}..{end}]")]
    InvalidPortRange {
        start: u32,
        end: u32,
        min: u32,
        max: u32,
    },
    #[error("invalid number of inter-service calls: expected at most {vertices} (#endpoints) * {colors} (#services) = {expected}, but got {got}")]
    InvalidEdgeCount {
        got: usize,
        vertices: usize,
        colors: usize,
        expected: usize,
    },
    #[error("invalid number of services: expected at most {expected} (#endpoints) colors, but got {got}")]
    InvalidColorCount { got: usize, expected: usize },
    #[error("invalid endpoint(s) in assignment: expected endpoints in the range [0..{vertices}[, but got {got:?}")]
    InvalidEndpointsInAssignment { got: Vec<usize>, vertices: usize },
    #[error("incomplete endpoint assignment: missing assignment for endpoints {missing:?}")]
    IncompleteEndpointAssignment { missing: Vec<usize> },
    #[error("incomplete handler function assignment: missing assignment for endpoints {missing:?}: either specify all endpoints or use the semi-manual mode")]
    IncompleteHandlerFunctionAssignment { missing: Vec<usize> },
}

pub type Result<T> = std::result::Result<T, Error>;
