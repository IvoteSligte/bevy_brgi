use bevy::ecs::query::QueryItem;
use bevy::prelude::*;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{NodeRunError, RenderGraphContext, ViewNode};
use bevy::render::render_resource::{AsBindGroup, BindGroup, PipelineCache};
use bevy::render::renderer::RenderContext;
use bevy::render::texture::FallbackImage;
use bevy::render::Render;
use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::{
        render_resource::{CachedComputePipelineId, Shader},
        renderer::RenderDevice,
        RenderApp,
    },
};

use crate::{BrgiMarker, Params};

const PERS_SPAWN_INTERSECT_PROBES_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(11765195943180295704);
const PERS_RENDER_PROBES_SHADER_HANDLE: Handle<Shader> =
    Handle::weak_from_u128(4669503846081955822);
const PERS_RENDER_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(6203937213307372526);

pub struct ScreenCachePlugin;

impl Plugin for ScreenCachePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PERS_SPAWN_INTERSECT_PROBES_SHADER_HANDLE,
            "../assets/shaders/pers_spawn_intersect_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PERS_RENDER_PROBES_SHADER_HANDLE,
            "../assets/shaders/pers_render_probes.wgsl",
            Shader::from_wgsl
        );
        load_internal_asset!(
            app,
            PERS_RENDER_SHADER_HANDLE,
            "../assets/shaders/pers_render.wgsl",
            Shader::from_wgsl
        );

        app.sub_app_mut(RenderApp)
            .add_systems(Render, init_bind_group);
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<ScreenCachePipelines>();
    }
}

#[derive(Resource)]
pub struct ScreenCachePipelines {
    // TODO:
}

impl FromWorld for ScreenCachePipelines {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = ScreenCache::bind_group_layout(&render_device);
        let mut cache = world.resource_mut::<PipelineCache>();

        Self {
            // TODO: remaining shaders
        }
    }
}

#[derive(Component, AsBindGroup)]
pub struct ScreenCache {
    // TODO:
    #[storage(0)]
    outer_offset_buffer: Vec<u32>,
    #[storage(1)]
    inner_count_buffer: Vec<u32>,
    #[storage(2)]
    distance_buffer: Vec<f32>,
    #[storage(3)]
    transient_probe_index_buffer: Vec<u32>,
    #[storage(4)]
    match_distance_buffer: Vec<f32>,
    #[storage(5)]
    match_index_buffer: Vec<u32>,
    #[storage(6)]
    occlusion_bit_buffer: Vec<u32>,
}

#[derive(Component)]
pub struct ScreenCacheBindGroup(BindGroup);

fn init_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    fallback_image: Res<FallbackImage>,
    query: Query<(Entity, &ScreenCache), Without<ScreenCacheBindGroup>>,
) {
    for (entity, world_cache) in query.iter() {
        let Ok(prepared_bind_group) = world_cache.as_bind_group(
            &ScreenCache::bind_group_layout(&render_device),
            &render_device,
            &RenderAssets::default(),
            &fallback_image,
        ) else {
            continue; // TODO: error handling
        };

        commands
            .get_entity(entity)
            .unwrap()
            .insert(ScreenCacheBindGroup(prepared_bind_group.bind_group));
    }
}

#[derive(Default)]
pub struct ScreenCacheNode;

impl ViewNode for ScreenCacheNode {
    type ViewQuery = (&'static BrgiMarker, &'static Params, &'static ScreenCache);

    fn run<'w>(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext<'w>,
        (_, params, world_cache): QueryItem<Self::ViewQuery>,
        world: &'w World,
    ) -> Result<(), NodeRunError> {
        let render_device = world.resource::<RenderDevice>();
        let Ok(bind_group) = world_cache.as_bind_group(
            &ScreenCache::bind_group_layout(render_device),
            render_device,
            &RenderAssets::default(),
            world.resource::<FallbackImage>(),
        ) else {
            return Ok(()); // TODO: error handling
        };

        // TODO: the rest of the shaders

        Ok(())
    }
}
