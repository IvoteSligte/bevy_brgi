use bevy::asset::load_internal_asset;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::extract_component::ComponentUniforms;
use bevy::render::render_graph::NodeRunError;
use bevy::render::render_graph::{RenderGraphContext, ViewNode};
use bevy::render::render_resource::binding_types::{
    storage_buffer, storage_buffer_read_only, uniform_buffer,
};
use bevy::render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedComputePipelineId,
    ComputePipelineDescriptor, PipelineCache, ShaderStages,
};
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::{Render, RenderApp};

use crate::{BrgiMarker, CountBuffer, Params, Probe, ProbeBuffer};

pub const HANDLE: Handle<Shader> = Handle::weak_from_u128(6369956254542331318);
pub const WORKGROUP_SIZE: u32 = 64;

struct CountProbesPlugin;

impl Plugin for CountProbesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Render, prepare_bind_group);

        load_internal_asset!(
            app,
            HANDLE,
            "../shaders/count_probes.wgsl",
            Shader::from_wgsl
        );
    }

    fn finish(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        render_app.init_resource::<CountProbesPipeline>();
    }
}

#[derive(Component)]
pub struct CountProbesBindGroup(BindGroup);

fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<CountProbesPipeline>,
    probes: Res<ProbeBuffer>,
    params: Res<ComponentUniforms<Params>>,
    query: Query<(Entity, &BrgiMarker, &CountBuffer)>,
    render_device: Res<RenderDevice>,
) {
    let Some(param_binding) = params.binding() else {
        warn!("Params binding could not be retreived.");
        return;
    };
    let Some(probe_binding) = probes.0.binding() else {
        warn!("Probe buffer binding could not be retreived.");
        return;
    };

    for (entity, _, count_buffer) in query.iter() {
        let Some(count_binding) = count_buffer.0.binding() else {
            warn!("Count buffer binding could not be retreived.");
            continue;
        };

        let bind_group = render_device.create_bind_group(
            "count_probes_bind_group",
            &pipeline.layout,
            &BindGroupEntries::sequential((
                param_binding.clone(),
                probe_binding.clone(),
                count_binding,
            )),
        );
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            warn!("Could not retreive entity.");
            continue;
        };
        entity_commands.insert(CountProbesBindGroup(bind_group));
    }
}

#[derive(Default)]
pub struct CountProbesNode;

impl ViewNode for CountProbesNode {
    type ViewQuery = (
        &'static BrgiMarker,
        &'static Params,
        &'static CountProbesBindGroup,
    );

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (_, params, bind_group): QueryItem<Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let count_probes_pipeline = world.resource::<CountProbesPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_compute_pipeline(count_probes_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&default());
        pass.set_bind_group(0, &bind_group.0, &[]);
        pass.set_pipeline(pipeline);
        pass.dispatch_workgroups(
            (params.probe_count + WORKGROUP_SIZE - 1) / WORKGROUP_SIZE,
            1,
            1,
        );

        Ok(())
    }
}

#[derive(Resource)]
pub struct CountProbesPipeline {
    layout: BindGroupLayout,
    pipeline_id: CachedComputePipelineId,
}

impl FromWorld for CountProbesPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();

        let layout = render_device.create_bind_group_layout(
            "count_probes_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::COMPUTE,
                (
                    uniform_buffer::<Params>(false),
                    storage_buffer_read_only::<Vec<Probe>>(false),
                    storage_buffer::<Vec<u32>>(false),
                ),
            ),
        );

        let pipeline_id = world
            .resource_mut::<PipelineCache>()
            .queue_compute_pipeline(ComputePipelineDescriptor {
                label: Some("count_probes_pipeline".into()),
                layout: vec![layout.clone()],
                push_constant_ranges: vec![],
                shader: HANDLE,
                shader_defs: vec![],
                entry_point: "main".into(),
            });

        Self {
            layout,
            pipeline_id,
        }
    }
}
