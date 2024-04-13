use bevy::prelude::*;
use bevy::render::extract_component::{
    ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::render_graph::{RenderGraphApp, ViewNodeRunner};
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::render::RenderApp;
use world_cache::WorldCache;

pub mod screen_cache;
pub mod world_cache;

use crate::{
    screen_cache::ScreenCachePlugin,
    world_cache::{WorldCacheNode, WorldCachePlugin},
};

pub const MAX_PROBE_COUNT: u32 = 32 << 15; // must be a multiple of 32 for prefix sum
pub const WORKGROUP_SIZE: u32 = 64;

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

struct BrgiPlugin;

impl Plugin for BrgiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            WorldCachePlugin,
            ScreenCachePlugin,
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
    world_to_screen: Mat4,
    dimension: u32,
    probe_count: u32, // clamped every frame
    max_probe_count: u32,
    direction: Vec3,
    world_to_clip: Mat4,
    screen_to_world: Mat4,
    pers_world_to_clip: Mat4,
    pers_screen_to_world: Mat4,
}

#[derive(Clone, ShaderType)]
pub struct Material {
    dif_rg: u32,
    dif_b_emis_r: u32,
    emis_gb: u32,
}

#[derive(Clone, Copy, ShaderType)]
pub struct Probe {
    position: Vec3,
    normal_material: u32,
}

#[derive(Clone, Copy, ShaderType)]
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
    #[uniform(3)]
    material_uniform: Vec<Material>,
}

#[derive(Bundle)]
pub struct BrgiBundle {
    common: CommonCache,
    world: WorldCache,
    // screen: ScreenCache,
}

fn main() {
    App::new().add_plugins((DefaultPlugins, BrgiPlugin)).run();
}
