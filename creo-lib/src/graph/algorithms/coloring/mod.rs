mod equitable;
mod error;

use crate::graph::DiGraph;

pub use equitable::{equitable_coloring, is_coloring, is_equitable};
pub use error::{Error, Result};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ColorIndex(pub usize);

impl From<usize> for ColorIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<ColorIndex> for usize {
    fn from(value: ColorIndex) -> Self {
        value.0
    }
}

pub trait Colorer {
    fn new(graph: DiGraph, color_count: usize) -> Self;
    fn color() -> Vec<ColorIndex>;
}
