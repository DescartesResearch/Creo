use crate::graph::{DiGraph, EdgeIndex, NodeIndex};

pub struct Successors<'graph> {
    graph: &'graph DiGraph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Successors<'graph> {
    pub fn new(graph: &'graph DiGraph, first_outgoing_edge: Option<EdgeIndex>) -> Self {
        Self {
            graph,
            current_edge_index: first_outgoing_edge,
        }
    }
}

impl<'graph> Iterator for Successors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.0];
                self.current_edge_index = edge.next_outgoing_edge;
                Some(edge.target)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{DiGraphBuilder, Edge};

    #[test]
    fn test_successors() {
        let e0: Edge = (0, 1).into();
        let e1: Edge = (0, 2).into();

        // N0 ---E0---> N1
        // |
        // E1
        // |
        // v
        // N2
        let graph = DiGraphBuilder::with_node_and_edge_count(3, 2)
            .add_nodes(3)
            .add_edges(&[e0, e1])
            .build();

        let successors: Vec<NodeIndex> = graph.successors(0.into()).collect();
        assert_eq!(&successors[..], &[2.into(), 1.into()]);
    }
}
