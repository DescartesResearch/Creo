use crate::graph::{algorithms::coloring::ColorIndex, ColoredGraph, NodeIndex};

use super::NodeView;

pub struct NodeWithColorView<'graph> {
    graph: &'graph ColoredGraph,
    node_view: NodeView,
}

impl<'graph> NodeWithColorView<'graph> {
    pub fn new(graph: &'graph ColoredGraph) -> Self {
        Self {
            graph,
            node_view: graph.graph.iter_nodes(),
        }
    }
}

impl<'graph> Iterator for NodeWithColorView<'graph> {
    type Item = (NodeIndex, ColorIndex);

    fn next(&mut self) -> Option<Self::Item> {
        match self.node_view.next() {
            None => None,
            Some(node_index) => Some((node_index, self.graph.coloring[node_index.0])),
        }
    }
}
