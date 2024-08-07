use super::{
    algorithms::coloring::ColorIndex,
    colored_graph::{ColorData, ColorNodeIndex, ColorNodesData},
    ColoredGraph,
};
pub use super::{DiGraph, Edge, EdgeData, EdgeIndex, NodeData, NodeIndex};

pub struct DiGraphBuilder {
    nodes: Vec<NodeData>,
    edges: Vec<EdgeData>,
}

impl Default for DiGraphBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl DiGraphBuilder {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn with_node_count(count: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(count),
            edges: Vec::new(),
        }
    }

    pub fn with_edge_count(count: usize) -> Self {
        // Formula for minimum number of nodes given the number of edges.
        // Reference: https://math.stackexchange.com/a/1954272
        let minimum_node_count = ((2.0 * count as f64).sqrt() + 0.5).ceil() as usize;
        Self {
            nodes: Vec::with_capacity(minimum_node_count),
            edges: Vec::with_capacity(count),
        }
    }

    pub fn with_node_and_edge_count(node_count: usize, edge_count: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(node_count),
            edges: Vec::with_capacity(edge_count),
        }
    }

    pub fn add_node(mut self) -> Self {
        self.nodes.push(NodeData::new());
        self
    }

    pub fn add_nodes(mut self, amount: usize) -> Self {
        self.nodes.reserve(amount);
        for _ in 0..amount {
            self = self.add_node();
        }
        self
    }

    pub fn add_edge(mut self, edge: Edge) -> Self {
        let edge_index = self.edges.len();
        {
            let outgoing_node_data = &self.nodes[edge.source.0];
            let incoming_node_data = &self.nodes[edge.target.0];
            self.edges.push(EdgeData::new(
                edge.source,
                edge.target,
                outgoing_node_data.first_outgoing_edge,
                incoming_node_data.first_incoming_edge,
            ));
        }
        {
            let outgoing_node_data = &mut self.nodes[edge.source.0];
            outgoing_node_data.first_outgoing_edge = Some(EdgeIndex(edge_index));
        }
        {
            let incoming_node_data = &mut self.nodes[edge.target.0];
            incoming_node_data.first_incoming_edge = Some(EdgeIndex(edge_index));
        }

        self
    }

    pub fn add_edges(mut self, edges: &[Edge]) -> Self {
        self.edges.reserve(edges.len());
        for edge in edges {
            self = self.add_edge(*edge);
        }

        self
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn maximum_degree(&self) -> usize {
        let mut degrees: Vec<usize> = vec![0; self.nodes.len()];
        for edge in &self.edges {
            degrees[edge.source.0] += 1;
            degrees[edge.target.0] += 1;
        }
        degrees.into_iter().max().unwrap_or_default()
    }

    pub fn build(self) -> DiGraph {
        DiGraph {
            nodes: self.nodes,
            edges: self.edges,
        }
    }
}

pub struct ColoredGraphBuilder {
    graph: DiGraph,
    coloring: Vec<ColorIndex>,
    colors: Vec<ColorData>,
    color_nodes: Vec<ColorNodesData>,
}

impl ColoredGraphBuilder {
    pub fn new(graph: DiGraph, coloring: Vec<ColorIndex>, color_count: usize) -> Self {
        let nc = graph.node_count();
        if nc != coloring.len() {
            panic!("not a valid coloring!");
        }
        let colors = vec![ColorData::new(); color_count];
        let color_nodes = Vec::with_capacity(nc);

        Self {
            graph,
            coloring,
            colors,
            color_nodes,
        }
    }

    fn add_color_node(&mut self, node: NodeIndex, color: ColorIndex) {
        let color_node_index = self.color_nodes.len();
        let color_data = &mut self.colors[color.0];
        self.color_nodes
            .push(ColorNodesData::new(node, color_data.first_color_node_index));
        color_data.first_color_node_index = Some(ColorNodeIndex(color_node_index));
    }

    pub fn build(mut self) -> ColoredGraph {
        for (node, color) in self.coloring.clone().iter().enumerate() {
            self.add_color_node(node.into(), *color);
        }

        ColoredGraph {
            graph: self.graph,
            coloring: self.coloring,
            colors: self.colors,
            color_nodes: self.color_nodes,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_di_graph_builder() {
        let graph = DiGraphBuilder::new().build();
        assert_eq!(graph.node_count(), 0);
        assert_eq!(graph.iter_edges().count(), 0);
    }

    #[test]
    fn test_di_graph_builder_with_node_count() {
        let graph = DiGraphBuilder::with_node_count(3).build();
        assert_eq!(graph.nodes.capacity(), 3);
    }

    #[test]
    fn test_di_graph_builder_with_edge_count() {
        let graph = DiGraphBuilder::with_edge_count(2).build();
        assert_eq!(graph.nodes.capacity(), 3);
        assert_eq!(graph.edges.capacity(), 2);
    }

    #[test]
    fn test_di_graph_builder_with_node_and_edge_count() {
        let graph = DiGraphBuilder::with_node_and_edge_count(3, 2).build();

        assert_eq!(graph.edges.capacity(), 2);
        assert_eq!(graph.nodes.capacity(), 3);
    }

    #[test]
    fn test_di_graph_builder_add_node() {
        let graph = DiGraphBuilder::with_node_count(1).add_node().build();

        assert_eq!(graph.node_count(), 1);
    }

    #[test]
    fn test_di_graph_builder_add_nodes() {
        let graph = DiGraphBuilder::with_node_count(2).add_nodes(2).build();

        assert_eq!(graph.node_count(), 2);
    }

    #[test]
    fn test_di_graph_builder_add_edge() {
        let graph = DiGraphBuilder::with_node_and_edge_count(2, 1)
            .add_nodes(2)
            .add_edge((0, 1).into())
            .build();

        assert_eq!(graph.iter_edges().count(), 1);
    }

    #[test]
    fn test_di_graph_builder_add_edges() {
        let graph = DiGraphBuilder::with_node_and_edge_count(3, 2)
            .add_nodes(3)
            .add_edges(&[(0, 1).into(), (1, 2).into()])
            .build();

        assert_eq!(graph.iter_edges().count(), 2);
    }
}
