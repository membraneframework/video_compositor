use std::{time::Duration, sync::Arc};

use compositor_common::scene::{InputId, NodeId, Resolution};

use self::{
    component::{FixedPositionLayout, Text},
    layout_node::Layout,
    shader_node::Shader, tiled_layout_component::TiledLayout,
};

mod build_scene;
mod component;
mod layout_node;
mod shader_node;
mod text_node;
mod tiled_layout_component;

pub(crate) struct SceneState {
    pub root: Component,
    pub last_render: Option<NodeParams>,
    pub pts_last_render: Option<Duration>,
    pub last_update: Option<NodeParams>,
}

pub struct Scene {
    pub output_id: NodeId,
    pub resolution: Resolution,
    pub root: Component,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum ComponentId {
    UserDefined(Arc<str>),
    Auto(u64),
    Input(InputId),
}

pub enum Component {
    Text(Text),
    TiledLayout(TiledLayout),
    FixedPositionLayout(FixedPositionLayout),
    InputStream(InputId),
}

impl Component {
    fn children(&self) -> &[Component] {
        todo!()
    }

    fn id(&self) -> NodeId {
        todo!()
    }
}

pub struct NodeParams {
    pub id: NodeId,
    pub inner: InnerNodeParams,
    pub children: Vec<NodeParams>,
}

pub enum InnerNodeParams {
    Text(Text),
    Shader(Shader),
    Layout(Layout),
    InputStream(InputId),
}
