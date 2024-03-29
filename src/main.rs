use std::num::NonZeroU64;
use std::sync::Arc;

use bevy::prelude::*;
use bevy::render::extract_component::{
    ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::render_graph::{RenderGraphApp, ViewNodeRunner};
use bevy::render::render_resource::{Buffer, ShaderType, StorageBuffer};
use bevy::render::RenderApp;

pub mod node;

use node::count_probes::*;

pub const MAX_PROBE_COUNT: u32 = 32 << 15; // must be a multiple of 32 for prefix sum

pub mod graph {
    use bevy::render::render_graph::{RenderLabel, RenderSubGraph};

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderSubGraph)]
    pub struct Brgi;

    #[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
    pub enum NodeBrgi {
        CountProbes,
        PrefixSum,
        ProbeDistance,
        CheckIntersect,
        SpawnIntersectProbes,
        PersRender,
        PersRenderProbes,
        PersSpawnIntersectProbes,
        AccumulateProbeColor,
    }
}

struct BrgiPlugin;

impl Plugin for BrgiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<Params>::default(),
            UniformComponentPlugin::<Params>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.add_render_sub_graph(graph::Brgi);

        render_app.add_render_graph_node::<ViewNodeRunner<CountProbesNode>>(
            graph::Brgi,
            graph::NodeBrgi::CountProbes,
        );
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

#[repr(C)]
#[derive(Clone, Copy, ShaderType)]
pub struct Probe {
    position: Vec3,
    normal_material: u32,
}

pub struct AtomicBufferBinding {
    pub buffer: Arc<Buffer>,
    pub offset: u64,
    pub size: Option<NonZeroU64>,
}

#[derive(Component)]
pub struct CountBuffer(StorageBuffer<Vec<u32>>);

#[derive(Resource)]
pub struct ProbeBuffer(StorageBuffer<Vec<Probe>>);

#[derive(Component)]
pub struct BrgiMarker;

#[derive(Bundle)]
pub struct BrgiBundle {
    param: Params,
    counts: CountBuffer,
}

fn main() {
    App::new().add_plugins((DefaultPlugins, BrgiPlugin)).run();
}
