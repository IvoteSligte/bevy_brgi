use std::mem::size_of;
use std::num::NonZeroU64;

use bevy::prelude::*;
use bevy::render::extract_component::{
    ComponentUniforms, ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::render_resource::binding_types::{storage_buffer, uniform_buffer_sized};
use bevy::render::render_resource::{
    BindGroup, BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, Buffer, ShaderStages,
    ShaderType,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::{Render, RenderApp, RenderSet};
use bevy::{
    app::{App, Plugin},
    asset::{load_internal_asset, Handle},
    render::render_resource::Shader,
};
use bytemuck::{Pod, Zeroable};

use crate::{init_device_storage_buffer, BrgiCamera, DEFAULT_PROBE_COUNT};

const UTILS_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(6162463479261672493);

pub struct ProbePlugin;

impl Plugin for ProbePlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            UTILS_SHADER_HANDLE,
            "../assets/shaders/utils.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins((
            ExtractComponentPlugin::<ProbeUniform>::default(),
            ExtractComponentPlugin::<ProbeBuffers>::default(),
            UniformComponentPlugin::<ProbeUniform>::default(),
        ))
        .add_systems(Update, add_probe_buffers);

        app.sub_app_mut(RenderApp)
            .add_systems(Render, init_bind_group.in_set(RenderSet::PrepareBindGroups));
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<ProbeBindGroupLayout>();
    }
}

#[derive(Clone, Component, ExtractComponent, ShaderType)]
pub struct ProbeUniform {
    pub probe_count: u32, // should be clamped every frame
    pub max_probe_count: u32,
}

impl ProbeUniform {
    pub fn new(max_probe_count: u32) -> Self {
        Self {
            probe_count: 0,
            max_probe_count,
        }
    }
}

impl Default for ProbeUniform {
    fn default() -> Self {
        Self {
            probe_count: 0,
            max_probe_count: DEFAULT_PROBE_COUNT,
        }
    }
}

/// shaders do not create new probes if the max probe count has been reached
/// but the [ProbeUniform.probe_count] value should still be limited to prevent
/// integer overflow
fn limit_probe_count(mut query: Query<&mut ProbeUniform>) {
    for mut probe_uniform in query.iter_mut() {
        probe_uniform.probe_count = probe_uniform.probe_count.min(probe_uniform.max_probe_count);
    }
}

#[repr(C)]
#[derive(Clone, Copy, ShaderType, Default, Zeroable, Pod)]
pub struct Probe {
    position: Vec3,
    normal: u32,
}

pub type ProbeColor = Vec4;

pub type ProbeGData = UVec4;

#[derive(Component, ExtractComponent, Clone)]
pub struct ProbeBuffers {
    primary: Buffer,
    color: Buffer,
    /// g buffer
    geometry: Buffer,
}

impl ProbeBuffers {
    pub fn new(render_device: &RenderDevice, num_probes: usize) -> Self {
        Self {
            primary: init_device_storage_buffer(render_device, &vec![Probe::default(); num_probes]),
            color: init_device_storage_buffer(
                render_device,
                &vec![ProbeColor::default(); num_probes],
            ),
            geometry: init_device_storage_buffer(
                render_device,
                &vec![ProbeGData::default(); num_probes],
            ),
        }
    }
}

pub fn add_probe_buffers(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    query: Query<(Entity, &ProbeUniform), (With<BrgiCamera>, Without<ProbeBuffers>)>,
) {
    for (entity, params) in query.iter() {
        let Some(mut entity_commands) = commands.get_entity(entity) else {
            continue;
        };

        entity_commands.insert(ProbeBuffers::new(
            &render_device,
            params.max_probe_count as usize,
        ));
    }
}

#[derive(Resource, Deref)]
pub struct ProbeBindGroupLayout(pub BindGroupLayout);

impl FromWorld for ProbeBindGroupLayout {
    fn from_world(world: &mut World) -> Self {
        let entries = BindGroupLayoutEntries::sequential(
            ShaderStages::COMPUTE | ShaderStages::FRAGMENT,
            (
                // TODO: determine if params should have a dynamic offset or not
                uniform_buffer_sized(false, NonZeroU64::new(size_of::<ProbeUniform>() as u64)),
                storage_buffer::<Vec<Probe>>(false),
                storage_buffer::<Vec<ProbeColor>>(false),
                storage_buffer::<Vec<ProbeGData>>(false),
            ),
        );

        let layout = world
            .resource::<RenderDevice>()
            .create_bind_group_layout(Some("common_cache_bind_group_layout"), &entries);

        Self(layout)
    }
}

#[derive(Component)]
pub struct ProbeBindGroup(pub BindGroup);

fn init_bind_group(
    mut commands: Commands,
    render_device: Res<RenderDevice>,
    param_uniforms: Res<ComponentUniforms<ProbeUniform>>,
    bind_group_layout: Res<ProbeBindGroupLayout>,
    mut query: Query<(Entity, &ProbeBuffers), Without<ProbeBindGroup>>,
) {
    let Some(param_binding) = param_uniforms.uniforms().binding() else {
        return;
    };

    for (entity, probe_buffers) in query.iter_mut() {
        let bind_group = render_device.create_bind_group(
            Some("common_cache_bind_group"),
            &bind_group_layout,
            &BindGroupEntries::sequential((
                param_binding.clone(),
                probe_buffers.primary.as_entire_binding(),
                probe_buffers.color.as_entire_binding(),
                probe_buffers.geometry.as_entire_binding(),
            )),
        );

        commands
            .get_entity(entity)
            .unwrap()
            .insert(ProbeBindGroup(bind_group));
    }
}
