use crate::{BrgiMarker, Params, WORKGROUP_SIZE};

use bevy::asset::load_internal_asset;
use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::NodeRunError;
use bevy::render::render_graph::{RenderGraphContext, ViewNode};
use bevy::render::render_resource::{
    AsBindGroup, BindGroup, CachedComputePipelineId, ComputePipelineDescriptor, PipelineCache,
};
use bevy::render::renderer::{RenderContext, RenderDevice};
use bevy::render::texture::FallbackImage;
use bevy::render::{Render, RenderApp, RenderSet};

const MATCHING_PROBES_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(7916842541810802449);
const SPAWN_INTERSECT_PROBES_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(12748321941431287895);
const PREFIX_SUM_DISTRIBUTE_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(8600298598939286931);
const UTILS_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(6162463479261672493);
const PREFIX_SUM_REDUCE_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(11327118442002064687);
const COUNT_PROBES_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(10070001324747794273);
const CHECK_INTERSECT_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(11037825407511410396);
const PROBE_DISTANCE_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(15541187797796128866);
const ACCUMULATE_PROBE_COLOR_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(7052893674961671838);

pub struct WorldCachePlugin;

impl Plugin for WorldCachePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            MATCHING_PROBES_SHADER_HANDLE,
            "../assets/shaders/matching_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            SPAWN_INTERSECT_PROBES_SHADER_HANDLE,
            "../assets/shaders/spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PREFIX_SUM_DISTRIBUTE_SHADER_HANDLE,
            "../assets/shaders/prefix_sum_distribute.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            UTILS_SHADER_HANDLE,
            "../assets/shaders/utils.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PREFIX_SUM_REDUCE_SHADER_HANDLE,
            "../assets/shaders/prefix_sum_reduce.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            COUNT_PROBES_SHADER_HANDLE,
            "../assets/shaders/count_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            CHECK_INTERSECT_SHADER_HANDLE,
            "../assets/shaders/check_intersect.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PROBE_DISTANCE_SHADER_HANDLE,
            "../assets/shaders/probe_distance.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            ACCUMULATE_PROBE_COLOR_SHADER_HANDLE,
            "../assets/shaders/accumulate_probe_color.wgsl",
            Shader::from_wgsl
        );
        app.sub_app_mut(RenderApp)
            .add_systems(Render, init_bind_group.in_set(RenderSet::PrepareBindGroups));
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<WorldCachePipelines>();
    }
}

#[derive(Resource)]
pub struct WorldCachePipelines {
    accumulate_probe_color: CachedComputePipelineId,
}

impl FromWorld for WorldCachePipelines {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = WorldCache::bind_group_layout(&render_device);
        let cache = world.resource_mut::<PipelineCache>();

        let basic_pipeline = |shader: Handle<Shader>| {
            cache.queue_compute_pipeline(ComputePipelineDescriptor {
                label: None,
                layout: vec![layout.clone()],
                push_constant_ranges: vec![],
                shader,
                shader_defs: vec![],
                entry_point: "main".into(),
            })
        };

        Self {
            accumulate_probe_color: basic_pipeline(ACCUMULATE_PROBE_COLOR_SHADER_HANDLE),
            // TODO: remaining shaders
        }
    }
}

#[derive(Component, AsBindGroup)]
pub struct WorldCache {
    // TODO: verify that the bind group layout here matches up with the shader bg layout
    #[storage(0)]
    outer_offset_buffer: Vec<u32>,
    #[storage(1)]
    distance_buffer: Vec<f32>,
    #[storage(2)]
    transient_probe_index_buffer: Vec<u32>,
    #[storage(3)]
    match_distance_buffer: Vec<f32>,
    #[storage(4)]
    match_index_buffer: Vec<u32>,
    #[storage(5)]
    occlusion_bit_buffer: Vec<u32>,
}

impl WorldCache {
    fn new(num_probes: usize, image_len: usize) -> Self {
        Self {
            outer_offset_buffer: vec![0; image_len],
            distance_buffer: vec![0.0; num_probes],
            transient_probe_index_buffer: vec![0; num_probes],
            match_distance_buffer: vec![0.0; num_probes],
            match_index_buffer: vec![0; num_probes],
            occlusion_bit_buffer: vec![0; num_probes / 32],
        }
    }

    fn clear(&mut self) {
        self.outer_offset_buffer.clear();
        self.distance_buffer.clear();
        self.transient_probe_index_buffer.clear();
        self.match_distance_buffer.clear();
        self.match_index_buffer.clear();
        self.occlusion_bit_buffer.clear();
    }
}

#[derive(Component)]
pub struct WorldCacheBindGroup(BindGroup);

fn init_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    fallback_image: Res<FallbackImage>,
    mut query: Query<(Entity, &mut WorldCache), Without<WorldCacheBindGroup>>,
) {
    for (entity, mut world_cache) in query.iter_mut() {
        let Ok(prepared_bind_group) = world_cache.as_bind_group(
            &WorldCache::bind_group_layout(&render_device),
            &render_device,
            &RenderAssets::default(),
            &fallback_image,
        ) else {
            continue; // TODO: error handling
        };

        world_cache.clear();

        commands
            .get_entity(entity)
            .unwrap()
            .insert(WorldCacheBindGroup(prepared_bind_group.bind_group));
    }
}

#[derive(Default)]
pub struct WorldCacheNode;

impl ViewNode for WorldCacheNode {
    type ViewQuery = (&'static BrgiMarker, &'static Params, &'static WorldCache);

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (_, params, world_cache): QueryItem<Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let render_device = world.resource::<RenderDevice>();
        let Ok(bind_group) = world_cache.as_bind_group(
            &WorldCache::bind_group_layout(render_device),
            render_device,
            &RenderAssets::default(),
            world.resource::<FallbackImage>(),
        ) else {
            return Ok(()); // TODO: error handling
        };

        let pipeline_ids = world.resource::<WorldCachePipelines>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) =
            pipeline_cache.get_compute_pipeline(pipeline_ids.accumulate_probe_color)
        else {
            return Ok(());
        };

        let mut pass = render_context
            .command_encoder()
            .begin_compute_pass(&default());
        pass.set_bind_group(0, &bind_group.bind_group, &[]);
        pass.set_pipeline(pipeline);
        pass.dispatch_workgroups(params.probe_count.div_ceil(WORKGROUP_SIZE), 1, 1);

        // TODO: the rest of the shaders

        Ok(())
    }
}
