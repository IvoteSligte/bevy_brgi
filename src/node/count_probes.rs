use std::sync::Arc;

use bevy::prelude::*;
use bevy::render::render_graph::{RenderGraphContext, self};
use bevy::render::renderer::RenderContext;
use bevy::render::render_graph::NodeRunError;
use bevy::render::render_resource::{AsBindGroup, StorageBuffer, encase::UniformBuffer};
use bevy::render::extract_component::ExtractComponentPlugin;

use crate::shader::{Params, Probe};

pub struct CountProbesPlugin;

impl Plugin for CountProbesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(ExtractComponentPlugin::<Bindings>::default());
        //
        // if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
        //     render_app
        // }
    }
}

#[derive(Component, AsBindGroup)]
pub struct Bindings {
    #[uniform(0)]
    pub params: Arc<UniformBuffer<Params>>,
    #[storage(1)]
    pub probes: Arc<StorageBuffer<Probe>>,
    #[storage(2)]
    pub counts: Arc<StorageBuffer<u32>>,
}

pub struct Node;

impl render_graph::Node for Node {
    fn run<'w>(
            &self,
            graph: &mut RenderGraphContext,
            render_context: &mut RenderContext<'w>,
            world: &'w World,
        ) -> Result<(), NodeRunError> {
        Ok(())
    }
}

