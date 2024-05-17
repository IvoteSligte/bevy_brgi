use bevy::prelude::*;
use bevy::render::extract_component::{ExtractComponent, ExtractComponentPlugin};
use bevy::render::render_graph::RenderGraphApp;
use bevy::render::render_resource::{Buffer, BufferInitDescriptor, BufferUsages};
use bevy::render::renderer::RenderDevice;
use bevy::render::RenderApp;

use bytemuck::Pod;
use deferred::DeferredPbrLightingPlugin;
use probe::{ProbePlugin, ProbeUniform};

pub mod deferred;
pub mod probe;
pub mod screen_cache;
pub mod world_cache;

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
            ProbePlugin,
            // WorldCachePlugin,
            // ScreenCachePlugin,
            DeferredPbrLightingPlugin,
            ExtractComponentPlugin::<BrgiCamera>::default(),
        ));

        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            error!("Could not fetch render app.");
            return;
        };

        render_app.add_render_sub_graph(graph::Brgi);

        // render_app.add_render_graph_node::<ViewNodeRunner<WorldCacheNode>>(
        //     graph::Brgi,
        //     graph::NodeBrgi::World,
        // ); // TODO: screen cache node; connect both nodes
    }
}

/// marker struct for a camera with the brgi bundle
/// should NOT be added manually, instead add [BrgiBundle]
#[derive(Component, Clone, Default, ExtractComponent)]
pub struct BrgiCamera; // TODO: properties such as brgi ortho image resolution

/// bundle of brgi renderworld-only components
#[derive(Bundle, Default)]
pub struct BrgiBundle {
    camera: BrgiCamera,
    probe_uniform: ProbeUniform,
}

fn init_device_buffer<T: Pod>(
    render_device: &RenderDevice,
    data: &[T],
    usage: BufferUsages,
) -> Buffer {
    render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: None,
        contents: bytemuck::cast_slice(data),
        usage,
    })
}

fn init_device_storage_buffer<T: Pod>(render_device: &RenderDevice, data: &[T]) -> Buffer {
    init_device_buffer(render_device, data, BufferUsages::STORAGE)
}
