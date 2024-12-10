use std::{collections::VecDeque, fmt::Display};

use super::{EdgeView, NodeView, Predecessors, Successors};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
/// A simple identifier type for nodes in a [`DiGraph`].
pub struct NodeIndex(pub usize);

impl From<usize> for NodeIndex {
    fn from(value: usize) -> Self {
        Self(value)
    }
}

impl From<NodeIndex> for usize {
    fn from(value: NodeIndex) -> Self {
        value.0
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug, PartialOrd, Ord)]
/// A struct representing a directed edge from a source node to a target node.
pub struct Edge {
    pub source: NodeIndex,
    pub target: NodeIndex,
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}", self.source.0, self.target.0)
    }
}
impl From<(usize, usize)> for Edge {
    fn from(value: (usize, usize)) -> Self {
        Self {
            source: value.0.into(),
            target: value.1.into(),
        }
    }
}

impl From<(NodeIndex, NodeIndex)> for Edge {
    fn from(value: (NodeIndex, NodeIndex)) -> Self {
        Self {
            source: value.0,
            target: value.1,
        }
    }
}

impl From<Edge> for (usize, usize) {
    fn from(value: Edge) -> Self {
        (value.target.into(), value.source.into())
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
/// A simple identifier type for edges in a [`DiGraph`].
pub struct EdgeIndex(pub usize);

pub struct NodeData {
    pub(super) first_outgoing_edge: Option<EdgeIndex>,
    pub(super) first_incoming_edge: Option<EdgeIndex>,
}

impl Default for NodeData {
    fn default() -> Self {
        Self::new()
    }
}

impl NodeData {
    pub fn new() -> Self {
        Self {
            first_outgoing_edge: None,
            first_incoming_edge: None,
        }
    }
}

pub struct EdgeData {
    pub(super) source: NodeIndex,
    pub(super) target: NodeIndex,
    pub(super) next_outgoing_edge: Option<EdgeIndex>,
    pub(super) next_incoming_edge: Option<EdgeIndex>,
}

impl EdgeData {
    pub fn new(
        source: NodeIndex,
        target: NodeIndex,
        next_outgoing_edge: Option<EdgeIndex>,
        next_incoming_edge: Option<EdgeIndex>,
    ) -> Self {
        Self {
            source,
            target,
            next_outgoing_edge,
            next_incoming_edge,
        }
    }
}

pub struct DiGraph {
    pub(super) nodes: Vec<NodeData>,
    pub(super) edges: Vec<EdgeData>,
}

impl DiGraph {
    pub fn successors(&self, source: NodeIndex) -> Successors {
        let first_outgoing_edge = self.nodes[source.0].first_outgoing_edge;
        Successors::new(self, first_outgoing_edge)
    }

    pub fn predecssors(&self, source: NodeIndex) -> Predecessors {
        let first_incoming_edge = self.nodes[source.0].first_incoming_edge;
        Predecessors::new(self, first_incoming_edge)
    }

    pub fn iter_edges(&self) -> EdgeView {
        EdgeView::new(self)
    }

    pub fn iter_nodes(&self) -> NodeView {
        NodeView::new(self)
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn maximum_degree(&self) -> usize {
        let mut degrees: Vec<usize> = vec![0; self.node_count()];
        for edge in self.iter_edges() {
            degrees[edge.source.0] += 1;
            degrees[edge.target.0] += 1;
        }
        degrees.into_iter().max().unwrap_or_default()
    }

    pub fn is_acyclic(&self) -> bool {
        let mut in_degrees: Vec<_> = self
            .iter_nodes()
            .map(|v| self.predecssors(v).count())
            .collect();
        let mut queue = VecDeque::from_iter(
            in_degrees
                .iter()
                .enumerate()
                .filter(|(_, d)| **d == 0)
                .map(|(v, _)| NodeIndex(v)),
        );
        let mut count = 0;
        while !queue.is_empty() {
            let front = queue
                .pop_front()
                .expect("non empty queue due to while condition");
            count += 1;
            for v in self.successors(front) {
                in_degrees[v.0] -= 1;
                if in_degrees[v.0] == 0 {
                    queue.push_back(v);
                }
            }
        }
        count == self.node_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::DiGraphBuilder;

    #[test]
    fn test_simple_example_graph() {
        // N0 ---E0---> N1 ---E1---> N2
        // |                         ^
        // E2                        |
        // |                         |
        // v                         |
        // N3 ----------E3-----------+

        let e0 = (0, 1).into();
        let e1 = (1, 2).into();
        let e2 = (0, 3).into();
        let e3 = (3, 2).into();
        let graph = DiGraphBuilder::with_node_and_edge_count(4, 4)
            .add_nodes(4)
            .add_edges(&[e0, e1, e2, e3])
            .build();

        let successors: Vec<NodeIndex> = graph.successors(0.into()).collect();
        assert_eq!(&successors[..], &[3.into(), 1.into()]);
        let edges: Vec<Edge> = graph.iter_edges().collect();
        assert_eq!(&edges[..], &[e2, e0, e1, e3]);
        let predecessors: Vec<NodeIndex> = graph.predecssors(2.into()).collect();
        assert_eq!(&predecessors[..], &[3.into(), 1.into()]);
        assert!(graph.is_acyclic());
    }
}
