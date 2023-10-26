use crate::{error::UpdateSceneError, renderer::RenderCtx};

use super::{Component, InnerNodeParams, NodeParams, Scene, SceneState};

impl SceneState {
    pub fn update(&mut self, ctx: &RenderCtx, scenes: Vec<Scene>) -> Result<(), UpdateSceneError> {
        for scene in scenes {
            let node_tree = Self::build_tree(&scene.root);
        }
        Ok(())
    }

    pub fn build_tree(component: &Component) -> NodeParams {
        match component {
            Component::Text(text) => NodeParams {
                id: component.id(),
                inner: InnerNodeParams::Text(text.clone()),
                children: vec![],
            },
            Component::TiledLayout(tiled_layout) => {
                let children: Vec<NodeParams> =
                    tiled_layout.children.iter().map(Self::build_tree).collect();
                let (layout, children) = tiled_layout.layout(children);
                NodeParams {
                    id: component.id(),
                    inner: InnerNodeParams::Layout(layout),
                    children,
                }
            }
            Component::FixedPositionLayout(fixed_position_layout) => {
                let children: Vec<NodeParams> = fixed_position_layout
                    .children
                    .iter()
                    .map(Self::build_tree)
                    .collect();
                let (layout, children) = fixed_position_layout.layout(&children);
                NodeParams {
                    id: component.id(),
                    inner: InnerNodeParams::Layout(layout),
                    children,
                }
            }
            Component::InputStream(input_id) => NodeParams {
                id: component.id(),
                inner: InnerNodeParams::InputStream(input_id.clone()),
                children: vec![],
            },
        }
    }
}
