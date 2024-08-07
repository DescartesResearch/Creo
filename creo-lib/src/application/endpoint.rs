use crate::graph::EndpointIndex;

#[derive(Clone, Debug)]
pub struct Endpoint<'service> {
    pub id: EndpointIndex,
    pub handler_dir: &'service std::path::PathBuf,
}

impl<'service> Endpoint<'service> {
    pub fn new(id: EndpointIndex, definiton: &'service std::path::PathBuf) -> Self {
        Self {
            id,
            handler_dir: definiton,
        }
    }
}
