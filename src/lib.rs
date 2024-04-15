use bevy::prelude::*;
use bevy::render::extract_component::{
    ExtractComponent, ExtractComponentPlugin, UniformComponentPlugin,
};
use bevy::render::render_graph::{RenderGraphApp, ViewNodeRunner};
use bevy::render::RenderApp;

use common_cache::{CommonCache, CommonCachePlugin, Params};
use deferred::DeferredPbrLightingPlugin;
use screen_cache::ScreenCache;
use world_cache::WorldCache;

pub mod common_cache;
pub mod deferred;
pub mod screen_cache;
pub mod world_cache;

use crate::world_cache::WorldCacheNode;

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
            CommonCachePlugin,
            // WorldCachePlugin,
            // ScreenCachePlugin,
            ExtractComponentPlugin::<Params>::default(),
            ExtractComponentPlugin::<BrgiCamera>::default(),
            UniformComponentPlugin::<Params>::default(),
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

#[derive(Component, Clone, Default)]
pub struct BrgiCamera; // TODO: properties such as number of probes and brgi ortho image resolution

impl ExtractComponent for BrgiCamera {
    type QueryData = ();
    type QueryFilter = (
        Without<CommonCache>,
        Without<ScreenCache>,
        Without<WorldCache>,
    );
    type Out = BrgiRenderBundle;

    fn extract_component(
        _item: bevy::ecs::query::QueryItem<'_, Self::QueryData>,
    ) -> Option<Self::Out> {
        // FIXME: called multiple times for every time the deferred node run function is called?
        Some(BrgiRenderBundle::default()) // TODO: non-default values
    }
}

/// bundle of brgi renderworld-only components
#[derive(Bundle, Default)]
pub struct BrgiRenderBundle {
    camera: BrgiCamera,
    common_cache: CommonCache,
    // screen_cache: ScreenCache,
    // world_cache: WorldCache,
}
