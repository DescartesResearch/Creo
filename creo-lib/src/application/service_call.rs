use crate::graph::EndpointIndex;

#[derive(Clone, Copy, Debug)]
pub struct ServiceCallEdge {
    pub source: EndpointIndex,
    pub target: EndpointIndex,
}

impl ServiceCallEdge {
    pub fn new(source: EndpointIndex, target: EndpointIndex) -> Self {
        Self { source, target }
    }
}
