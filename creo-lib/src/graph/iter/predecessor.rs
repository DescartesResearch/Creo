pub use crate::graph::{DiGraph, EdgeIndex, NodeIndex};

pub struct Predecessors<'graph> {
    graph: &'graph DiGraph,
    current_edge_index: Option<EdgeIndex>,
}

impl<'graph> Predecessors<'graph> {
    pub fn new(graph: &'graph DiGraph, first_incoming_edge: Option<EdgeIndex>) -> Self {
        Self {
            graph,
            current_edge_index: first_incoming_edge,
        }
    }
}

impl<'graph> Iterator for Predecessors<'graph> {
    type Item = NodeIndex;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_edge_index {
            None => None,
            Some(edge_index) => {
                let edge = &self.graph.edges[edge_index.0];
                self.current_edge_index = edge.next_incoming_edge;
                Some(edge.source)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::{DiGraphBuilder, Edge};

    #[test]
    fn test_predecessor() {
        let e0: Edge = (1, 0).into();
        let e1: Edge = (2, 0).into();

        // N0 <---E0--- N1
        // ^
        // |
        // E1
        // |
        // N2
        let graph = DiGraphBuilder::with_node_and_edge_count(3, 2)
            .add_nodes(3)
            .add_edges(&[e0, e1])
            .build();

        let predecessors: Vec<NodeIndex> = graph.predecssors(0.into()).collect();
        assert_eq!(&predecessors[..], &[2.into(), 1.into()]);
    }
}
