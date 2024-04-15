use bevy::prelude::*;
use bevy::render::extract_component::{
    ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::render_graph::{RenderGraphApp, ViewNodeRunner};
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::render::RenderApp;

use deferred::DeferredPbrLightingPlugin;
use world_cache::WorldCache;

pub mod deferred;
pub mod screen_cache;
pub mod world_cache;

use crate::{
    screen_cache::ScreenCachePlugin,
    world_cache::{WorldCacheNode, WorldCachePlugin},
};

pub const DEFAULT_PROBE_COUNT: u32 = 32 << 15; // 1 million // must be a multiple of 32 for prefix sum
pub const DEFAULT_IMAGE_LEN: u32 = 128; // resolution
pub const WORKGROUP_SIZE: u32 = 64;
pub const MATERIAL_COUNT: u32 = 256;

pub mod graph {
    use bevy::render::render_graph::{RenderLabel, RenderSubGraph};

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
    pub struct Brgi;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
    pub enum NodeBrgi {
        World,
        Screen,
    }
}

pub struct BrgiPlugin;

impl Plugin for BrgiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DeferredPbrLightingPlugin,
            // WorldCachePlugin,
            // ScreenCachePlugin,
            ExtractComponentPlugin::<Params>::default(),
            UniformComponentPlugin::<Params>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.add_render_sub_graph(graph::Brgi);

        render_app.add_render_graph_node::<ViewNodeRunner<WorldCacheNode>>(
            graph::Brgi,
            graph::NodeBrgi::World,
        ); // TODO: screen cache node; connect both nodes
    }
}

#[derive(Clone, Component, ExtractComponent, ShaderType)]
pub struct Params {
    probe_count: u32, // clamped every frame
    max_probe_count: u32,
}

#[derive(Clone, ShaderType)]
pub struct Material {
    dif_rg: u32,
    dif_b_emis_r: u32,
    emis_gb: u32,
}

#[derive(Clone, Copy, ShaderType, Default)]
pub struct Probe {
    position: Vec3,
    normal_material: u32,
}

#[derive(Clone, Copy, ShaderType, Default)]
pub struct ProbeColorData {
    color_rg: u32,
    color_b_material: u32,
}

#[derive(Component)]
pub struct BrgiMarker;

#[derive(Component, AsBindGroup)]
pub struct CommonCache {
    #[uniform(0)]
    params: Params,
    #[storage(1)]
    probe_buffer: Vec<Probe>,
    #[storage(2)]
    probe_color_data_buffer: Vec<ProbeColorData>,
}

impl CommonCache {
    fn new(num_probes: usize) -> Self {
        Self {
            params: Params {
                probe_count: 0,
                max_probe_count: num_probes as u32,
            },
            probe_buffer: vec![Probe::default(); num_probes],
            probe_color_data_buffer: vec![ProbeColorData::default(); num_probes],
        }
    }
}

impl Default for CommonCache {
    fn default() -> Self {
        Self::new(DEFAULT_PROBE_COUNT as usize)
    }
}

#[derive(Bundle, Default)]
pub struct BrgiBundle {
    common: CommonCache,
    // world: WorldCache,
    // screen: ScreenCache,
}
