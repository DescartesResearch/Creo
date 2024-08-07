use super::{
    algorithms::coloring::ColorIndex,
    iter::{ColorView, NodeWithColorView},
    ColorNodeView, DiGraph, NodeIndex,
};

#[derive(Clone, Debug)]
pub struct ColorData {
    pub(super) first_color_node_index: Option<ColorNodeIndex>,
}

impl ColorData {
    pub fn new() -> Self {
        Self {
            first_color_node_index: None,
        }
    }
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct ColorNodeIndex(pub usize);

pub struct ColorNodesData {
    pub(super) node: NodeIndex,
    pub(super) next_color_node_index: Option<ColorNodeIndex>,
}

impl ColorNodesData {
    pub fn new(node: NodeIndex, next_color_map_index: Option<ColorNodeIndex>) -> Self {
        Self {
            node,
            next_color_node_index: next_color_map_index,
        }
    }
}

pub struct ColoredGraph {
    pub graph: DiGraph,
    pub(super) coloring: Vec<ColorIndex>,
    pub(super) colors: Vec<ColorData>,
    pub(super) color_nodes: Vec<ColorNodesData>,
}

impl ColoredGraph {
    pub fn color_count(&self) -> usize {
        self.colors.len()
    }

    pub fn color_nodes(&self, color: ColorIndex) -> ColorNodeView {
        let first_color_node = self.colors[color.0].first_color_node_index;
        ColorNodeView::new(self, first_color_node)
    }

    pub fn iter_colors(&self) -> ColorView {
        ColorView::new(self)
    }

    pub fn iter_nodes_with_colors(&self) -> NodeWithColorView {
        NodeWithColorView::new(self)
    }
}
