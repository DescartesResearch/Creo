use crate::{application::MicroService, graph::ApplicationGraph};

use super::ColorView;

pub struct MicroServiceView<'graph> {
    graph: &'graph ApplicationGraph,
    color_view: ColorView,
}

impl<'graph> MicroServiceView<'graph> {
    pub fn new(graph: &'graph ApplicationGraph) -> Self {
        Self {
            graph,
            color_view: graph.graph.iter_colors(),
        }
    }
}

impl<'graph> Iterator for MicroServiceView<'graph> {
    type Item = MicroService;

    fn next(&mut self) -> Option<Self::Item> {
        match self.color_view.next() {
            None => None,
            Some(color_index) => Some(MicroService::new(
                color_index.into(),
                self.graph.languages[color_index.0],
                self.graph.start_port + (color_index.0 as u32),
            )),
        }
    }
}
