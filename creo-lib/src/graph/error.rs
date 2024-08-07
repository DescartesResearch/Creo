use super::di_graph::NodeIndex;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("NodeID {0:?} does not exists in the graph")]
    NodeIDNotFound(NodeIndex),
}

pub type Result<T> = std::result::Result<T, Error>;
