#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("cannot guarantee coloring with {given} colors, need at least {expected} colors.")]
    NotEnoughColors { given: usize, expected: usize },
    #[error("graph is not acyclic.")]
    CyclicGraph,
}

pub type Result<T> = std::result::Result<T, Error>;
