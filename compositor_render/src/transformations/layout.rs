use std::time::Duration;

use compositor_common::scene::NodeId;

use crate::wgpu::texture::NodeTexture;

pub struct LayoutNode {}

impl LayoutNode {
    pub fn new() -> Self {
        LayoutNode {}
    }

    pub fn render(
        &self,
        sources: &[(&NodeId, &NodeTexture)],
        target: &mut NodeTexture,
        pts: Duration,
    ) {
    }
}
