use compositor_common::{scene::Resolution, util::colors::RGBAColor};

use super::{InnerNodeParams, NodeParams};

pub struct Layout {
    pub elements: Vec<LayoutView>,
    pub resolution: Resolution,
}

pub struct LayoutView {
    pub background_color: RGBAColor,
    pub top: f32,
    pub left: f32,
    pub width: f32,
    pub height: f32,
    pub rotation: f32,
    pub child_index: Option<usize>,
}

impl LayoutView {
    pub fn fit_into(&self, child_layout: &Layout) -> Vec<LayoutView> {
        let input_x_scale = self.width / child_layout.resolution.width as f32;
        let input_y_scale = self.height / child_layout.resolution.height as f32;

        let input_scale = f32::min(input_x_scale, input_y_scale);

        let x_padding = self.width - child_layout.resolution.width as f32 * input_scale;
        let y_padding = self.height - child_layout.resolution.height as f32 * input_scale;

        let left_padding = x_padding / 2.0;
        let top_padding = y_padding / 2.0;
        // TODO: add back alignment methods (maybe)

        child_layout
            .elements
            .iter()
            .map(|view| LayoutView {
                background_color: view.background_color,
                top: (view.top * input_scale) + self.top + top_padding,
                left: (view.left * input_scale) + self.left + left_padding,
                width: view.width * input_scale,
                height: view.height * input_scale,
                rotation: view.rotation, // TODO: this will be a bit broken for now
                child_index: view.child_index,
            })
            .collect()
    }
}

impl Layout {
    pub fn embed_children(
        &mut self,
        views: Vec<LayoutView>,
        children: Vec<NodeParams>,
    ) -> Vec<NodeParams> {
        let mut combined_children = vec![];
        for (mut view, mut child) in views.into_iter().zip(children.into_iter()) {
            match child.inner {
                InnerNodeParams::Layout(layout) => {
                    let mut children_views = view.fit_into(&layout);

                    // child index needs to be recalculated when flattening
                    let index_offset = combined_children.len();
                    children_views.iter_mut().for_each(|view| {
                        view.child_index = view.child_index.map(|index| index + index_offset)
                    });

                    self.elements.push(view);
                    self.elements.append(&mut children_views);

                    combined_children.append(&mut child.children);
                }
                _ => {
                    view.child_index = Some(combined_children.len());
                    self.elements.push(view);
                    combined_children.push(child)
                }
            }
        }
        combined_children
    }
}
