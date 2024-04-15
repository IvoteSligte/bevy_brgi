use bevy::prelude::*;
use bevy::render::extract_component::ExtractComponent;
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_resource::{AsBindGroup, BindGroup, ShaderType};
use bevy::render::renderer::RenderDevice;
use bevy::render::texture::FallbackImage;
use bevy::render::{Render, RenderApp, RenderSet};
use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};

use crate::DEFAULT_PROBE_COUNT;

const UTILS_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(6162463479261672493);

pub struct CommonCachePlugin;

impl Plugin for CommonCachePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            UTILS_SHADER_HANDLE,
            "../assets/shaders/utils.wgsl",
            Shader::from_wgsl
        );
        app.sub_app_mut(RenderApp)
            .add_systems(Render, init_bind_group.in_set(RenderSet::PrepareBindGroups));
    }
}

#[derive(Clone, Component, ExtractComponent, ShaderType)]
pub struct Params {
    pub probe_count: u32, // clamped every frame
    pub max_probe_count: u32,
}

#[derive(Clone, Copy, ShaderType, Default)]
pub struct Probe {
    position: Vec3,
    normal: u32,
}

pub type ProbeColor = Vec4;

pub type ProbeGData = UVec4;

// TODO: spawn in renderapp
#[derive(Component, AsBindGroup)]
pub struct CommonCache {
    #[uniform(0)]
    params: Params,
    #[storage(1)]
    probe_buffer: Vec<Probe>,
    #[storage(2)]
    probe_color_buffer: Vec<ProbeColor>,
    #[storage(3)]
    probe_g_buffer: Vec<ProbeGData>,
}

impl CommonCache {
    fn new(num_probes: usize) -> Self {
        Self {
            params: Params {
                probe_count: 0,
                max_probe_count: num_probes as u32,
            },
            probe_buffer: vec![Probe::default(); num_probes],
            probe_color_buffer: vec![ProbeColor::default(); num_probes],
            probe_g_buffer: vec![ProbeGData::default(); num_probes],
        }
    }

    fn clear(&mut self) {
        self.probe_buffer.clear();
        self.probe_color_buffer.clear();
        self.probe_g_buffer.clear();
    }
}

impl Default for CommonCache {
    fn default() -> Self {
        Self::new(DEFAULT_PROBE_COUNT as usize)
    }
}

#[derive(Component)]
pub struct CommonCacheBindGroup(pub BindGroup);

fn init_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    fallback_image: Res<FallbackImage>,
    mut query: Query<(Entity, &mut CommonCache), Without<CommonCacheBindGroup>>,
) {
    for (entity, mut common_cache) in query.iter_mut() {
        let Ok(prepared_bind_group) = common_cache.as_bind_group(
            &CommonCache::bind_group_layout(&render_device),
            &render_device,
            &RenderAssets::default(),
            &fallback_image,
        ) else {
            continue; // TODO: error handling
        };

        common_cache.clear();

        commands
            .get_entity(entity)
            .unwrap()
            .insert(CommonCacheBindGroup(prepared_bind_group.bind_group));
    }
}
