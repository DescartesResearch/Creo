pub use crate::graph::{DiGraph, Edge, NodeIndex, Successors};

pub struct EdgeView<'graph> {
    graph: &'graph DiGraph,
    current_node_index: NodeIndex,
    successors: Option<Successors<'graph>>,
}

impl<'graph> EdgeView<'graph> {
    pub fn new(graph: &'graph DiGraph) -> Self {
        let current_node_index: NodeIndex = 0.into();
        let successors = if graph.node_count() > 0 {
            Some(graph.successors(current_node_index))
        } else {
            None
        };
        Self {
            graph,
            current_node_index,
            successors,
        }
    }
}

impl<'graph> Iterator for EdgeView<'graph> {
    type Item = Edge;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(successors) = &mut self.successors {
            // Check if the current node still has any outgoing edge.
            if let Some(target) = successors.next() {
                return Some((self.current_node_index, target).into());
            }

            // At this point, the current node has no outgoing edges anymore, so search for the
            // next node with any outgoing edges.
            let mut next_index = self.current_node_index.0 + 1;
            let node_count = self.graph.node_count();

            while next_index < node_count
                && self.graph.nodes[next_index].first_outgoing_edge.is_none()
            {
                next_index += 1;
            }

            // When the index is equal to the node count, all nodes have been checked for their
            // outgoing edges.
            if next_index == node_count {
                return None;
            }

            // At this point, the `next_index` points to the next node with at least one outgoing
            // edge. It is safe to call `unwrap()` on the first `next()` call of the successor
            // iterator, as the iterator contains at least one element.
            let mut successors = self.graph.successors(next_index.into());
            let target = successors.next().unwrap();

            // Update the internal state, so the succeeding call of `next()` checks for any
            // additional outgoing edges of the current node.
            self.current_node_index = next_index.into();
            self.successors = Some(successors);
            return Some((self.current_node_index, target).into());
        }

        // At this point, the graph does not contain any edges, so this iterator is empty.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::DiGraphBuilder;

    #[test]
    fn test_iter_edges() {
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

        let edges: Vec<Edge> = graph.iter_edges().collect();
        assert_eq!(&edges[..], &[e1, e0]);
    }
}
