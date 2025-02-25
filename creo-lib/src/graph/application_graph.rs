use crate::programming_language::ProgrammingLanguage;

use super::{
    algorithms::coloring::ColorIndex,
    iter::{EndpointView, MicroServiceEndpointView, MicroServiceView, ServiceCallView},
    ColoredGraph, NodeIndex,
};

#[derive(Clone, Copy, Debug)]
pub struct MicroServiceIndex(pub usize);

impl From<ColorIndex> for MicroServiceIndex {
    fn from(value: ColorIndex) -> Self {
        MicroServiceIndex(value.0)
    }
}

impl From<MicroServiceIndex> for ColorIndex {
    fn from(val: MicroServiceIndex) -> Self {
        ColorIndex(val.0)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct EndpointIndex(pub usize);

impl From<NodeIndex> for EndpointIndex {
    fn from(value: NodeIndex) -> Self {
        EndpointIndex(value.0)
    }
}

impl From<EndpointIndex> for NodeIndex {
    fn from(val: EndpointIndex) -> Self {
        NodeIndex(val.0)
    }
}

pub struct ApplicationGraph {
    pub(super) graph: ColoredGraph,
    pub(super) languages: Vec<ProgrammingLanguage>,
    pub(super) start_port: u32,
    pub(super) handler_definitions: Vec<std::path::PathBuf>,
}

impl ApplicationGraph {
    pub fn new(
        graph: ColoredGraph,
        languages: Vec<ProgrammingLanguage>,
        start_port: u32,
        handler_definitions: Vec<std::path::PathBuf>,
    ) -> Self {
        Self {
            graph,
            languages,
            start_port,
            handler_definitions,
        }
    }

    pub fn service_count(&self) -> usize {
        self.graph.color_count()
    }

    pub fn iter_micro_services(&self) -> MicroServiceView {
        MicroServiceView::new(self)
    }

    pub fn iter_endpoints(&self) -> EndpointView {
        EndpointView::new(self)
    }

    pub fn iter_service_endpoints(
        &self,
        micro_service: MicroServiceIndex,
    ) -> MicroServiceEndpointView {
        MicroServiceEndpointView::new(self, micro_service)
    }

    pub fn iter_service_calls(&self, endpoint: EndpointIndex) -> ServiceCallView {
        ServiceCallView::new(self, endpoint)
    }

    pub fn get_endpoint_path(&self, endpoint: EndpointIndex) -> String {
        format!("/endpoint{}", endpoint.0)
    }

    pub fn get_service(&self, endpoint: EndpointIndex) -> MicroServiceIndex {
        self.graph.coloring[endpoint.0].into()
    }

    pub fn is_user_frontend(&self, endpoint: EndpointIndex) -> bool {
        self.graph.graph.predecssors(endpoint.into()).count() == 0
    }

    pub fn get_host_env_var(&self, service: MicroServiceIndex) -> String {
        format!("HOST_SERVICE_{}", service.0)
    }
}
