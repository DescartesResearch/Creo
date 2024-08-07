use crate::graph::{colored_graph::ColorNodeIndex, ColoredGraph, NodeIndex};

pub struct ColorNodeView<'graph> {
    graph: &'graph ColoredGraph,
    current_color_node_index: Option<ColorNodeIndex>,
}

impl<'graph> ColorNodeView<'graph> {
    pub fn new(
        graph: &'graph ColoredGraph,
        first_color_node_index: Option<ColorNodeIndex>,
    ) -> Self {
        Self {
            graph,
            current_color_node_index: first_color_node_index,
        }
    }
}

impl<'graph> Iterator for ColorNodeView<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_color_node_index {
            None => None,
            Some(color_node_index) => {
                let color_node = &self.graph.color_nodes[color_node_index.0];
                self.current_color_node_index = color_node.next_color_node_index;
                Some(color_node.node)
            }
        }
    }
}
