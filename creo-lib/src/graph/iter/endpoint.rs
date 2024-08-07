use crate::{application::Endpoint, graph::ApplicationGraph};

use super::NodeView;

pub struct EndpointView<'graph> {
    graph: &'graph ApplicationGraph,
    node_view: NodeView,
}

impl<'graph> EndpointView<'graph> {
    pub fn new(graph: &'graph ApplicationGraph) -> Self {
        Self {
            graph,
            node_view: graph.graph.graph.iter_nodes(),
        }
    }
}

impl<'graph> Iterator for EndpointView<'graph> {
    type Item = Endpoint<'graph>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.node_view.next() {
            None => None,
            Some(node_index) => Some(Endpoint::new(
                node_index.into(),
                &self.graph.handler_definitions[node_index.0],
            )),
        }
    }
}
