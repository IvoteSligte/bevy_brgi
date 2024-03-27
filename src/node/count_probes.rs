use bevy::prelude::*;
use bevy::render::render_graph::NodeRunError;
use bevy::render::render_graph::{ViewNode, ViewNodeRunner, RenderGraphContext};
use bevy::render::renderer::RenderContext;
use bevy::render::render_graph::SlotInfo;
use bevy::render::RenderApp;
use bevy::render::render_graph::RenderGraphApp;

use crate::graph;

pub struct CountProbesPlugin;

impl Plugin for CountProbesPlugin {
    fn build(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.add_render_graph_node::<ViewNodeRunner<CountProbesNode>>(graph::Brgi, graph::NodeBrgi::CountProbes);
    }

    fn finish(&self, app: &mut App) {
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.init_resource::<CountProbesPipeline>();        
    }
}

#[derive(Default)]
pub struct CountProbesNode;

impl ViewNode for CountProbesNode {
    fn run<'w>(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        todo!()
    }
    
    fn output(&self) -> Vec<SlotInfo> {
        todo!()
    }
}

#[derive(Resource, Default)]
pub struct CountProbesPipeline;



