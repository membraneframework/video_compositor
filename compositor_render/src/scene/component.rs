use compositor_common::{
    scene::{
        builtin_transformations::{tiled_layout::TiledLayoutSpec, FixedPositionLayoutSpec},
        text_spec::TextSpec,
    },
    util::degree::Degree,
};

use crate::scene::{layout_node::LayoutView, InnerNodeParams};

use super::{layout_node::Layout, Component, NodeParams};

#[derive(Clone)]
pub struct Text {
    spec: TextSpec,
}

pub struct FixedPositionLayout {
    pub spec: FixedPositionLayoutSpec,
    pub children: Vec<Component>,
}

impl FixedPositionLayout {
    pub fn layout(&self, children: &[NodeParams]) -> (Layout, Vec<NodeParams>) {
        todo!()
    }
}
