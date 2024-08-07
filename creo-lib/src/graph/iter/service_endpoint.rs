use crate::{
    application::Endpoint,
    graph::{ApplicationGraph, MicroServiceIndex},
};

use super::ColorNodeView;

pub struct MicroServiceEndpointView<'graph> {
    graph: &'graph ApplicationGraph,
    color_node_view: ColorNodeView<'graph>,
}

impl<'graph> MicroServiceEndpointView<'graph> {
    pub fn new(graph: &'graph ApplicationGraph, micro_service: MicroServiceIndex) -> Self {
        Self {
            graph,
            color_node_view: graph.graph.color_nodes(micro_service.into()),
        }
    }
}

impl<'graph> Iterator for MicroServiceEndpointView<'graph> {
    type Item = Endpoint<'graph>;

    fn next(&mut self) -> Option<Self::Item> {
        match self.color_node_view.next() {
            None => None,
            Some(color_node_index) => Some(Endpoint::new(
                color_node_index.into(),
                &self.graph.handler_definitions[color_node_index.0],
            )),
        }
    }
}
