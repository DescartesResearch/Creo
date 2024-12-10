use crate::graph::Edge;

use super::ColorIndex;

#[derive(thiserror::Error, PartialEq, Debug)]
pub enum Error {
    #[error("cannot guarantee coloring with {given} colors, need at least {expected} colors.")]
    NotEnoughColors { given: usize, expected: usize },
    #[error("graph is not acyclic.")]
    CyclicGraph,
    #[error("invalid coloring: vertices of edge {edge} have the same color {color}")]
    InvalidColoring { edge: Edge, color: ColorIndex },
}

pub type Result<T> = std::result::Result<T, Error>;
