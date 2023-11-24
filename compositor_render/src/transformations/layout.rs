use std::{sync::Arc, time::Duration};

use crate::{
    renderer::{render_graph::NodeId, RenderCtx},
    scene::Size,
    wgpu::texture::NodeTexture,
};

mod flatten;
mod layout_renderer;
mod params;
mod shader;
mod transformation_matrices;

use compositor_common::{scene::Resolution, util::colors::RGBAColor};

use self::{
    params::{LayoutNodeParams, ParamsBuffer},
    shader::LayoutShader,
};

pub(crate) use layout_renderer::LayoutRenderer;
pub(crate) use transformation_matrices::{vertices_transformation_matrix, Position};

pub(crate) trait LayoutProvider: Send {
    fn layouts(&mut self, pts: Duration, inputs: &[Option<Resolution>]) -> NestedLayout;
    fn resolution(&self, pts: Duration) -> Resolution;
}

pub(crate) struct LayoutNode {
    layout_provider: Box<dyn LayoutProvider>,
    shader: Arc<LayoutShader>,
    params: ParamsBuffer,
}

#[derive(Debug, Clone)]
pub struct Crop {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Debug, Clone)]
struct RenderLayout {
    top: f32,
    left: f32,
    width: f32,
    height: f32,
    rotation_degrees: f32,
    content: RenderLayoutContent,
}

#[derive(Debug, Clone)]
enum RenderLayoutContent {
    Color(RGBAColor),
    ChildNode { index: usize, crop: Crop },
}

#[derive(Debug, Clone)]
pub enum LayoutContent {
    Color(RGBAColor),
    ChildNode { index: usize, size: Size },
    None,
}

#[derive(Debug, Clone)]
pub struct NestedLayout {
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
    pub rotation_degrees: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    /// Crop is applied before scaling.
    pub crop: Option<Crop>,
    pub content: LayoutContent,

    pub(crate) children: Vec<NestedLayout>,
    /// Describes how many children of this component are nodes. This value also
    /// counts `layout` if its content is a `LayoutContent::ChildNode`.
    ///
    /// `child_nodes_count` is not necessarily equal to number of `LayoutContent::ChildNode` in
    /// a sub-tree. For example, if we have a component that conditionally shows one
    /// of its children then child_nodes_count will count all of those components even
    /// though only one of those children will be present in the layouts tree.
    pub(crate) child_nodes_count: usize,
}

impl LayoutNode {
    pub fn new(ctx: &RenderCtx, layout_provider: Box<dyn LayoutProvider>) -> Self {
        let shader = ctx.renderers.layout.0.clone();

        Self {
            layout_provider,
            shader,
            params: ParamsBuffer::new(ctx.wgpu_ctx, vec![]),
        }
    }

    pub fn render(
        &mut self,
        ctx: &RenderCtx,
        sources: &[(&NodeId, &NodeTexture)],
        target: &mut NodeTexture,
        pts: Duration,
    ) {
        let input_resolutions: Vec<Option<Resolution>> = sources
            .iter()
            .map(|(_, node_texture)| node_texture.resolution())
            .collect();
        let layouts = self
            .layout_provider
            .layouts(pts, &input_resolutions)
            .flatten(0);
        let layout_count = layouts.len();
        let output_resolution = self.layout_provider.resolution(pts);

        let params: Vec<LayoutNodeParams> = layouts
            .iter()
            .map(|layout| {
                let (texture_id, background_color, input_resolution) = match layout.content {
                    RenderLayoutContent::ChildNode { index, .. } => (
                        index as i32,
                        RGBAColor(0, 0, 0, 0),
                        *input_resolutions.get(index).unwrap_or(&None),
                    ),
                    RenderLayoutContent::Color(color) => (-1, color, None),
                };

                LayoutNodeParams {
                    texture_id,
                    background_color,
                    transform_vertices_matrix: layout
                        .vertices_transformation_matrix(&output_resolution),
                    transform_texture_coords_matrix: layout
                        .texture_coords_transformation_matrix(&input_resolution),
                }
            })
            .collect();
        self.params.update(params, ctx.wgpu_ctx);

        let target = target.ensure_size(ctx.wgpu_ctx, output_resolution);
        self.shader.render(
            ctx.wgpu_ctx,
            self.params.bind_group(),
            sources,
            target,
            layout_count as u32,
        );
    }
}
