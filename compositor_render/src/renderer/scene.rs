use std::{collections::HashMap, sync::Arc, time::Duration};

use compositor_common::{
    scene::{InputId, NodeId, NodeParams, NodeSpec, OutputId, Resolution, SceneSpec},
    SpecValidationError,
};
use log::error;

use crate::{
    render_loop::NodeRenderPass,
    transformations::{
        builtin::{node::BuiltinNode, Builtin},
        image_renderer::ImageNode,
        shader::node::ShaderNode,
        text_renderer::TextRendererNode,
        web_renderer::WebRenderer,
    },
};

use super::{
    node::CreateNodeError,
    texture::{InputTexture, NodeTexture, OutputTexture},
    RenderCtx, WgpuError, WgpuErrorScope,
};

pub enum RenderNode {
    Shader(ShaderNode),
    Web { renderer: Arc<WebRenderer> },
    Text(TextRendererNode),
    Image(ImageNode),
    Builtin(BuiltinNode),
    InputStream,
}

impl RenderNode {
    fn new(ctx: &RenderCtx, spec: &NodeSpec) -> Result<Self, CreateNodeError> {
        match &spec.params {
            NodeParams::WebRenderer { instance_id } => {
                let renderer = ctx
                    .renderers
                    .web_renderers
                    .get(instance_id)
                    .ok_or_else(|| CreateNodeError::WebRendererNotFound(instance_id.clone()))?;
                Ok(Self::Web { renderer })
            }
            NodeParams::Shader {
                shader_id,
                shader_params,
                resolution,
            } => {
                let shader = ctx
                    .renderers
                    .shaders
                    .get(shader_id)
                    .ok_or_else(|| CreateNodeError::ShaderNotFound(shader_id.clone()))?;
                let node = ShaderNode::new(
                    ctx.wgpu_ctx,
                    shader,
                    shader_params.as_ref(),
                    None,
                    *resolution,
                );
                Ok(Self::Shader(node))
            }
            NodeParams::Builtin { transformation } => {
                let shader = ctx.renderers.builtin.shader(transformation);
                let input_count = spec.input_pads.len() as u32;

                Ok(Self::Builtin(BuiltinNode::new(
                    shader,
                    Builtin(transformation.clone()),
                    input_count,
                )))
            }
            NodeParams::TextRenderer {
                text_params,
                resolution,
            } => {
                let renderer = TextRendererNode::new(ctx, text_params.clone(), resolution.clone());
                Ok(Self::Text(renderer))
            }
            NodeParams::Image { image_id } => {
                let image = ctx
                    .renderers
                    .images
                    .get(image_id)
                    .ok_or_else(|| CreateNodeError::ImageNotFound(image_id.clone()))?;
                let node = ImageNode::new(image);
                Ok(Self::Image(node))
            }
        }
    }

    pub fn render(
        &mut self,
        ctx: &mut RenderCtx,
        sources: &[(&NodeId, &NodeTexture)],
        target: &mut NodeTexture,
        pts: Duration,
    ) {
        match self {
            RenderNode::Shader(ref shader) => {
                shader.render(sources, target, pts);
            }
            RenderNode::Builtin(builtin_node) => builtin_node.render(sources, target, pts),
            RenderNode::Web { ref renderer } => renderer.render(ctx, sources, target),
            RenderNode::Text(ref renderer) => {
                renderer.render(ctx, target);
            }
            RenderNode::Image(ref node) => node.render(ctx, target, pts),
            RenderNode::InputStream => {
                // Nothing to do, textures on input nodes should be populated
                // at the start of render loop
            }
        }
    }

    pub fn resolution(&self) -> Option<Resolution> {
        match self {
            RenderNode::Shader(node) => Some(node.resolution()),
            RenderNode::Web { renderer } => Some(renderer.resolution()),
            RenderNode::Text(node) => Some(node.resolution()),
            RenderNode::Image(node) => Some(node.resolution()),
            RenderNode::InputStream => None,
            RenderNode::Builtin(node) => node.resolution_from_spec(),
        }
    }
}

pub struct Node {
    pub node_id: NodeId,
    pub output: NodeTexture,
    pub inputs: Vec<NodeId>,
    pub fallback: Option<NodeId>,
    pub renderer: RenderNode,
}

impl Node {
    pub fn new(ctx: &RenderCtx, spec: &NodeSpec) -> Result<Self, CreateNodeError> {
        let node = RenderNode::new(ctx, spec)?;
        let mut output = NodeTexture::new();
        if let Some(resolution) = node.resolution() {
            output.ensure_size(ctx.wgpu_ctx, resolution);
        }
        Ok(Self {
            node_id: spec.node_id.clone(),
            renderer: node,
            inputs: spec.input_pads.clone(),
            fallback: spec.fallback_id.clone(),
            output,
        })
    }

    pub fn new_input(node_id: &NodeId) -> Self {
        let output = NodeTexture::new();

        Self {
            node_id: node_id.clone(),
            renderer: RenderNode::InputStream,
            inputs: vec![],
            fallback: None,
            output,
        }
    }
}

pub struct Scene {
    pub nodes: SceneNodesSet,
    pub outputs: HashMap<OutputId, (NodeId, OutputTexture)>,
    pub inputs: HashMap<InputId, InputTexture>,
}

#[derive(Debug, thiserror::Error)]
pub enum UpdateSceneError {
    #[error("Failed to create node \"{1}\". {0}")]
    CreateNodeError(#[source] CreateNodeError, NodeId),

    #[error("Invalid scene: {0}")]
    InvalidSpec(#[from] SpecValidationError),

    #[error("Unknown node \"{0}\" used in scene.")]
    NoNodeWithIdError(NodeId),

    #[error(transparent)]
    WgpuError(#[from] WgpuError),

    #[error("Unknown resolution on the output node. Nodes that are declared as outputs need to have constant resolution that is the same as resolution of the output stream.")]
    UnknownResolutionOnOutput(NodeId),
}

impl Scene {
    pub fn empty() -> Self {
        Self {
            nodes: SceneNodesSet::new(),
            outputs: HashMap::new(),
            inputs: HashMap::new(),
        }
    }

    pub fn update(&mut self, ctx: &RenderCtx, spec: &SceneSpec) -> Result<(), UpdateSceneError> {
        // TODO: If we want nodes to be stateful we could try reusing nodes instead
        //       of recreating them on every scene update
        let scope = WgpuErrorScope::push(&ctx.wgpu_ctx.device);

        let mut new_nodes = HashMap::new();
        let mut inputs = HashMap::new();
        let outputs = spec
            .outputs
            .iter()
            .map(|output| {
                Self::ensure_node(ctx, &output.input_pad, spec, &mut inputs, &mut new_nodes)?;
                let node = new_nodes
                    .get(&output.input_pad)
                    .ok_or_else(|| UpdateSceneError::NoNodeWithIdError(output.input_pad.clone()))?;
                let resolution = node.renderer.resolution().ok_or_else(|| {
                    UpdateSceneError::UnknownResolutionOnOutput(node.node_id.clone())
                })?;
                let output_texture = OutputTexture::new(ctx.wgpu_ctx, resolution);
                Ok((
                    output.output_id.clone(),
                    (node.node_id.clone(), output_texture),
                ))
            })
            .collect::<Result<_, UpdateSceneError>>()?;

        scope.pop(&ctx.wgpu_ctx.device)?;

        self.inputs = inputs;
        self.outputs = outputs;
        self.nodes = SceneNodesSet { nodes: new_nodes };

        Ok(())
    }

    fn ensure_node(
        ctx: &RenderCtx,
        node_id: &NodeId,
        spec: &SceneSpec,
        inputs: &mut HashMap<InputId, InputTexture>,
        new_nodes: &mut HashMap<NodeId, Node>,
    ) -> Result<(), UpdateSceneError> {
        // check if node already exists
        if new_nodes.get(node_id).is_some() {
            return Ok(());
        }

        // handle a case where node_id refers to transform node
        {
            let node_spec = spec.nodes.iter().find(|node| &node.node_id == node_id);
            if let Some(node_spec) = node_spec {
                for child_id in &node_spec.input_pads {
                    Self::ensure_node(ctx, child_id, spec, inputs, new_nodes)?;
                }
                if let Some(fallback_id) = &node_spec.fallback_id {
                    Self::ensure_node(ctx, fallback_id, spec, inputs, new_nodes)?;
                }
                let node = Node::new(ctx, node_spec)
                    .map_err(|err| UpdateSceneError::CreateNodeError(err, node_id.clone()))?;
                new_nodes.insert(node_id.clone(), node);
                return Ok(());
            }
        }

        // If there is no node with id node_id, assume it's an input. Pipeline validation should
        // make sure that scene does not refer to missing entities.
        let node = Node::new_input(node_id);
        new_nodes.insert(node_id.clone(), node);
        inputs.insert(InputId(node_id.clone()), InputTexture::new());
        Ok(())
    }
}

#[derive(Default)]
pub struct SceneNodesSet {
    nodes: HashMap<NodeId, Node>,
}

impl SceneNodesSet {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn node(&self, node_id: &NodeId) -> Result<&Node, InternalSceneError> {
        self.nodes
            .get(node_id)
            .ok_or_else(|| InternalSceneError::MissingNode(node_id.clone()))
    }

    pub fn node_mut(&mut self, node_id: &NodeId) -> Result<&mut Node, InternalSceneError> {
        self.nodes
            .get_mut(node_id)
            .ok_or_else(|| InternalSceneError::MissingNode(node_id.clone()))
    }

    pub fn node_or_fallback<'a>(
        &'a self,
        node_id: &NodeId,
    ) -> Result<&'a Node, InternalSceneError> {
        let nodes: HashMap<&NodeId, &Node> = self.nodes.iter().collect();
        Self::find_fallback_node(&nodes, node_id)
    }

    /// Borrow all nodes that are needed to render node node_id.
    pub(crate) fn node_render_pass<'a>(
        &'a mut self,
        node_id: &NodeId,
    ) -> Result<NodeRenderPass<'a>, InternalSceneError> {
        let input_ids: Vec<NodeId> = self.node(node_id)?.inputs.to_vec();

        // Borrow all the references, Fallback technically can be applied on every
        // level, so the easiest approach is to just borrow everything
        let mut nodes_mut: HashMap<&NodeId, &mut Node> = self.nodes.iter_mut().collect();

        // Extract mutable borrow for the node we will render.
        let node = nodes_mut
            .remove(&node_id)
            .ok_or_else(|| InternalSceneError::MissingNode(node_id.clone()))?;

        // Convert mutable borrows on rest of the nodes into immutable.
        // One input might be used multiple times, so we might need to
        // borrow it more than once, so it needs to be immutable.
        let nodes: HashMap<&NodeId, &Node> = nodes_mut
            .into_iter()
            .map(|(id, node)| (id, &*node))
            .collect();

        // Get immutable borrows for inputs. For each input if node texture
        // is empty go through the fallback chain
        let inputs: Vec<(NodeId, &Node)> = input_ids
            .into_iter()
            .map(|input_id| {
                let node = Self::find_fallback_node(&nodes, &input_id)?;
                // input_id and node.node_id are different if fallback is triggered
                Ok((input_id, node))
            })
            .collect::<Result<Vec<_>, InternalSceneError>>()?;
        Ok(NodeRenderPass { node, inputs })
    }

    fn find_fallback_node<'a>(
        nodes: &HashMap<&NodeId, &'a Node>,
        node_id: &NodeId,
    ) -> Result<&'a Node, InternalSceneError> {
        let mut node: &Node = nodes
            .get(node_id)
            .ok_or_else(|| InternalSceneError::MissingNode(node_id.clone()))?;
        while node.output.is_empty() && node.fallback.is_some() {
            let fallback_id = node.fallback.clone().unwrap();
            node = nodes
                .get(&fallback_id)
                .ok_or_else(|| InternalSceneError::MissingNode(fallback_id.clone()))?
        }
        Ok(node)
    }
}

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum InternalSceneError {
    #[error("Missing node \"{0}\"")]
    MissingNode(NodeId),
}
