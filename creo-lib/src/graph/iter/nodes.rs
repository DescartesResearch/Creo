use std::ops::Range;

use crate::graph::{DiGraph, NodeIndex};

pub struct NodeView {
    nodes: Range<usize>,
}

impl NodeView {
    pub fn new(graph: &DiGraph) -> Self {
        let nodes = 0..graph.node_count();
        Self { nodes }
    }
}

impl From<usize> for NodeView {
    fn from(value: usize) -> Self {
        Self { nodes: 0..value }
    }
}

impl Iterator for NodeView {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        self.nodes.next().map(|node_index| node_index.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::DiGraphBuilder;

    #[test]
    fn test_empty_node_view() {
        let graph = DiGraphBuilder::new().build();

        let node_view = NodeView::new(&graph);
        assert_eq!(node_view.count(), 0);
    }

    #[test]
    fn test_node_view() {
        let graph = DiGraphBuilder::with_node_count(2).add_nodes(2).build();

        let n0: NodeIndex = 0.into();
        let n1: NodeIndex = 1.into();
        let nodes: Vec<_> = NodeView::new(&graph).collect();

        assert_eq!(&nodes[..], &[n0, n1]);
    }
}
