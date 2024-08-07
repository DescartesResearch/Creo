use crate::{
    application::ServiceCallEdge,
    graph::{ApplicationGraph, EndpointIndex},
};

use super::Successors;

pub struct ServiceCallView<'graph> {
    successor: Successors<'graph>,
    endpoint: EndpointIndex,
}

impl<'graph> ServiceCallView<'graph> {
    pub fn new(graph: &'graph ApplicationGraph, endpoint: EndpointIndex) -> Self {
        Self {
            successor: graph.graph.graph.successors(endpoint.into()),
            endpoint,
        }
    }
}

impl<'graph> Iterator for ServiceCallView<'graph> {
    type Item = ServiceCallEdge;

    fn next(&mut self) -> Option<Self::Item> {
        match self.successor.next() {
            None => None,
            Some(node_index) => Some(ServiceCallEdge::new(self.endpoint, node_index.into())),
        }
    }
}
