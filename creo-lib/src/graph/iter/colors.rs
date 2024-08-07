use std::ops::Range;

use crate::graph::{algorithms::coloring::ColorIndex, ColoredGraph};

pub struct ColorView {
    colors: Range<usize>,
}

impl ColorView {
    pub fn new(graph: &ColoredGraph) -> Self {
        let colors = 0..graph.color_count();
        Self { colors }
    }
}

impl Iterator for ColorView {
    type Item = ColorIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.colors.next().map(|color_index| color_index.into())
    }
}
